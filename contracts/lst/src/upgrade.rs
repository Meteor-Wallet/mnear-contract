use crate::*;

#[near]
impl Contract {
    /// A method to migrate a state during the contract upgrade.
    /// Can only be called after upgrade method.
    #[handle_result(aliased)]
    #[private]
    #[init(ignore_state)]
    pub fn migrate_state() -> ContractResult<Self> {
        let mut contract: Contract =
            env::state_read().ok_or(GlobalError::ContractStateIsMissing)?;
        contract.data = match contract.data {
            VersionedContractData::Current(data) => VersionedContractData::Current(data),
        };
        Ok(contract)
    }

    /// Returns semver of this contract.
    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
