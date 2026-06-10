

provide a fully project structure on how to archive this on stellar 
QR-code payment network

i want the org to be knotpay
and also the org to be of 3 repo

For a QR-Code Payment Network on Stellar under the organization KnotPay, a clean production architecture is to split it into 3 repositories:

Project Overview
KnotPay is a merchant payment network that allows customers to pay merchants by scanning QR codes.

Flow:

Merchant creates payment request

QR code generated

Customer scans QR

Wallet signs Stellar payment

Backend verifies transaction

Merchant receives confirmation

Dashboard updates in real time

Supported Assets:

XLM

USDC

Merchant-issued assets

Future stablecoins

Organization Structure
KnotPay/
│
├── knotpay-contracts
├── knotpay-backend
└── knotpay-frontend
Repository 1: knotpay-contracts
Purpose:

Smart contracts

Merchant registry

Payment request creation

Escrow (optional)

Settlement logic

Fee management

Technology:

Soroban

Rust

Stellar SDK

Structure
knotpay-contracts/
│
├── contracts/
│   │
│   ├── merchant_registry/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── storage.rs
│   │   │   ├── types.rs
│   │   │   ├── errors.rs
│   │   │   └── admin.rs
│   │
│   ├── payment_request/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── qr.rs
│   │   │   ├── payment.rs
│   │   │   ├── storage.rs
│   │   │   └── events.rs
│   │
│   ├── fee_manager/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── fees.rs
│   │   │   └── admin.rs
│
├── tests/
│   ├── merchant_test.rs
│   ├── payment_test.rs
│   └── fee_test.rs
│
├── scripts/
│   ├── deploy.sh
│   ├── initialize.sh
│   └── upgrade.sh
│
├── Cargo.toml
└── README.md
Main Contract Features
Merchant Registration
register_merchant()
Stores:

Merchant {
    merchant_id,
    wallet,
    business_name,
    active
}
Create Payment Request
create_request()
Input:

amount
asset
merchant_id
description
expiry
Output:

payment_id
Verify Payment
verify_payment()
Checks:

tx_hash
amount
merchant
asset
Platform Fees
set_fee_rate()
Example:

0.5%
Automatically deduct fee.

Repository 2: knotpay-backend
Purpose:

APIs

QR generation

Stellar monitoring

Webhooks

Merchant management

Settlement engine

Technology:

Node.js

TypeScript

PostgreSQL

Redis

Stellar SDK

Structure
knotpay-backend/
│
├── src/
│   │
│   ├── api/
│   │   ├── auth/
│   │   ├── merchant/
│   │   ├── payments/
│   │   ├── qr/
│   │   └── webhooks/
│   │
│   ├── services/
│   │   ├── stellar/
│   │   ├── qr/
│   │   ├── notification/
│   │   ├── settlement/
│   │   └── analytics/
│   │
│   ├── workers/
│   │   ├── payment-monitor.ts
│   │   ├── settlement-worker.ts
│   │   └── webhook-worker.ts
│   │
│   ├── database/
│   │   ├── migrations/
│   │   ├── seeders/
│   │   └── models/
│   │
│   ├── middleware/
│   │
│   ├── config/
│   │
│   └── app.ts
│
├── tests/
├── docs/
├── Dockerfile
├── docker-compose.yml
└── README.md
Database Schema
Merchants
merchants
id
wallet
business_name
email
status
created_at
Payment Requests
payment_requests
id
merchant_id
amount
asset
description
qr_data
status
expires_at
Payments
payments
id
payment_request_id
tx_hash
amount
asset
payer
status
confirmed_at
QR Generation Logic
Merchant enters:

Amount = 25 USDC
Backend creates:

{
  "merchant":"G....",
  "amount":"25",
  "asset":"USDC",
  "memo":"KP123456"
}
Encoded into QR.

Payment Monitoring
Worker continuously watches Stellar.

Horizon API
↓
Detect payment
↓
Match memo
↓
Verify amount
↓
Mark paid
↓
Notify merchant
Webhook System
Merchant receives:

{
  "payment_id":"123",
  "status":"completed",
  "amount":"25"
}
Repository 3: knotpay-frontend
Purpose:

Merchant dashboard

Customer payment page

Analytics

QR display

Technology:

Next.js

TypeScript

Tailwind

Stellar Wallet Kit

Structure
knotpay-frontend/
│
├── public/
│
├── src/
│   │
│   ├── app/
│   │   ├── dashboard/
│   │   ├── payments/
│   │   ├── merchants/
│   │   ├── settings/
│   │   └── analytics/
│   │
│   ├── components/
│   │   ├── qr/
│   │   ├── tables/
│   │   ├── charts/
│   │   ├── wallet/
│   │   └── ui/
│   │
│   ├── hooks/
│   │
│   ├── services/
│   │   ├── api.ts
│   │   └── stellar.ts
│   │
│   ├── store/
│   │
│   └── types/
│
├── tests/
├── next.config.js
└── README.md
Frontend Pages
Merchant Dashboard
Total Sales
Total Transactions
Pending Payments
Revenue Chart
Create QR
Merchant enters:

Amount
Asset
Description
Click:

Generate QR
Payment Status Page
Shows:

Waiting Payment
Processing
Completed
Failed
Expired
Transaction History
Filters:

Today
Week
Month
Asset
Merchant
Stellar Transaction Flow
Merchant
    │
    ▼
Create Payment Request
    │
    ▼
Backend Generates QR
    │
    ▼
Customer Scans QR
    │
    ▼
Wallet Opens
    │
    ▼
Signs Stellar Payment
    │
    ▼
Stellar Network
    │
    ▼
Payment Monitor Detects TX
    │
    ▼
Contract Verification
    │
    ▼
Merchant Dashboard Updated
MVP Features
Phase 1
Merchant registration

QR generation

XLM payments

USDC payments

Transaction history

Dashboard

Phase 2
Soroban payment requests

Merchant fees

Settlement reports

Webhooks

Mobile support

Phase 3
NFC payments

Offline QR mode

Loyalty rewards

Subscription billing

Multi-store merchants

Cross-border settlements

Repo Relationship Description
If a grant form asks "You selected multiple repos. Please describe how they are related."

Answer:

The KnotPay platform is split into three repositories following a modular architecture. knotpay-contracts contains Soroban smart contracts responsible for merchant registration, payment requests, and fee management. knotpay-backend provides APIs, QR generation, Stellar transaction monitoring, settlement services, and webhook integrations. knotpay-frontend delivers the merchant dashboard and customer-facing payment interfaces. Together, the three repositories form the complete QR-code payment network built on the Stellar ecosystem.


