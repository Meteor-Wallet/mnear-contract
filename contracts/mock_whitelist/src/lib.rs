use near_sdk::store::IterableSet;
use near_sdk::{near, AccountId, PanicOnDefault};

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    whitelist: IterableSet<AccountId>,
    allow_all: bool,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            whitelist: IterableSet::new(b"w"),
            allow_all: false,
        }
    }

    pub fn add_whitelist(&mut self, account_id: AccountId) {
        self.whitelist.insert(account_id);
    }

    pub fn allow_all(&mut self) {
        self.allow_all = true;
    }

    pub fn is_whitelisted(&self, staking_pool_account_id: AccountId) -> bool {
        self.allow_all || self.whitelist.contains(&staking_pool_account_id)
    }
}
