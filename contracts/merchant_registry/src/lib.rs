#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    MerchantId(String),
    MerchantWallet(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct MerchantData {
    merchant_id: String,
    wallet: Address,
    business_name: String,
    active: bool,
}

pub struct MerchantRegistryContract;

impl MerchantRegistryContract {
    fn assert_owner(env: &Env) {
        let owner: Address = env.storage().get(&DataKey::Owner).unwrap();
        if env.invoker() != owner {
            panic!("unauthorized");
        }
    }
}

#[contractimpl]
impl MerchantRegistryContract {
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().has(&DataKey::Owner) {
            panic!("already initialized");
        }
        env.storage().set(&DataKey::Owner, &owner);
    }

    pub fn register_merchant(
        env: Env,
        merchant_id: String,
        wallet: Address,
        business_name: String,
        active: bool,
    ) {
        MerchantRegistryContract::assert_owner(&env);
        let id_key = DataKey::MerchantId(merchant_id.clone());
        if env.storage().has(&id_key) {
            panic!("merchant_id already registered");
        }

        let wallet_key = DataKey::MerchantWallet(wallet.clone());
        if env.storage().has(&wallet_key) {
            panic!("wallet already registered");
        }

        let data = MerchantData {
            merchant_id: merchant_id.clone(),
            wallet: wallet.clone(),
            business_name,
            active,
        };
        env.storage().set(&id_key, &data);
        env.storage().set(&wallet_key, &data);
    }

    pub fn set_active(env: Env, merchant_id: String, active: bool) {
        MerchantRegistryContract::assert_owner(&env);
        let id_key = DataKey::MerchantId(merchant_id.clone());
        let mut data: MerchantData = env.storage().get(&id_key).unwrap();
        data.active = active;
        env.storage().set(&id_key, &data);
        env.storage().set(&DataKey::MerchantWallet(data.wallet.clone()), &data);
    }

    pub fn get_merchant_by_id(env: Env, merchant_id: String) -> Option<MerchantData> {
        env.storage().get(&DataKey::MerchantId(merchant_id))
    }

    pub fn get_merchant_by_wallet(env: Env, wallet: Address) -> Option<MerchantData> {
        env.storage().get(&DataKey::MerchantWallet(wallet))
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
    fn test_initialize_and_register() {
        let env = Env::default();
        let owner = test_address(&env, 1);
        let merchant = test_address(&env, 2);

        MerchantRegistryContract::initialize(env.clone(), owner.clone());
        MerchantRegistryContract::register_merchant(
            env.clone(),
            String::from_str(&env, "merchant_001"),
            merchant.clone(),
            String::from_str(&env, "Example Store"),
            true,
        );

        let stored = MerchantRegistryContract::get_merchant_by_id(env.clone(), String::from_str(&env, "merchant_001")).unwrap();
        assert_eq!(stored.merchant_id, String::from_str(&env, "merchant_001"));
        assert_eq!(stored.wallet, merchant);
        assert_eq!(stored.business_name, String::from_str(&env, "Example Store"));
        assert!(stored.active);
    }

    #[test]
    fn test_set_active() {
        let env = Env::default();
        let owner = test_address(&env, 1);
        let merchant = test_address(&env, 2);

        MerchantRegistryContract::initialize(env.clone(), owner.clone());
        MerchantRegistryContract::register_merchant(
            env.clone(),
            String::from_str(&env, "merchant_002"),
            merchant.clone(),
            String::from_str(&env, "Active Store"),
            true,
        );
        MerchantRegistryContract::set_active(env.clone(), String::from_str(&env, "merchant_002"), false);

        let stored = MerchantRegistryContract::get_merchant_by_id(env.clone(), String::from_str(&env, "merchant_002")).unwrap();
        assert!(!stored.active);
    }
}
