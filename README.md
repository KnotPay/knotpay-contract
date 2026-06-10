# KnotPay Contract Repository

This repository contains the Soroban smart contract layer for the KnotPay QR payment platform on Stellar.

## What this repo contains

The contract layer is split into three dedicated Soroban crates, each handling a core domain:

- `contracts/merchant_registry` — merchant onboarding, registration, and status management
- `contracts/payment_request` — payment request creation, payment validation, and lifecycle management
- `contracts/fee_manager` — platform fee configuration, authorization, and fee calculation

Additional support files:

- `scripts/` — deployment, initialization, and upgrade helper scripts
- `tests/` — workspace-level guidance and testing notes
- `.gitignore` — common ignore patterns for build artifacts and temporary files

## Why this repo exists

KnotPay uses a modular on-chain contract design so the payments, merchant registry, and fee logic are separated and auditable. This repository provides the majority of KnotPay's contract-side implementation and is intended to deliver 80-90% of the contract work for the platform.

## Key features

- Owner-based authorization for contract initialization and administrative actions
- Merchant registry with merchant ID and wallet lookup
- Payment request states: `Pending`, `Paid`, `Cancelled`, `Expired`
- Fee manager with basis-point limits and safe fee calculation
- Independent contract crates for easier development and testing

## Repository structure

```text
.
├── Cargo.toml
├── README.md
├── contracts/
│   ├── fee_manager/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/lib.rs
│   ├── merchant_registry/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/lib.rs
│   └── payment_request/
│       ├── Cargo.toml
│       ├── README.md
│       └── src/lib.rs
├── scripts/
│   ├── deploy.sh
│   ├── initialize.sh
│   └── upgrade.sh
├── src/
│   └── lib.rs
└── tests/
    └── README.md
```

## Getting started

### Prerequisites

- Rust toolchain installed (`rustup`)
- Soroban SDK installed if you want to build or deploy Soroban contracts

### Build

```bash
cargo build --workspace
```

### Test

```bash
cargo test
```

## Contract summaries

### Merchant Registry

- Initialize the contract owner
- Register merchants by merchant ID and wallet address
- Enable or disable merchant status
- Query merchants by ID or wallet

### Payment Request

- Create payment requests with merchant, amount, asset, description, and expiry
- Verify payments against stored request data
- Cancel requests before payment
- Track request status transitions

### Fee Manager

- Initialize with an owner and a fee rate (basis points)
- Update the fee rate securely by owner only
- Calculate fee amounts against an input payment amount

## Development notes

- Each contract lives in its own workspace member to keep the implementation isolated and testable.
- The `scripts/` folder provides stubs for deployment and upgrade commands. Customize them for your Soroban environment.
- Add more tests inside each contract crate as the logic evolves.

## Next steps

To complete the KnotPay platform, this repo should be paired with external repositories for the backend and frontend:

- `knotpay-backend` — APIs, QR generation, payment monitoring, settlement, and webhook handling
- `knotpay-frontend` — merchant dashboard, payment flows, and analytics

## License

Specify your project license here.


This repository contains the Soroban smart contract layer for the KnotPay payment network.

## Purpose

The contract repository defines the on-chain business logic for KnotPay, including:

- Merchant registration and status management
- Payment request creation and verification
- Platform fee configuration and fee calculation

## Workspace structure

- `contracts/merchant_registry` — merchant onboarding and registry contract
- `contracts/payment_request` — payment request lifecycle and verification contract
- `contracts/fee_manager` — fee setup and calculation contract
- `scripts/` — deployment and upgrade helper scripts
- `tests/` — contract testing guidance

## Repo progress

This repo targets 80-90% of the KnotPay contract layer by providing:

- separate Soroban contract crates for each major domain
- owner-based authorization for sensitive contract actions
- payment request state management with pending/paid/cancelled/expired states
- fee management with basis-point rate limits and fee calculation

## Getting started

Install the Rust toolchain and Soroban SDK, then run:

```bash
cargo build --workspace
cargo test
```

If `cargo` is not installed, install it via `rustup`.

## Notes

- Backend and frontend repositories are separate and are not included here.
- Use the `scripts/` folder as a starting point for deployment automation.
