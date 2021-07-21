# BNS Airdrop

This is a sample contract that realizes the airdrop functionality. The smart contract is built on CosmWasm library and dedicated to terra ecosystem (tested on LocalTerra only).

## Build the contract

``
cargo run-script optimize
``

## Interacting with the Contract

### Deploy contract to local-terra blockchain:

``
terracli tx wasm store artifacts/bns_airdrop.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block
``

### Create a contract instance

* code_id - id of the deployed contract;
* owner - human address of the contract owner (admin);
* token - human address of the token contract;

``
terracli tx wasm instantiate <code_id> '{"owner": "<owner_address>", "token": "<token_address>"}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
``


### Register the merkle tree root

* contract_address;
* merkle_root;

``
terracli tx wasm execute <contract_address> '{"merkle_root": {"node": "<merkle_root>"} }' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
``

### Query

* contract_address;
* claim_address;
* stage_index;

``
terracli query wasm contract-store <contract_address> '{"merkle_root": {"stage_index": <stage_index>}}'
``

``
terracli query wasm contract-store <contract_address> '{"is_claimed": {"stage_index": 1, "address": "<claim_address>" }}'
``

### Claim airdrop

``
terracli tx wasm execute <contract_address> '{"claim": {"stage_index": 1, "amount": "1111", "proof": ["9446abab3c6c205e08648131ed635e9fd53cb58ecc495eeb98e20bb5ddde8e75", "ba6494fc293a91a73a7ee94d31beba417ed5ad43a648227613920c39237f77bc"]}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
``
