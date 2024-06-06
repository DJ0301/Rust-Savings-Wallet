#SavingsWallet

##Overview

The SavingsWallet is a smart contract designed to facilitate automatic savings from user transactions. When a user performs a transaction, the contract rounds up the amount and transfers the remainder to a designated savings account. Additionally, the contract includes functionality for freezing and unfreezing accounts to prevent or allow transactions.

##Contract Functions
Admin Management
set_admin(env: Env, new_admin: Address)
Sets a new admin for the contract. The current admin must authorize this action.


##Deployment
To deploy this smart contract on the Soroban blockchain platform, compile the code and deploy the resulting bytecode. Once deployed, interact with the contract using Soroban-compatible tools or libraries. Smart Contract Address: "CAFKXFGHSWB5VKN3PKVN3NKW5CUD6EPN6PILNVRCAPQHFV2OZSJC4YCZ"

##Project Structure
This repository uses the recommended structure for a Soroban project:

.
├── contracts
│   └── Rust-Savings-Wallet
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
├── Cargo.lock
└── README.md
