# KnotPay Contract REST Integration Guide

This document explains how a REST backend application can connect to the KnotPay Soroban contracts and use contract APIs for merchant registration, payment request handling, and fee management.

## Overview

This repository contains the Soroban contract implementation for KnotPay. The REST backend should interact with the Stellar network and Soroban contracts through a Soroban client or SDK.

The core contract crates are:

- `merchant_registry` — merchant onboarding and registry
- `payment_request` — payment request creation and payment verification
- `fee_manager` — platform fee configuration and fee calculation

A REST backend typically performs:

1. Contract deployment and configuration on Stellar/Soroban.
2. Contract invocation for operations such as registering merchants and creating payment requests.
3. Monitoring Stellar transactions and verifying payments against on-chain request state.

## Contract function mapping

### Merchant registry

- `initialize(owner)` — configure contract owner
- `register_merchant(merchant_id, wallet, business_name, active)` — add a merchant record
- `set_active(merchant_id, active)` — enable/disable merchants
- `get_merchant_by_id(merchant_id)` — retrieve merchant data
- `get_merchant_by_wallet(wallet)` — retrieve merchant data by wallet

### Payment request

- `create_request(request_id, merchant, amount, asset, description, expiry)` — create a payment request
- `verify_payment(request_id, tx_hash, amount, asset)` — verify a completed payment
- `cancel_request(request_id)` — cancel a pending request
- `get_request(request_id)` — read request details and status

### Fee manager

- `initialize(owner, fee_rate_bps)` — configure fee owner and rate
- `set_fee_rate(fee_rate_bps)` — update the fee rate
- `get_fee_rate()` — read current fee rate
- `calculate_fee(amount)` — compute fee amount for a payment

## REST backend design

A REST backend should expose endpoints that translate HTTP requests into Soroban contract calls.

Example REST endpoint mapping:

- `POST /merchants` -> `register_merchant`
- `GET /merchants/:merchantId` -> `get_merchant_by_id`
- `PUT /merchants/:merchantId/status` -> `set_active`
- `POST /payment-requests` -> `create_request`
- `POST /payment-requests/:requestId/verify` -> `verify_payment`
- `POST /payment-requests/:requestId/cancel` -> `cancel_request`
- `GET /payment-requests/:requestId` -> `get_request`
- `GET /fees` -> `get_fee_rate`
- `PUT /fees` -> `set_fee_rate`

## Example backend integration (Node.js / TypeScript)

This sample uses a generic Soroban client for contract invocation. Adjust the code to your chosen Stellar/Soroban SDK.

### Configuration

```ts
const config = {
  networkUrl: process.env.SOROBAN_NETWORK_URL,
  contractIds: {
    merchantRegistry: process.env.MERCHANT_REGISTRY_CONTRACT_ID,
    paymentRequest: process.env.PAYMENT_REQUEST_CONTRACT_ID,
    feeManager: process.env.FEE_MANAGER_CONTRACT_ID,
  },
  ownerSecret: process.env.CONTRACT_OWNER_SECRET,
};
```

### Contract call helper

```ts
import { SorobanClient, Account, Keypair } from '@stellar/soroban-client';

const client = new SorobanClient(config.networkUrl);
const ownerKeypair = Keypair.fromSecret(config.ownerSecret);

async function invokeContract(contractId: string, functionName: string, args: any[]) {
  const account = await client.getAccount(ownerKeypair.publicKey());
  const transaction = new SorobanClient.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: SorobanClient.Networks.TESTNET,
  })
    .addOperation(
      SorobanClient.Operation.invokeHostFunction({
        function: functionName,
        contractId,
        args,
      })
    )
    .setTimeout(180)
    .build();

  transaction.sign(ownerKeypair);
  const result = await client.submitTransaction(transaction);
  return result;
}
```

> Replace the exact operation and SDK classes with the version used by your chosen Soroban client.

### Merchant registration endpoint

```ts
app.post('/merchants', async (req, res) => {
  const { merchantId, wallet, businessName } = req.body;
  const response = await invokeContract(
    config.contractIds.merchantRegistry,
    'register_merchant',
    [merchantId, wallet, businessName, true]
  );
  res.json({ success: true, response });
});
```

### Create payment request endpoint

```ts
app.post('/payment-requests', async (req, res) => {
  const { requestId, merchant, amount, asset, description, expiry } = req.body;
  const response = await invokeContract(
    config.contractIds.paymentRequest,
    'create_request',
    [requestId, merchant, amount, asset, description, expiry]
  );
  res.json({ success: true, response });
});
```

### Verify payment endpoint

```ts
app.post('/payment-requests/:requestId/verify', async (req, res) => {
  const requestId = req.params.requestId;
  const { txHash, amount, asset } = req.body;
  const response = await invokeContract(
    config.contractIds.paymentRequest,
    'verify_payment',
    [requestId, txHash, amount, asset]
  );
  res.json({ success: true, response });
});
```

### Fee rate endpoint

```ts
app.put('/fees', async (req, res) => {
  const { feeRateBps } = req.body;
  const response = await invokeContract(
    config.contractIds.feeManager,
    'set_fee_rate',
    [feeRateBps]
  );
  res.json({ success: true, response });
});
```

## Payment workflow

1. REST backend receives payment request creation from the merchant dashboard.
2. Backend calls `create_request` on the `payment_request` contract.
3. Backend generates a QR payload with the request ID, merchant wallet, amount, and asset.
4. Customer scans the QR and submits a Stellar payment.
5. Backend monitors Stellar transactions and detects matching payment memo or destination.
6. Backend calls `verify_payment` on the contract to mark the request as paid.

## Contract implementation reference

The contract implementation in this repo is designed to support the REST backend as follows:

- `merchant_registry` stores merchant details and ensures only the contract owner can register or update merchants.
- `payment_request` stores request state and validates payment parameters before marking a request as paid.
- `fee_manager` stores the platform fee rate and provides a reusable fee calculation helper.

## Deployment and contract IDs

The REST backend should be configured with the deployed contract IDs for each crate. Example environment variables:

- `MERCHANT_REGISTRY_CONTRACT_ID`
- `PAYMENT_REQUEST_CONTRACT_ID`
- `FEE_MANAGER_CONTRACT_ID`

These values are assigned once the contracts are deployed to the Soroban network.

## Notes

- This file is intended as a guide for backend implementers connecting a REST API to the contract layer.
- Adjust contract invocation code to your chosen Soroban/SDK version.
- The repository does not include backend REST code, but this file shows the integration pattern.
