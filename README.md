
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

``
terracli tx wasm instantiate <code_id> '{"owner": <owner_address>}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
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
