# Ownership

This may be the simplest implementation a ownership of addresses.
It contains a set of admins that are defined upon creation.
Any of those admins may `Execute` any message via the contract,

To make this slightly less minimalistic, you can allow the admin set
to be mutable or immutable. If it is mutable, then any admin may
(a) change the admin set and (b) freeze it (making it immutable).

While largely an example contract for OWS, this has various real-world use-cases,
such as a common account that is shared among multiple trusted devices,
or trading an entire account (used as 1 of 1 mutable).

## Allowing Custom Messages

By default, this doesn't support `CustomMsg` in order to be fully generic
among blockchains. However, all types are Generic over `T`, and this is only
fixed in `handle`. You can import this contract and just redefine your `handle`
function, setting a different parameter to `ExecuteMsg`, and you can produce
a chain-specific message.

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via:

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```sh
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/ownership.wasm .
ls -l ownership.wasm
sha256sum ownership.wasm
```

Or for a production-ready (optimized) build, run a build command:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```

This will compile all packages in the `contracts` directory and output the stripped and optimized wasm code under the
`artifacts` directory as output, along with a `checksums.txt` file.

If you hit any issues there and want to debug, you can try to run the following in each contract dir:
`RUSTFLAGS="-C link-arg=-s" cargo build --release --target=wasm32-unknown-unknown --locked`

## Ownership Spec: Proxy Contracts

OWS is a specification for proxy contracts in Rust. It is a very simple, but flexible interface designed for
the case where one contract is meant to hold assets (or rights) on behalf of other contracts.

The simplest example is a contract that will accept messages from the creator and resend them from its address. Simply
by making this transferable, you can then begin to transfer non-transferable assets (eg. staked tokens, voting power,
etc).

You can imagine more complex examples, such as a "1 of N" multisig, or conditional approval, where "sub-accounts" have
the right to spend a limited amount of funds from this account, with a "admin account" retaining full control.

The common denominator is that they allow you to immediately execute arbitrary `Msg` in the same transaction.

## Messages

`Execute{msgs}` - This accepts `Vec<Msg>` and checks permissions before re-dispatching all those messages from the
contract address. It emits the following attributes:

| Key      | Value        |
| -------- | ------------ |
| "action" | "execute"    |
| "owner"  | [msg sender] |

### Queries

`CanExecute{sender, msg}` - This accepts one `Msg` and checks permissions, returning true or false based on the
permissions. If `CanExecute` returns true then a call to `Execute` from that sender, with the same message, before any
further state changes, should also succeed. This can be used to dynamically provide some client info on a generic ows
contract without knowing the extension details. (eg. detect if they can send coins or stake)

## Quality Control

One of the basic metrics of assurance over code quality is how much is covered by unit tests. There are several tools
available for Rust to do such analysis and we will describe one below. This should be used as a baseline metric to give
some confidence in the code.

Beyond code coverage metrics, just having a robust PR review process with a few more trained eyes looking for bugs is
very helpful in detecting paths the original coder was not aware of. This is more subjective, but looking at the
relevant PRs and depth of discussion can give an idea how much review was present.

After that, fuzzing it (ideally with an intelligent fuzzer that understands the domain) can be valuable. And beyond that
formal verification can provide even more assurance (but is very time-consuming and expensive).

### Code Coverage

I recommend the use of [tarpaulin](https://github.com/xd009642/tarpaulin): `cargo install cargo-tarpaulin`

To get some nice interactive charts, you can go to the root directory and run:

`cargo tarpaulin -o html` and then `xdg-open tarpaulin-report.html` (or just `open` on MacOS).

Once you find a package that you want to improve, you can do the following to just analyze this package, which gives
much faster turn-around:

`cargo tarpaulin -o html --packages cw3-fixed-multisig`

Note that it will produce a code coverage report for the entire project, but only the coverage in that package is the
real value. If does give quick feedback for you if you unit test writing was successful.

## Contributing

See our [Contributing Guidelines](CONTRIBUTING.md).

## Generating changelog

To generate a changelog we decided to use
[github-changelog-generator](https://github.com/github-changelog-generator/github-changelog-generator).

To install tool you need Ruby's `gem` package manager.

  ```sh  $ gem --user install github_changelog_generator```

And put `$HOME/.gem/ruby/*/bin/` into your PATH.

Generating changelog file first time:

  ```sh  $ github_changelog_generator -u username -p projectname```

Appending next releases could be done adding `--base` flag:

  ```sh  $ github_changelog_generator -u username -p projectname --base CHANGELOG.md```

If you hit GitHub's 50 requests/hour limit, please follow
[this](https://github.com/github-changelog-generator/github-changelog-generator#github-token) guide to create a token
key which you can pass using `--token` flag.

There's also a convenience `scripts/update_changelog.sh`, which can take a --since-tag parameter (to avoid processing
the entire history). It can also auto-detect the latest version tag for you, with --latest-tag.
