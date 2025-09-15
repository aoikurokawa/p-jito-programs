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

