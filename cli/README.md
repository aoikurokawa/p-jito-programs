# Jito Tip CLI

## Initialize Config

## Config

### Get

```bash
cargo r --bin jito-tip-cli -- tip-distribution \
    config \
    get \
    --keypair-path ~/.config/solana/id.json
```

## TipDistributionAccount

### Initialize

```bash
cargo r --bin jito-tip-cli -- tip-distribution \
    tip-distribution-account \
    initialize \
    --vote-account 8QyvcGJuZ55HjhqwR3uSqsyziww41hDV4osDEGMER2tc \
    --merkle-root-upload-authority 8QyvcGJuZ55HjhqwR3uSqsyziww41hDV4osDEGMER2tc \
    --validator-commission-bps 10 \
    --keypair-path ~/.config/solana/id.json
```

### Upload Merkle Root

```bash
 cargo r --bin jito-tip-cli -- tip-distribution \
    tip-distribution-account \
    upload-merkle-root \
    --vote-account 8QyvcGJuZ55HjhqwR3uSqsyziww41hDV4osDEGMER2tc \
    --root '1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32' \
    --max-total-claim 10 \
    --max-num-nodes 10 \
    --keypair-path ~/.config/solana/id.json
```

### Get

```bash
cargo r --bin jito-tip-cli -- tip-distribution \
    tip-distribution-account \
    get \
    --vote-account 8QyvcGJuZ55HjhqwR3uSqsyziww41hDV4osDEGMER2tc \
    --epoch 944 \
    --keypair-path ~/.config/solana/id.json
```

### List

```bash
cargo r --bin jito-tip-cli -- tip-distribution \
    tip-distribution-account \
    list \
    --keypair-path ~/.config/solana/id.json
```

