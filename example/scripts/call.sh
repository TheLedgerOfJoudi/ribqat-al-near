
near call $CONTRACT set_info '{"token_id": "first_token1", "account_id" : "alhadi1.testnet"}' --accountId "alhadi.testnet" --depositYocto 4050000000000000000001
near view $CONTRACT get_owner '{"token_id": "first_token1"}' 
near view $CONTRACT get_token '{"owner_id": "alhadi1.testnet"}' 