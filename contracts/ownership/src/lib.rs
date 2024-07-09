/*!
This may be the simplest implementation of Ows, an ownership of addresses.
It contains a set of admins that are defined upon creation.
Any of those admins may `Execute` any message via the contract,
per the OWS spec.

To make this slighly less minimalistic, you can allow the admin set
to be mutable or immutable. If it is mutable, then any admin may
(a) change the admin set and (b) freeze it (making it immutable).

While largely an example contract for OWS, this has various real-world use-cases,
such as a common account that is shared among multiple trusted devices,
or trading an entire account (used as 1 of 1 mutable). Most of the time,
this can be used as a framework to build your own,
more advanced OWS implementations.

For more information on this contract, please check out the
[README](https://github.com/golnarmordadi/Rust_Proxy_Contract/README.md).
*/

pub mod contract;
pub mod error;
#[cfg(test)]
mod integration_tests;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
