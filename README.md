# sudo-contract

A Substrate runtime module that adds an access control layer to the Contract module. With this module, only the Sudo key can call `put_code` to submit new Wasm smart contracts that can be deployed. Any user should be able to deploy a smart contract from the ones that have been uploaded.

NOTE: Untested.
