use crate::*;

pub struct MockWhitelistContract(pub Contract);

impl MockWhitelistContract {
    pub async fn add_whitelist(
        &self,
        caller: &Account,
        account_id: &AccountId,
    ) -> Result<ExecutionFinalResult> {
        caller
            .call(self.0.id(), "add_whitelist")
            .args_json(json!({
                "account_id": account_id
            }))
            .max_gas()
            .transact()
            .await
    }

    pub async fn allow_all(&self, caller: &Account) -> Result<ExecutionFinalResult> {
        caller
            .call(self.0.id(), "allow_all")
            .args_json(json!({}))
            .max_gas()
            .transact()
            .await
    }

    pub async fn is_whitelisted(&self, staking_pool_account_id: &AccountId) -> Result<bool> {
        self.0
            .call("is_whitelisted")
            .args_json(json!({
                "staking_pool_account_id": staking_pool_account_id,
            }))
            .view()
            .await
            .unwrap()
            .json::<bool>()
    }
}
