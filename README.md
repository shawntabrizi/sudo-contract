# sudo-contract

Sudo Contract is a Substrate runtime module which adds an authorization layer for putting new Wasm smart contracts on the blockchain.

## Purpose

The Contract module which is included with Substrate allows all users to be able to create and deploy Wasm smart contracts on their custom blockchain.

Smart contract deployment through the Contract module takes two steps:

1. Putting a Wasm compiled smart contract on the blockchain
2. Creating an instance of the Wasm smart contract with it's own Contract account and storage

This runtime module adds an authorization layer to the Contract module such that the first step of putting the Wasm contract on the blockchain can only be performed by the "Sudo" key, which is defined by the Sudo module.

With this module, it would be possible to curate the smart contracts which get added to your blockchain, hopefully reducing the danger of users on your platform.

## Dependencies

### Traits

This module does not depend on any externally defined traits.

### Modules

This module has direct dependencies on:

* SRML Sudo Module
* SRML Contract Module

Both must be included in your runtime for this module to function. Instructions for this below.

## Installation

You can find an example of adding the Sudo Contract module to a runtime [here](https://github.com/shawntabrizi/substrate-package/commit/c0c1e4604db279c5940f528c378575fa2c5aaf7a).

### Runtime `Cargo.toml`

To add this module to your runtime you will need to modify how the SRML Contract module is added too.

Your runtime's `Cargo.toml` file should look something like:

```rust
[dependencies.srml-contract]
default_features = false
git = 'https://github.com/paritytech/substrate.git'
package = 'srml-contract'
branch = 'v1.0'

[dependencies.contract]
default_features = false
git = 'https://github.com/shawntabrizi/sudo-contract.git'
package = 'sudo-contract'
branch = 'v1.0'
```

Note how we imported the `srml-contract` module using the name `srml-contract`. This then allows us to import `sudo-contract` as `contract`, which is important for interacting with the UI.

You also need to update your runtime's `std` feature to include `srml-contract` and `contract`:

```rust
std = [
    ...
    'srml-contract/std',
    'contract/std',
]
```

### Runtime `lib.rs`

You should implement this module's trait like so:

```rust
impl contract::Trait for Runtime {}
```

While updating the SRML Contract module's trait to use `srml_contract`:

```rust
impl srml_contract::Trait for Runtime {
	type Currency = Balances;
	type Call = Call;
	type Event = Event;
	type Gas = u64;
	type DetermineContractAddress = srml_contract::SimpleAddressDeterminator<Runtime>;
	type ComputeDispatchFee = srml_contract::DefaultDispatchFeeComputor<Runtime>;
	type TrieIdGenerator = srml_contract::TrieIdFromParentCounter<Runtime>;
	type GasPayment = ();
}
```

Finally, in your `construct_runtime!` macro:

```rust
Contract: srml_contract::{Module, Storage, Config<T>, Event<T>},
SudoContract: contract::{Module, Call},
```

Note that we have removed the `Call` function from `srml_contract`. This is important so that users cannot bypass the Sudo Contract module by simply calling the SRML Contract module directly.

### Genesis Configuration

The Sudo Contract module itself does not have any genesis configuration, but you will need to make sure that the genesis configuration for the SRML Contract module is updated to use `srml_contract`:

```rust
srml_contract: Some(contract_config),
```

## Reference Docs

You can view the reference docs for this module by running:

```
cargo doc --open
```
