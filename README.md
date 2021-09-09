# Anyswap Integration on Casper
Building the following contracts for Anyswap integration in Casper.
1. An ERC20 contract that can mint and burn tokens.
2. A contract that can lock and unlock CSPR.
3. A factory contract that creates ERC20 tokens.

## Done
- [x] Implemented the `ERC20` contract.
- [x] Implemented the `cspr-holder` contract.
- [x] Implemented the `factory` contract.
- [x] Implemented the integration tests for the contracts.
- [x] Implemented a bash script `testnet.sh` which facilitates the contracts deployment and testing on `Casper-testnet`.

## Install compilation target
Make sure `wasm32-unknown-unknown` is installed.
```bash
$ make prepare
```

## Install wasm-strip
`wasm-strip` helps reduce the compiled wasm contract's size. It can be found in the `wabt` package.
```bash
$ sudo apt-get install wabt
```

## Test Math Library
```bash
$ cargo test -p libs
```

## Build contracts
```bash
$ make build-contract
```

## Test contracts locally
Test logic and smart contracts.
```bash
$ make test
```

## Test contracts on casper-testnet
Testing the contracts locally using `TestContext` has its limitations.
That's why we created a bash script that helps us test our contracts more easily on the `Casper Testnet`.  
The steps in order to test our contract's endpoints are the following:
1. Install the nightly version of the compiler and Casper client.
```bash
$ rustup toolchain install nightly
$ cargo +nightly-2021-06-17 install casper-client
```
2. Create keys for every contract deployer using `casper-client`.
```bash
$ casper-client keygen keys/erc20
$ casper-client keygen keys/cspr_holder
$ casper-client keygen keys/factory
$ casper-client keygen keys/governance
```
3. Connect `casper-signer` to https://testnet.cspr.live/ and import your created accounts.
4. For every account, visit https://testnet.cspr.live/tools/faucet and put the account's public key and click on `Request Tokens`.
5. After receiving `CSPR` from the faucet, getting an account's `main purse` can be done by visiting the account's profile and clicking on the `Show raw data` button.
6. Give permission to the testnet script.
```bash
$ chmod +x testnet.sh
```
7. Execute the following command to get the script's syntax.
```bash
$ ./testnet.sh syntax
```
8. After calling a deploy (transaction), you will get a `deploy hash` that in your terminal. To check the deploy status, execute the following.
```bash
$ ./testnet.sh check_status <DEPLOY_HASH>
```
The result will be written in the `deploy_status.json` file.