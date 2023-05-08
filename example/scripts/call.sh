
near call $CONTRACT set_info '{"token_id": "second_token", "account_id" : "alhadi2.testnet"}' --accountId "alhadi.testnet" --depositYocto 4050000000000000000001
near view $CONTRACT get_owner '{"token_id": "second_token"}' 
near view $CONTRACT get_token '{"owner_id": "alhadi2.testnet"}' 