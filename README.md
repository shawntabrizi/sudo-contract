# sudo-contract

A Substrate runtime module that adds an access control layer to the Contract module. With this module, only the Sudo key can call `put_code` to submit new Wasm smart contracts that can be deployed. Any user should be able to deploy a smart contract from the ones that have been uploaded.

NOTE: Currently not working.

```bash
Shawns-MBP:sudo-contract shawntabrizi$ cargo build
   Compiling sudo-contract v0.1.0 (/Users/shawntabrizi/Documents/GitHub/sudo-contract)
error[E0624]: method `update_schedule` is private
  --> src/lib.rs:23:13
   |
23 |             <contract::Module<T>>::update_schedule(schedule)
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0624]: method `put_code` is private
  --> src/lib.rs:33:13
   |
33 |             <contract::Module<T>>::put_code(origin, gas_limit, code)
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0624]: method `call` is private
  --> src/lib.rs:44:13
   |
44 |             <contract::Module<T>>::call(origin, dest, value, gas_limit, data)
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0624]: method `create` is private
  --> src/lib.rs:55:13
   |
55 |             <contract::Module<T>>::create(origin, endowment, gas_limit, code_hash, data)
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 4 previous errors
```
