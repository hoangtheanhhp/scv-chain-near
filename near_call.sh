near call hoangtheanh.testnet new --accountId hoangtheanh.testnet
near call hoangtheanh.testnet create_item '{"id": "888","title": "I am Anh","score": 5,"content": "IPFS link"}' --accountId hoangtheanh.testnet
near call hoangtheanh.testnet get_item_info '{"id": "888"}' --accountId hoangtheanh.testnet
near call hoangtheanh.testnet revoke_item '{"id": "999"}' --accountId hoangtheanh.testnet
near call hoangtheanh.testnet reset_all --accountId hoangtheanh.testnet