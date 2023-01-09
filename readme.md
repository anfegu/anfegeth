# AnfeGETH 1.0 P2P Handshake

This Rust project demonstrates how to perform a handshake with a P2P local node using the `web3` crate and `coregeth`.

## Prerequisites

- Rust (latest stable version recommended)
- `coregeth` (latest version recommended)

## Installation

1. Install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
2. Install `coregeth` by following the instructions on the [Geth website](https://geth.ethereum.org/docs/install-and-build/installing-geth). Or the next:

***************************************************************************
## Prerequisites

- Go 1.16 or higher
- `go-ethereum` v1.10.26 or higher

## Getting Started

1. Download and install Go from the [Go website](https://golang.org/dl/).
2. Install `go-ethereum` version `v1.10.26`:

    go get -u github.com/ethereum/go-ethereum@v1.10.26

    unzip and follow the usual Linux: place it in the preferred directory (e.g usr/local/) and continue.
***************************************************************************

3. Clone this repository:

    git clone https://github.com/anfegu/anfegeth

4. Navigate to the root directory of the project and install the dependencies:

    cd anfegeth

    cargo install

## Running the AnfeGETH 1.0 P2P Handshake 

1. Start `coregeth` in a separate terminal window:

    Run `geth` with the `--http` and `--network` flags:

    in the installed directory: 

        ./geth --http --network=my-network-id

## NETWORKS LIST AVAILABLES:
- "id": "1", "name": "Mainnet"
- "id": "3", "name": "Ropsten"
- "id": "4", "name": "Rinkeby"
- "id": "5", "name": "Goerli"
- "id": "1337802", "name": "Kiln"
- "id": "11155111", "name": "Sepolia"

2. In the terminal window where you installed the project, run the anfeGETH 1.0 P2P Handshake:

    cargo run

3. Follow the prompts in the console menu to perform the handshake with the local `geth` node.


    If the handshake is successful, you should see a message indicating that the handshake was completed.

## Testing

To run the tests for this project, use the `cargo test` command:

    cargo test

This will run all the tests in the project and show the results.

You can also use the `--verbose` flag to see more detailed output:

    cargo test --verbose

Or, you can specify the name of a specific test or test module to run a subset of the tests:

    cargo test my_test

    cargo test my_module


## Additional Resources

- [web3 crate documentation](https://docs.rs/web3)
- [coregeth documentation](https://geth.ethereum.org/docs)
