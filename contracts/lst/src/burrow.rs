use crate::*;

pub(crate) type TokenId = AccountId;
pub type Balance = u128;
pub type Shares = U128;

pub mod u128_dec_format {
    use near_sdk::serde::de;
    use near_sdk::serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod u64_dec_format {
    use near_sdk::serde::de;
    use near_sdk::serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum FarmId {
    Supplied(TokenId),
    Borrowed(TokenId),
    NetTvl,
    TokenNetBalance(TokenId),
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AssetFarmReward {
    /// The amount of reward distributed per day.
    #[serde(with = "u128_dec_format")]
    pub reward_per_day: Balance,
    /// The log base for the booster. Used to compute boosted shares per account.
    /// Including decimals of the booster.
    pub booster_log_bases: HashMap<TokenId, U128>,

    /// The amount of rewards remaining to distribute.
    #[serde(with = "u128_dec_format")]
    pub remaining_rewards: Balance,

    /// The total number of boosted shares.
    #[serde(with = "u128_dec_format")]
    pub boosted_shares: Balance,
    #[serde(skip)]
    pub reward_per_share: BigDecimal,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountFarmRewardView {
    pub reward_token_id: TokenId,
    pub asset_farm_reward: AssetFarmReward,
    #[serde(with = "u128_dec_format")]
    pub boosted_shares: Balance,
    #[serde(with = "u128_dec_format")]
    pub unclaimed_amount: Balance,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountFarmView {
    pub farm_id: FarmId,
    pub rewards: Vec<AccountFarmRewardView>,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AssetView {
    pub token_id: TokenId,
    #[serde(with = "u128_dec_format")]
    pub balance: Balance,
    /// The number of shares this account holds in the corresponding asset pool
    pub shares: Shares,
    /// The current APR for this asset (either supply or borrow APR).
    pub apr: BigDecimal,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct BoosterStaking {
    /// The amount of Booster token staked.
    #[serde(with = "u128_dec_format")]
    pub staked_booster_amount: Balance,
    /// The amount of xBooster token.
    #[serde(with = "u128_dec_format")]
    pub x_booster_amount: Balance,
    /// When the staked Booster token can be unstaked in nanoseconds.
    #[serde(with = "u64_dec_format")]
    pub unlock_timestamp: u64,
}

#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountDetailedView {
    pub account_id: AccountId,
    /// A list of assets that are supplied by the account (but not used a collateral).
    pub supplied: Vec<AssetView>,
    /// A list of assets that are used as a collateral.
    pub collateral: Vec<AssetView>,
    /// A list of assets that are borrowed.
    pub borrowed: Vec<AssetView>,
    /// Account farms
    pub farms: Vec<AccountFarmView>,
    /// Whether the account has assets, that can be farmed.
    pub has_non_farmed_assets: bool,
    /// Staking of booster token.
    pub booster_staking: Option<BoosterStaking>,
    pub booster_stakings: HashMap<TokenId, BoosterStaking>,
}

#[ext_contract(ext_burrow)]
pub trait ExtBurrow {
    fn get_account(&self, account_id: AccountId) -> Option<AccountDetailedView>;
}
