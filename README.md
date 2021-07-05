
## Interacting with the Contract

### Deploy contract to local-terra blockchain:

``
terracli tx wasm store artifacts/bonus.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block
``

Response:
```
height: 61188
txhash: E1F3AE7DB1FC1F2AEA92DF9408F2023B5B42CB478D72806CFF01192CEE493057
codespace: ""
code: 0
data: ""
rawlog: '[{"msg_index":0,"log":"","events":[{"type":"message","attributes":[{"key":"action","value":"store_code"},{"key":"module","value":"wasm"}]},{"type":"store_code","attributes":[{"key":"sender","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"},{"key":"code_id","value":"3"}]}]}]'
logs:
- msgindex: 0
  log: ""
  events:
  - type: message
    attributes:
    - key: action
      value: store_code
    - key: module
      value: wasm
  - type: store_code
    attributes:
    - key: sender
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    - key: code_id
      value: "3"   // <<<< code_id of the deployed contract
      info: ""
      gaswanted: 804268
      gasused: 802485
      tx: null
      timestamp: ""
```

### Create a contract instance

``
terracli tx wasm instantiate 3 '{"owner": "ra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
``

Response:
```
height: 61233
txhash: A09B0B8B843532A1BBA0C046519043BE5C5B779A5CFC08253525BD0DF5CEC08D
codespace: ""
code: 0
data: ""
rawlog: '[{"msg_index":0,"log":"","events":[{"type":"instantiate_contract","attributes":[{"key":"owner","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"},{"key":"code_id","value":"3"},{"key":"contract_address","value":"terra1wgh6adn8geywx0v78zs9azrqtqdegufuegnwep"}]},{"type":"message","attributes":[{"key":"action","value":"instantiate_contract"},{"key":"module","value":"wasm"},{"key":"sender","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}]}]}]'
logs:
- msgindex: 0
  log: ""
  events:
  - type: instantiate_contract
    attributes:
    - key: owner
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    - key: code_id
      value: "3"
    - key: contract_address
      value: terra1wgh6adn8geywx0v78zs9azrqtqdegufuegnwep // <<<<< contract_address
  - type: message
    attributes:
    - key: action
      value: instantiate_contract
    - key: module
      value: wasm
    - key: sender
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
info: ""
gaswanted: 124376
gasused: 123657
tx: null
timestamp: ""
```

### Register the merkle tree root

``
terracli tx wasm execute terra1wgh6adn8geywx0v78zs9azrqtqdegufuegnwep '{"register_merkle_root": {"merkle_root": "7b5014d5b73125a763b97331c4f4cdb47fd94151b8dbfc625b77fd2bd249a2b7"} }' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
``

### Query

``
terracli query wasm contract-store terra1wgh6adn8geywx0v78zs9azrqtqdegufuegnwep '{"merkle_root": {"stage": 1}}'
``
