# Jito Programs by Pinocchio

## Overview

Reimplement Jito Programs by Pinocchio

## Features

## Compute Units

### Tip Payment Program

| Instruction          | CU (p-jito-programs) | CU (jito-programs) |
| -------------------- | -------------------- | ------------------ |
| `Initialize`         | 37232                |                    |
| `ChangeTipReceiver`  | 904                  | 30055              |
| `ChangeBlockBuilder` | 707                  | 28654              |

### Tip Distribution Program

| Instruction                           | CU (p-jito-programs) | CU (jito-programs) |
| ------------------------------------- | -------------------- | ------------------ |
| `Initialize`                          |                      |                    |
| `InitializeMerkleRootUploadConfig`    |                      |               |
| `InitializeTipDistributionAccount`    |                      |               |
| `UpdateConfig` |                      |               |
| `UploadMerkleRoot`                    |                      |               |
| `CloseClaimStatus`                    |                      |               |
| `CloseTipDistributionAccount`         |                      |               |
| `Claim`                               |                      |               |
| `InitializeMerkleRootUploadConfig`    |                      |               |
| `UpdateMerkleRootUploadConfig`        |                      |               |
| `MigrateTdaMerkleRootUploadAuthority` |                      |               |

## Instructions

### Initialize

Discriminator: [175, 175, 109, 31, 13, 152, 155, 237]

### ChangeTipReceiver

Discriminator: [69, 99, 22, 71, 11, 231, 86, 143]

### ChangeBlockBuilder

Discriminator: [134, 80, 38, 137, 165, 21, 114, 123]

## Accounts

### Config

Discriminator: [155, 12, 170, 224, 30, 250, 204, 130]

### TipPaymentAccount

Discriminator: [201, 33, 244, 116, 224, 68, 97, 40]

## References
- https://github.com/jito-foundation/jito-programs/blob/master/mev-programs/programs/tip-payment/Cargo.toml
- https://www.helius.dev/blog/pinocchio
