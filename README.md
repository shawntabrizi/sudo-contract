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

### Runtime `Cargo.toml`

To add this module to your runtime you will need to modify how the SRML Contract module is added too.

Your runtime's `Cargo.toml` file should look something like:

```rust
[dependencies.contract]
default_features = false
git = 'https://github.com/shawntabrizi/sudo-contract.git'
package = 'sudo-contract'
branch = 'v1.0'
```

and update your runtime's `std` feature to include this module:

```rust
std = [
    ...
    'sudo_contract/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for the module test_module
impl sudo_contract::Trait for Runtime {}
```

and include it in your `construct_runtime!` macro:

```rust
SudoContract: substrate_module_template::{Module, Call, Storage, Event<T>},
```

## Reference Docs

You can view the reference docs for this module by running:

```
cargo doc --open
```

or by visiting this site: <Add Your Link>