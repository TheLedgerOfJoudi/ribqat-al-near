use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};
use ribqat_al_near::{refund_storage, IntoStorageKeyGenerator};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenOwners {
    owners_to_tokens: UnorderedMap<String, String>,
    tokens_to_owners: UnorderedMap<String, String>,
    into_storage_key_generator: IntoStorageKeyGenerator,
}

impl Default for TokenOwners {
    fn default() -> Self {
        let mut gen: IntoStorageKeyGenerator = IntoStorageKeyGenerator::default();
        Self {
            owners_to_tokens: UnorderedMap::new(gen.get_into_storage_key()),
            tokens_to_owners: UnorderedMap::new(gen.get_into_storage_key()),
            into_storage_key_generator: gen,
        }
    }
}

#[near_bindgen]
impl TokenOwners {
    #[payable]
    pub fn set_info(&mut self, token_id: &String, account_id: &String) {
        let initial_storage = env::storage_usage();
        self.tokens_to_owners.insert(&token_id, &account_id);
        self.owners_to_tokens.insert(&account_id, &token_id);
        refund_storage(initial_storage);
    }

    pub fn get_owner(&self, token_id: &String) -> String {
        match self.tokens_to_owners.get(&token_id) {
            Some(owner) => owner,
            None => "No owner".to_string(),
        }
    }

    pub fn get_token(&self, owner_id: &String) -> String {
        match self.owners_to_tokens.get(&owner_id) {
            Some(token) => token,
            None => "No token".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    #[test]
    #[should_panic]
    fn test_set_info_with_no_deposit() {
        let context: VMContext = VMContextBuilder::new().context;
        testing_env!(context);
        let mut contract: TokenOwners = TokenOwners::default();
        let token_id: String = "your_token".to_string();
        let owner_id: String = "you.testnet".to_string();
        contract.set_info(&token_id, &owner_id);
        let owner_of_token: String = contract.get_owner(&token_id);
        let token_of_owner: String = contract.get_token(&owner_id);
        assert_eq!(owner_of_token, owner_id);
        assert_eq!(token_of_owner, token_id);
    }

    #[test]
    fn test_set_info_with_deposit(){
        let mut context: VMContext = VMContextBuilder::new().context;
        let mut contract: TokenOwners = TokenOwners::default();
        let token_id: String = "your_token".to_string();
        let owner_id: String = "you.testnet".to_string();
        pub const STORAGE_PRICE_PER_BYTE: u128 = 10_000_000_000_000_000_000;
        context.attached_deposit = STORAGE_PRICE_PER_BYTE * 1000;
        testing_env!(context.clone());
        contract.set_info(&token_id, &owner_id);
        let owner_of_token: String = contract.get_owner(&token_id);
        let token_of_owner: String = contract.get_token(&owner_id);
        assert_eq!(owner_of_token, owner_id);
        assert_eq!(token_of_owner, token_id);
    }
}
