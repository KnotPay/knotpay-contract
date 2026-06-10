#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    FeeRate,
}

pub struct FeeManagerContract;

impl FeeManagerContract {
    fn assert_owner(env: &Env) {
        let owner: Address = env.storage().get(&DataKey::Owner).unwrap();
        if env.invoker() != owner {
            panic!("unauthorized");
        }
    }
}

#[contractimpl]
impl FeeManagerContract {
    pub fn initialize(env: Env, owner: Address, fee_rate_bps: u32) {
        if env.storage().has(&DataKey::Owner) {
            panic!("already initialized");
        }
        if fee_rate_bps > 10_000 {
            panic!("fee rate must be <= 10000 basis points");
        }
        env.storage().set(&DataKey::Owner, &owner);
        env.storage().set(&DataKey::FeeRate, &fee_rate_bps);
    }

    pub fn set_fee_rate(env: Env, fee_rate_bps: u32) {
        FeeManagerContract::assert_owner(&env);
        if fee_rate_bps > 10_000 {
            panic!("fee rate must be <= 10000 basis points");
        }
        env.storage().set(&DataKey::FeeRate, &fee_rate_bps);
    }

    pub fn get_fee_rate(env: Env) -> u32 {
        env.storage().get(&DataKey::FeeRate).unwrap()
    }

    pub fn calculate_fee(env: Env, amount: i128) -> i128 {
        let rate: u32 = env.storage().get(&DataKey::FeeRate).unwrap();
        if amount <= 0 {
            panic!("amount must be positive");
        }
        amount * (rate as i128) / 10_000
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, BytesN, Env};

    fn test_address(env: &Env, seed: u8) -> Address {
        let mut data = [0u8; 32];
        data[0] = seed;
        Address::from_contract_id(env, &BytesN::from_array(env, &data))
    }

    #[test]
    fn test_initialize_and_calculate_fee() {
        let env = Env::default();
        let owner = test_address(&env, 1);

        FeeManagerContract::initialize(env.clone(), owner.clone(), 50);
        assert_eq!(FeeManagerContract::get_fee_rate(env.clone()), 50);
        assert_eq!(FeeManagerContract::calculate_fee(env.clone(), 1000), 5);
    }

    #[test]
    fn test_set_fee_rate() {
        let env = Env::default();
        let owner = test_address(&env, 1);

        FeeManagerContract::initialize(env.clone(), owner.clone(), 100);
        FeeManagerContract::set_fee_rate(env.clone(), 200);
        assert_eq!(FeeManagerContract::get_fee_rate(env.clone()), 200);
    }
}
