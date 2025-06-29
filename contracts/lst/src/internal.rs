use crate::*;

impl Contract {

    pub(crate) fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.data().owner_id,
            ERR_NOT_OWNER
        );
    }

    pub(crate) fn internal_get_beneficiaries(&self) -> HashMap<AccountId, u32> {
        self.data()
            .beneficiaries
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    /// When there are rewards, a part of them will be
    /// given to executor, manager or treasury by minting new LST tokens.
    pub(crate) fn internal_distribute_staking_rewards(&mut self, rewards: u128) {
        let hashmap: HashMap<AccountId, u32> = self.internal_get_beneficiaries();
        for (account_id, bps) in hashmap.iter() {
            let reward_near_amount: u128 = bps_mul(rewards, *bps);
            // mint extra LST for him
            self.internal_mint_beneficiary_rewards(account_id, reward_near_amount);
        }
    }

    /// Mint new LST tokens to given account at the current price.
    /// This will DECREASE the LST price.
    #[pause]
    fn internal_mint_beneficiary_rewards(
        &mut self,
        account_id: &AccountId,
        near_amount: u128,
    ) -> ShareBalance {
        let shares = self.num_shares_from_staked_amount_rounded_down(near_amount);
        self.mint_lst(account_id, shares, Some("beneficiary rewards"));
        shares
    }
}
