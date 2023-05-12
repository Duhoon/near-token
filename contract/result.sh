
# near deploy --wasmFile res/fungible_token.wasm --accountId aicotest.test412ock.testnet
# near call aicotest.test412ock.testnet new_default_meta '{"owner_id":"test412ock.testnet", "total_supply":"1000000000"}' --accountId test412ock.testnet

# near view aicotest.test412ock.testnet ft_balance_of '{"account_id":"test412ock.testnet"}'
near view aicotest.test412ock.testnet ft_metadata