use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum GlobalError {
    #[error("Invalid json: {0}")]
    InvalidJson(near_sdk::serde_json::Error),
    #[error("State error: Cannot load in contract due to missing state")]
    ContractStateIsMissing,
}

#[derive(thiserror::Error, Debug)]
pub enum AccountError {
    #[error("{0} not exist")]
    NotExist(AccountId),
    #[error("the account's conversions have reached the limit ({0})")]
    TooManyConversions(u8),
    #[error("{0} is locked")]
    Locked(AccountId),
    #[error("{0} not empty")]
    NotEmpty(AccountId),
}

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("Unsupported source token ID: {0}")]
    UnsupportedSourceTokenID(AccountId),
    #[error("Unsupported target token ID: {0}")]
    UnsupportedTargetTokenID(AccountId),
    #[error("Unsupported conversion strategy")]
    UnsupportedConversionStrategy,
    #[error("Mismatch between strategy's source token ID({0}) and transfer token ID({1})")]
    StrategyTokenMismatch(AccountId, AccountId),
    #[error("Insufficient available balance of target token, expect: {0}, available: {1}")]
    InsufficientTargetTokenAmount(u128, u128),
    #[error("Insufficient available balance of source token, expect: {0}, available: {1}")]
    InsufficientSourceTokenAmount(u128, u128),
    #[error("Invalid conversion rate string: {0}")]
    InvalidConversionRateString(String),
    #[error("Conversion rate must be greater than zero")]
    ZeroConversionRate,
    #[error("Non-existent conversion strategy")]
    NonExistentConversionStrategy,
    #[error("max_conversions_per_account exceeds the maximum limit of {0}")]
    MaxConversionsPerAccountExceedsLimit(u8),
    #[error("Batch claim exceeds the maximum limit of {0}")]
    BatchClaimExceedsLimit(usize),
    #[error("Invalid conversion id {0}")]
    InvalidConversionId(u64),
    #[error("The conversion id {0} cannot be claimed")]
    ClaimZero(u64),
    #[error("Invalid max_conversion_limit: {0}")]
    InvalidMaxConversionLimit(String),
    #[error("Exceeds max_conversion_limit, expect: {0}, remain: {1}")]
    ExceedsMaxConversionLimit(u128, u128),
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Insufficient deposit, expect: {0}, got: {1}")]
    InsufficientDeposit(u128, u128),
    #[error("Insufficient available balance, expect: {0}, available: {1}")]
    InsufficientAvailableBalance(u128, u128),
    #[error("Force unregister unsupported")]
    ForceUnregisterUnsupported,
}

