# near call dev-1684386111429-18223800021823 new_vote '{"prefix": "vote"}' --accountId test412ock.testnet

# near call dev-1684386528645-83109045951471 new_vote '{"prefix": "vote"}' --accountId test412ock.testnet

# near call vote.dev-1684386528645-83109045951471  
# near view vote.dev-1684386528645-83109045951471 get_is_voting

# near call dev-1684400856457-89780740888245 add_community '{"community_id":"dev-1684400781469-20417359155339"}' --accountId test412ock.testnet

# near call dev-1684400856457-89780740888245 new_vote '{"prefix": "vote","community_id":"dev-1684400781469-20417359155339"}' --accountId test412ock.testnet

# near view vote.dev-1684400856457-89780740888245 get_is_voting
# near view vote.dev-1684400856457-89780740888245 get_community_account_id
near view vote.dev-1684400856457-89780740888245 get_result