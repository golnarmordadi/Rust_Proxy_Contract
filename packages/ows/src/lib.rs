/*!
Ows is a specification for proxy contracts in Rust.
It is a very simple, but flexible interface designed for the case
where one contract is meant to hold assets (or rights) on behalf of
other contracts.

The simplest example is a contract that will accept messages from
the creator and resend them from its address. Simply by making this
transferable, you can then begin to transfer non-transferable assets
(eg. staked tokens, voting power, etc).

You can imagine more complex examples, such as a "1 of N" multisig,
or conditional approval, where "sub-accounts" have the right to spend
a limited amount of funds from this account, with a "admin account"
retaining full control.

The common denominator is that they allow you to immediately
execute arbitrary `msg` in the same transaction.

For more information on this specification, please check out the
[README](https://github.com/golnarmordadi/Rust_Proxy_Contract/README.md).
*/

pub mod helpers;
pub mod msg;
pub mod query;

pub use crate::helpers::OwsContract;
pub use crate::msg::OwsExecuteMsg;
pub use crate::query::{CanExecuteResponse, OwsQueryMsg};
