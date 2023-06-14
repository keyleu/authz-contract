# Authz Contract

This is a contract that allows sending from a contract that has previously been given authz rights by another address.

## Steps
1. Build the contract.
2. Store the contract on chain.
3. Instantiate the contract.
4. Give the contract authz rights for a bank send.
5. Execute the contract to transfer funds.

### For a production-ready (compressed) build:

Run the following from the repository root

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.13
```

The optimized contracts are generated in the artifacts/ directory.