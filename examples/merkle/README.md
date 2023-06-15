# Merkle Roots With Bonsai

## Getting Started

This project utilizes RISC Zero's Bonsai network and zkVM to calculate the Merkle
root of an array of 32-byte hashes (H256) and store the result directly on a
blockchain. The Bonsai network's ability to perform off-chain computations is a
key component in the calculation of these Merkle roots. Users interact with this
system through a function call, which results in the creation of a Merkle root
that is stored in a smart contract.

### Methods

The main functionality of the code is encapsulated in the `merkle_root` function.

`merkle_root` is responsible for calculating the Merkle root of an array of `H256`
values. It hashes each `H256` value with Sha256, constructs a Merkle tree from
the hashed values, and extracts the Merkle root from the tree. This is
useful for creating a compact representation of a large set of data, which can
be used to verify whether a specific piece of data is part of the set.
The entry point reads an array of `H256` values from the user, calculates the
Merkle root of the array, and commits the result back to the contract.
The input is read as a slice of bytes, which is then decoded into an array of `H256`
values. This allows the function to handle a large amount of input data efficiently.

### Contracts

The contract `Merkle` is a smart contract that stores the Merkle roots calculated
by the `merkle_root` bonsai callback. It maps each Merkle root to a unique ID.
The contract provides a `merkle_root` function that allows users to submit an
array of 32 `H256` values, and the callback function stores the resulting Merkle
root in the contract when the calculation is complete. This contract is useful
for storing the results of Merkle root calculations on the blockchain, where they
can be publicly verified.

### Build

Running the following will build the project, including Ethereum contracts and RISC Zero guest program.

```bash
cargo build
```

### Test

Running the following will run all tests, including Ethereum contracts and RISC Zero guest program.

```bash
cargo test
```

#### Deploy

The `deploy` command shows how to deploy your application to Bonsai and to an Ethereum chain.

```text
cargo --bin deploy -- --help
Usage: deploy --ethereum-node-url <ETHEREUM_NODE_URL> --bonsai-url <BONSAI_URL> --bonsai-proxy-contract-address <BONSAI_PROXY_CONTRACT_ADDRESS> --bonsai-api-key <BONSAI_API_KEY> --ethereum-private-key <ETHEREUM_PRIVATE_KEY>

Options:
  -e, --ethereum-node-url <ETHEREUM_NODE_URL>
          JSON RPC URL for an Ethereum node that will serve call and transaction requests. Currently only HTTP(S) URLs are supported.
  -b, --bonsai-url <BONSAI_URL>
          URL for the Bonsai service for to upload the ELF binary
  -p, --bonsai-proxy-contract-address <BONSAI_PROXY_CONTRACT_ADDRESS>
          Ethereum contract address for the Bonsai proxy
      --bonsai-api-key <BONSAI_API_KEY>
          API Key for Bonsai to authorize requests
      --ethereum-private-key <ETHEREUM_PRIVATE_KEY>
          Ethereum private key to use for sending transactions
```

#### Poke

The `poke` command shows a basic interaction with the application contract.
Making a request an waiting for a callback from Bonsai.

```text
cargo --bin poke -- --help
Usage: poke --ethereum-node-url <ETHEREUM_NODE_URL> --hello-bonsai-contract-address <HELLO_BONSAI_CONTRACT_ADDRESS> --ethereum-private-key <ETHEREUM_PRIVATE_KEY> <N>

Arguments:
  <N>  Value of n to use as the input to the Fibonacci calculation

Options:
  -e, --ethereum-node-url <ETHEREUM_NODE_URL>
          JSON RPC URL for an Ethereum node that will serve call and transaction requests. Currently only HTTP(S) URLs are supported
  -a, --hello-bonsai-contract-address <HELLO_BONSAI_CONTRACT_ADDRESS>
          HelloBonsai contract address
      --ethereum-private-key <ETHEREUM_PRIVATE_KEY>
          Ethereum private key to use for sending transactions
```

[Bonsai]: https://dev.bonsai.xyz/
[RISC Zero]: https://www.risczero.com/
[ethers]: https://docs.rs/ethers/latest/ethers/
[Cargo]: https://doc.rust-lang.org/cargo/
[RISC Zero examples]: https://github.com/risc0/risc0/tree/main/examples
[RISC-V]: https://www.risczero.com/docs/reference-docs/about-risc-v
[waitlist]: https://fmree464va4.typeform.com/to/t6hZD54Z
