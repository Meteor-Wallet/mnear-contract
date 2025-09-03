use crate::*;

pub type Rate = u128;

#[near(serializers = [borsh])]
pub(crate) struct EstimatedBalance {
    pub balance: Balance,
    pub last_updated: EpochHeight,
    pub apr: Rate,
}

impl EstimatedBalance {
    pub fn compute_interest_rate(&self, current_balance: Balance) -> Rate {
        if self.last_updated == 0 {
            return 0;
        }

        let delta_balance = U384::from(current_balance - self.balance);
        let delta_time = U384::from(env::block_timestamp() - self.last_updated);
        let big_divisor = U384::from(BIG_DIVISOR);

        let rate = (delta_balance * big_divisor) / delta_time;

        rate.as_u128()
    }

    pub fn estimate_current_balance(&self) -> Balance {
        if self.last_updated == 0 {
            return self.balance;
        }

        let delta_time = U384::from(env::block_timestamp() - self.last_updated);
        let rate = U384::from(self.apr);
        let big_divisor = U384::from(BIG_DIVISOR);

        let future_balance = U384::from(self.balance) + (rate * delta_time) / big_divisor;

        future_balance.as_u128()
    }
}

#[ext_contract(ext_rnear)]
pub trait ExtRnear {
    fn ft_price(&self) -> U128;
}

#[near]
impl Contract {
    pub fn rnear_deposit_rate(&self) -> U128 {
        U128(self.internal_convert_rnear_to_near(ONE_NEAR))
    }
}

impl Contract {
    pub fn internal_convert_rnear_to_near(&self, amount: Balance) -> Balance {
        let rnear_price = self.data().rnear_price.estimate_current_balance();

        let amount_u256 = U256::from(amount);
        let rnear_price_u256 = U256::from(rnear_price);
        let one_near_u256 = U256::from(ONE_NEAR);

        let near_u256 = amount_u256 * rnear_price_u256 / one_near_u256;
        near_u256.as_u128()
    }

    pub fn handle_rnear_ft_price(&mut self, promise_result: PromiseResult) {
        match promise_result {
            PromiseResult::Successful(result) => {
                let current_price = serde_json::from_slice::<U128>(&result)
                    .expect(ERR_FAILED_TO_PARSE_RNEAR_PRICE)
                    .0;

                let new_apr = self.data().rnear_price.compute_interest_rate(current_price);

                self.data_mut().rnear_price = EstimatedBalance {
                    balance: current_price,
                    last_updated: env::block_timestamp(),
                    apr: new_apr,
                };
            }
            PromiseResult::Failed => {
                panic!("{}", ERR_FAILED_TO_GET_RNEAR_PRICE);
            }
        }
    }
}