#[derive(thiserror::Error, Debug)]
pub enum ContractError {
    #[error("{0}.")]
    Global(#[from] GlobalError),
    #[error("Conversion error: {0}.")]
    Conversion(#[from] ConversionError),
    #[error("Account error: {0}.")]
    Account(#[from] AccountError),
    #[error("Account error: {0}.")]
    Storage(#[from] StorageError),
}

impl near_sdk::FunctionError for ContractError {
    fn panic(&self) -> ! {
        env::panic_str(&self.to_string())
    }
}

// initialization
pub const ERR_ALREADY_INITIALIZED: &str = "Already initialized";
pub const ERR_NO_ENOUGH_INIT_DEPOSIT: &str = "The account doesn't have enough balance for initialization";

// owner
pub const ERR_NOT_OWNER: &str = "Only owner can perform this action";

// account
pub const ERR_FORCE_UNGREGISTER: &str = "Force unregister is not allowed";
pub const ERR_UNREGISTER_POSITIVE_UNSTAKED: &str = "Can't unregister the account with the positive unstaked balance";

// fraction
pub const ERR_BPS_SUM_ONE: &str = "bps sum should be less than 1";

// beneficiary
pub const ERR_TOO_MANY_BENEFICIARIES: &str = "Too many beneficiaries";

// stake
pub const ERR_NON_POSITIVE_STAKING_AMOUNT: &str = "Staking amount should be positive";
pub const ERR_NON_POSITIVE_CALCULATED_STAKING_SHARE: &str =
    "The calculated number of \"stake\" shares received for staking should be positive";
pub const ERR_NO_ENOUGH_UNSTAKED_BALANCE: &str = "Not enough unstaked balance to stake";
pub const ERR_NO_ENOUGH_WITHDRAW_BALANCE: &str = "No enough unstaked balance to withdraw";

// unstake
pub const ERR_NON_POSITIVE_UNSTAKING_AMOUNT: &str = "Unstaking amount should be positive";
pub const ERR_NON_POSITIVE_CALCULATED_UNSTAKING_SHARE: &str = "Invariant violation. The calculated number of \"stake\" shares for unstaking should be positive";
pub const ERR_NON_POSITIVE_TOTAL_STAKED_BALANCE: &str = "The total staked balance can't be 0";
pub const ERR_NON_POSITIVE_TOTAL_STAKE_SHARES: &str = "The total number of stake shares can't be 0";
pub const ERR_CONTRACT_NO_STAKED_BALANCE: &str = "Invariant violation. The calculated number of \"stake\" shares for unstaking should be positive";

// drain operations
pub const ERR_NON_ZERO_WEIGHT: &str = "Validator weight must be zero for drain operation";
pub const ERR_NON_ZERO_BASE_STAKE_AMOUNT: &str =
    "Validator base stake amount must be zero for drain operation";
pub const ERR_BAD_UNSTAKED_AMOUNT: &str = "Validator unstaked amount too large for drain unstake";
pub const ERR_NON_ZERO_STAKED_AMOUNT: &str =
    "Validator staked amount must be zero when drain withdraw";
pub const ERR_DRAINING: &str = "Validator is currently in draining process";
pub const ERR_NOT_IN_DRAINING: &str =
    "Validator is not in draining process. Cannot run drain withdraw";

// deposit
pub const ERR_NON_POSITIVE_DEPOSIT_AMOUNT: &str = "Deposit amount should be positive";

// withdraw
pub const ERR_NON_POSITIVE_WITHDRAWAL_AMOUNT: &str = "Withdrawal amount should be positive";
pub const ERR_NO_ENOUGH_UNSTAKED_BALANCE_TO_WITHDRAW: &str =
    "Not enough unstaked balance to withdraw";
pub const ERR_UNSTAKED_BALANCE_NOT_AVAILABLE: &str =
    "The unstaked balance is not yet available due to unstaking delay";
pub const ERR_NO_ENOUGH_CONTRACT_BALANCE: &str =
    "No enough balance in contract to perform withdraw";

// validator
pub const ERR_MIN_RESERVE: &str = "Contract min reserve error";
pub const ERR_VALIDATOR_NOT_EXIST: &str = "Validator not exist";
pub const ERR_VALIDATOR_ALREADY_EXIST: &str = "Validator already exists";
pub const ERR_VALIDATOR_IN_USE: &str = "Validator is in use, cannot remove";
pub const ERR_NO_ENOUGH_GAS: &str = "No enough gas";
pub const ERR_BAD_VALIDATOR_LIST: &str = "Bad validator list";
pub const ERR_VALIDATOR_NOT_WHITELISTED: &str = "Validator not whitelisted";
pub const ERR_VALIDATOR_WHITELIST_NOT_SET: &str = "Validator whitelist not set";

pub const ERR_VALIDATOR_UNSTAKE_AMOUNT: &str = "No enough amount to unstake from validator";
pub const ERR_VALIDATOR_UNSTAKE_WHEN_LOCKED: &str =
    "Cannot unstake from a pending release validator";
pub const ERR_VALIDATOR_WITHDRAW_WHEN_LOCKED: &str =
    "Cannot withdraw from a pending release validator";
pub const ERR_VALIDATOR_ALREADY_EXECUTING_ACTION: &str = "Validator is already executing action";
pub const ERR_VALIDATOR_SYNC_BALANCE_NOT_EXPECTED: &str =
    "Validator sync balance is expected to be called after stake or unstake";

// LST
pub const ERR_NON_POSITIVE_SHARES: &str = "Share number should be positive";
