use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, Promise};

mod events;

pub const STORAGE_PRICE_PER_BYTE: u128 = 10_000_000_000_000_000_000;

pub fn refund_storage(initial_storage: u64) {
    let current_storage = env::storage_usage();
    let attached_deposit = env::attached_deposit();

    let storage_log: events::EventLog = events::EventLog {
        standard: "nep171".to_string(),
        version: "storage-0.1.0".to_string(),
        event: events::SetInfoLog {
            initial_storage: initial_storage,
            current_storage: current_storage,
            attached_deposit: attached_deposit,
            memo: None,
        },
    };

    env::log_str(&storage_log.to_string());

    let refund_amount = if current_storage > initial_storage {
        let required_deposit =
            u128::from(current_storage - initial_storage) * STORAGE_PRICE_PER_BYTE;
        assert!(
            required_deposit <= attached_deposit,
            "The required attached deposit is {}, but the given attached deposit is is {}",
            required_deposit,
            attached_deposit,
        );
        attached_deposit - required_deposit
    } else {
        attached_deposit + u128::from(initial_storage - current_storage) * STORAGE_PRICE_PER_BYTE
    };
    if refund_amount > 0 {
        env::log_str(format!("Refunding {} tokens for storage", refund_amount).as_str());
        Promise::new(env::predecessor_account_id()).transfer(refund_amount);
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct IntoStorageKeyGenerator {
    pub ascii: u8,
}

impl Default for IntoStorageKeyGenerator {
    fn default() -> Self {
        Self { ascii: 64 }
    }
}

impl IntoStorageKeyGenerator {
    pub fn get_into_storage_key(&mut self) -> Vec<u8> {
        if self.ascii == 90 {
            self.ascii += 6;
        }
        self.ascii += 1;
        return vec![self.ascii];
    }
}
