# TokenWithHooks

This project implements a token with transfer hooks to enforce custom logic, such as on-chain royalties.

## Prerequisites

- [Solana CLI](https://solana.com/docs/getting-started/installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [Hardhat](https://hardhat.org/)

## Setup

1. Clone the repository:
    ```bash
    git clone <repository_url>
    cd christex-bounty
    ```

2. Install dependencies:
    ```bash
    npm install
    ```

## Deployment

1. Compile the contracts:
    ```bash
    npx hardhat compile
    ```

2. Deploy the contract:
    ```bash
    npx hardhat run scripts/deploy.js --network <network_name>
    ```

## Testing

1. Run the tests:
    ```bash
    npx hardhat test
    ```

## Interaction

1. Interact with the deployed contract:
    ```bash
    npx hardhat run scripts/interact.js --network <network_name>
    ```

## License

This project is licensed under the MIT License.