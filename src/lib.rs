// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use contract::{CodeHash, Schedule};
use runtime_primitives::traits::StaticLookup;
use srml_support::traits::Currency;
use srml_support::{decl_module, dispatch::Result};
use system::ensure_root;

pub type BalanceOf<T> =
    <<T as contract::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: contract::Trait + sudo::Trait {}

decl_module! {
    /// Dispatchable functions copied from https://github.com/paritytech/substrate/tree/master/srml/contract
    pub struct Module<T: Trait> for enum Call where origin: <T as system::Trait>::Origin {

        /// Updates the schedule for metering contracts.
        ///
        /// The schedule must have a greater version than the stored schedule.
        fn update_schedule(schedule: Schedule<T::Gas>) -> Result {
            <contract::Module<T>>::update_schedule(schedule)
        }

        /// Checks for `Root` origin before forwarding to `put_code` in the Contract module.
        fn put_code(
            origin,
            #[compact] gas_limit: T::Gas,
            code: Vec<u8>
        ) -> Result {
            ensure_root(origin)?;
            <contract::Module<T>>::put_code(origin, gas_limit, code)
        }

        /// Simply forwards to the `call` function in the Contract module.
        fn call(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            #[compact] value: BalanceOf<T>,
            #[compact] gas_limit: T::Gas,
            data: Vec<u8>
        ) -> Result {
            <contract::Module<T>>::call(origin, dest, value, gas_limit, data)
        }

        /// Simply forwards to the `create` function in the Contract module.
        fn create(
            origin,
            #[compact] endowment: BalanceOf<T>,
            #[compact] gas_limit: T::Gas,
            code_hash: CodeHash<T>,
            data: Vec<u8>
        ) -> Result {
            <contract::Module<T>>::create(origin, endowment, gas_limit, code_hash, data)
        }

        fn on_finalize(n: T::BlockNumber) {
            <contract::Module<T>>::on_finalize(n)
        }
    }
}