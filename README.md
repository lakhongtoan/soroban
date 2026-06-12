# Stellar Diploma Management System

## Problem

Universities and employers often struggle to verify the authenticity of graduation certificates, leading to time-consuming manual verification processes and risks of diploma fraud.

## Solution

We built a decentralized diploma management system on the Stellar blockchain that allows universities to issue, store, and verify graduation certificates securely and transparently.

## Why Stellar

Stellar's Soroban smart contracts provide a fast, low-cost, and secure platform for storing and verifying diploma records on-chain.

## Target User

* Universities and educational institutions
* Graduates
* Employers and recruiters
* Government education agencies

## Live Demo

* Network: Stellar Testnet

* **Contract ID**: `YOUR_CONTRACT_ID`

* **Transaction**: `https://stellar.expert/explorer/testnet/tx/YOUR_TX_HASH`

## Features

* Issue graduation diplomas on-chain
* Store diploma information securely
* Verify diploma authenticity instantly
* Prevent unauthorized modifications
* Transparent and immutable record keeping

## Smart Contract Functions

### Add Diploma

Creates a new diploma record on the blockchain.

### Get Diploma

Retrieves diploma information using a diploma ID.

### Verify Diploma

Checks whether a diploma exists and is valid.

### Delete Diploma

Removes a diploma record (admin-only functionality for demonstration purposes).

## How to Run

### 1. Clone Repository

```bash
git clone https://github.com/yourname/stellar-diploma-management.git
cd stellar-diploma-management
```

### 2. Build Contract

```bash
cd contracts/diploma-management
stellar contract build
```

### 3. Run Tests

```bash
cargo test
```

### 4. Deploy to Testnet

```bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/diploma_management.wasm \
--source-account student \
--network testnet
```

### 5. Run Frontend

```bash
cd frontend
npx serve .
```

## Diploma Data Structure

Each diploma contains:

* Diploma ID
* Student Name
* Student ID
* Major
* Classification / Grade
* Issue Date
* Diploma File Hash (SHA-256)
* Issuer Address

## Tech Stack

* Smart Contract: Rust + Soroban SDK v25
* Blockchain: Stellar Testnet
* Frontend: HTML, CSS, JavaScript
* SDK: @stellar/stellar-sdk
* Wallet: Freighter
* Storage: Stellar Ledger
* Development Tools: Stellar CLI

## Project Architecture

```text
University
    │
    ▼
Issue Diploma
    │
    ▼
Soroban Smart Contract
    │
    ├── Store Diploma
    ├── Verify Diploma
    └── Retrieve Diploma
    │
    ▼
Student / Employer
```

## Future Improvements

* University role-based authorization
* PDF diploma upload with IPFS integration
* QR code verification
* Multi-university support
* Diploma revocation mechanism
* Public verification portal

## Team

* Your Name
* University Name
* Final Year Student
* Email: [khongtoan331@gmail.com](mailto:khongtoan331@example.com)
* Telegram: @yourtelegram

## License

This project is developed for educational and demonstration purposes on Stellar Testnet.
