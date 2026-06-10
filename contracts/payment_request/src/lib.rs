#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub enum RequestStatus {
    Pending,
    Paid,
    Cancelled,
    Expired,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Request(String),
}

#[contracttype]
#[derive(Clone)]
pub struct PaymentRequestData {
    merchant: Address,
    amount: i128,
    asset: String,
    description: String,
    expiry: u64,
    status: RequestStatus,
}

pub struct PaymentRequestContract;

fn is_request_expired(env: &Env, expiry: u64) -> bool {
    let ts = env.ledger().timestamp();
    ts > expiry
}

#[contractimpl]
impl PaymentRequestContract {
    pub fn create_request(
        env: Env,
        request_id: String,
        merchant: Address,
        amount: i128,
        asset: String,
        description: String,
        expiry: u64,
    ) {
        let key = DataKey::Request(request_id.clone());
        if env.storage().has(&key) {
            panic!("payment request already exists");
        }
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let request = PaymentRequestData {
            merchant,
            amount,
            asset,
            description,
            expiry,
            status: RequestStatus::Pending,
        };
        env.storage().set(&key, &request);
    }

    pub fn verify_payment(
        env: Env,
        request_id: String,
        tx_hash: String,
        amount: i128,
        asset: String,
    ) -> bool {
        let key = DataKey::Request(request_id.clone());
        let mut request: PaymentRequestData = env.storage().get(&key).unwrap();

        if matches!(request.status, RequestStatus::Paid | RequestStatus::Cancelled) {
            return false;
        }

        if is_request_expired(&env, request.expiry) {
            request.status = RequestStatus::Expired;
            env.storage().set(&key, &request);
            return false;
        }

        if request.amount != amount || request.asset != asset {
            return false;
        }

        request.status = RequestStatus::Paid;
        env.storage().set(&key, &request);
        true
    }

    pub fn cancel_request(env: Env, request_id: String) {
        let key = DataKey::Request(request_id.clone());
        let mut request: PaymentRequestData = env.storage().get(&key).unwrap();
        if matches!(request.status, RequestStatus::Paid | RequestStatus::Cancelled) {
            panic!("cannot cancel completed or cancelled request");
        }
        request.status = RequestStatus::Cancelled;
        env.storage().set(&key, &request);
    }

    pub fn get_request(env: Env, request_id: String) -> Option<PaymentRequestData> {
        env.storage().get(&DataKey::Request(request_id))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, BytesN, Env, String};

    fn test_address(env: &Env, seed: u8) -> Address {
        let mut data = [0u8; 32];
        data[0] = seed;
        Address::from_contract_id(env, &BytesN::from_array(env, &data))
    }

    #[test]
    fn test_create_and_verify_request() {
        let env = Env::default();
        let merchant = test_address(&env, 1);
        let request_id = String::from_str(&env, "req_001");

        PaymentRequestContract::create_request(
            env.clone(),
            request_id.clone(),
            merchant.clone(),
            25,
            String::from_str(&env, "USDC"),
            String::from_str(&env, "Test payment"),
            9999999999,
        );

        let request = PaymentRequestContract::get_request(env.clone(), request_id.clone()).unwrap();
        assert_eq!(request.amount, 25);
        assert_eq!(request.asset, String::from_str(&env, "USDC"));
        assert!(matches!(request.status, RequestStatus::Pending));

        let verified = PaymentRequestContract::verify_payment(
            env.clone(),
            request_id.clone(),
            String::from_str(&env, "tx_hash_001"),
            25,
            String::from_str(&env, "USDC"),
        );
        assert!(verified);

        let request = PaymentRequestContract::get_request(env.clone(), request_id.clone()).unwrap();
        assert!(matches!(request.status, RequestStatus::Paid));
    }

    #[test]
    fn test_cancel_request() {
        let env = Env::default();
        let merchant = test_address(&env, 2);
        let request_id = String::from_str(&env, "req_002");

        PaymentRequestContract::create_request(
            env.clone(),
            request_id.clone(),
            merchant.clone(),
            10,
            String::from_str(&env, "XLM"),
            String::from_str(&env, "Cancel test"),
            9999999999,
        );

        PaymentRequestContract::cancel_request(env.clone(), request_id.clone());
        let request = PaymentRequestContract::get_request(env.clone(), request_id.clone()).unwrap();
        assert!(matches!(request.status, RequestStatus::Cancelled));
    }
}
