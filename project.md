# KnotPay QR Payment Network

KnotPay is a Stellar-based merchant payment platform that enables customers to pay merchants by scanning QR codes. The organization is split into three repositories so each layer is modular, auditable, and independently deployable.

## Overview

This repo contains the Soroban contract layer for the KnotPay platform. The contracts handle merchant registration, payment request lifecycle, fee configuration, and payment verification.

## Architecture

```
KnotPay/
├── knotpay-contracts
├── knotpay-backend
└── knotpay-frontend
```

### Repository roles

- `knotpay-contracts` — on-chain Soroban contracts for merchant registry, payment requests, and fee management.
- `knotpay-backend` — REST APIs, QR generation, Stellar monitoring, settlement, and webhooks.
- `knotpay-frontend` — merchant dashboard, customer payment flow, and analytics.

## Contract repo structure

- `contracts/merchant_registry` — merchant onboarding and registry contract
- `contracts/payment_request` — request creation and verification contract
- `contracts/fee_manager` — fee configuration and calculation contract
- `scripts/` — deployment, initialization, and upgrade helper scripts
- `tests/` — workspace-level testing guidance

## Core flow

1. Merchant registers and creates a payment request.
2. Backend generates a QR code for the request.
3. Customer scans the QR code and sends a Stellar payment.
4. Backend monitors Stellar transactions and verifies payment details.
5. The payment request contract updates request state to Paid.
6. Merchant dashboard updates with the completed payment.

## Supported assets

- XLM
- USDC on Stellar
- Merchant-issued assets
- Future stablecoins and asset-backed tokens

## Contract features

### Merchant Registry

- `initialize(owner)`
- `register_merchant(merchant_id, wallet, business_name, active)`
- `set_active(merchant_id, active)`
- `get_merchant_by_id(merchant_id)`
- `get_merchant_by_wallet(wallet)`

### Payment Request

- `create_request(request_id, merchant, amount, asset, description, expiry)`
- `verify_payment(request_id, tx_hash, amount, asset)`
- `cancel_request(request_id)`
- `get_request(request_id)`

### Fee Manager

- `initialize(owner, fee_rate_bps)`
- `set_fee_rate(fee_rate_bps)`
- `get_fee_rate()`
- `calculate_fee(amount)`

## Notes

This repository focuses on the contract-side implementation. Backend and frontend repos are separate and should integrate with this contract layer through Soroban contract invocations.
