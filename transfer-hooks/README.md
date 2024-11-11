# TokenWithHooks

This project implements a token with transfer hooks to enforce custom logic, such as on-chain royalties.

## Prerequisites

- [Solana CLI](https://solana.com/docs/getting-started/installation)
- [Rust](https://www.rust-lang.org/tools/install)

## Setup

1. Clone the repository:
    ```bash
    git clone <repository_url>
    cd christex-bounty
    ```

2. Build the contract:
    ```bash
    cargo build-bpf
    ```

## Deployment

1. Deploy the contract:
    ```bash
    ./scripts/deploy.sh
    ```

## Testing

1. Run the tests:
    ```bash
    cargo test-bpf
    ```

## Interaction

1. Interact with the deployed contract:
    ```bash
    ./scripts/interact.sh
    ```

## License

This project is licensed under the MIT License.