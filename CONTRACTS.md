# Contracts

You can use this contract as a template for your desired contract. It will serve as a starting point for your project. Follow the steps below to get started:

## Creating a new contract

Use [`template`](https://github.com/golnarmordadi/Rust_Proxy_Contract) as a
basis, in particular the `main` branch.

```bash
cd contracts
cargo generate --git https://github.com/golnarmordadi/Rust_Proxy_Contract.git --branch main --name PROJECT_NAME
cd PROJECT_NAME
rm -rf .git
rm .gitignore
rm .cargo-ok
git add .
```

Now, integrate it into the CI and build system

1. Edit `.circleci/config.yml`, copy an existing contracts job and replace the name.
Then add your new job to the jobs list on top.

1. Add to the `ALL_CONTRACTS` variable in `scripts/publish.sh`

1. Edit the root `Cargo.toml` file and add a `profile.release.package.CONTRACT_NAME`
section, with your package name.

1. Run `cargo build && cargo test` in the new contract dir

1. Commit all changes and push the branch. Open a PR and ensure the CI runs this.
