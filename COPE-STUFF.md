Some bits that are important to build the code to exploit the roulette

```typescript
export const rouletteInstruction = async (
  rngAccountKey: StringPublicKey,
  honeypotAccount: StringPublicKey,
  vaultAccount: StringPublicKey,
  tokenAccount: StringPublicKey,
  mintAccount: StringPublicKey,
  pythProductKey1: StringPublicKey,
  pythPriceKey1: StringPublicKey,
  pythProductKey2: StringPublicKey,
  pythPriceKey2: StringPublicKey,
  pythProductKey3: StringPublicKey,
  pythPriceKey3: StringPublicKey,
  wallet: any,
  bets: RouletteBet[],
) => {
  if (!wallet.publicKey) throw new WalletNotConnectedError();
  let settings = new RouletteArgs({ tolerance: new BN(10), bets });
  const data = Buffer.from(serialize(schema, settings));
  return {
    ix: [
      new TransactionInstruction({
        keys: [
          {
            pubkey: toPublicKey(rngAccountKey),
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: toPublicKey(wallet.publicKey),
            isSigner: true,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(tokenAccount),
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: toPublicKey(mintAccount),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(honeypotAccount),
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: toPublicKey(vaultAccount),
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythProductKey1),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythPriceKey1),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythProductKey2),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythPriceKey2),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythProductKey3),
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: toPublicKey(pythPriceKey3),
            isSigner: false,
            isWritable: false,
          },
        ],
        programId: toPublicKey(RNG_PROGRAM_ID),
        data,
      }),
    ],
  };
};
```

Some constant

```typescript
export const MAINNET_SOL_PRODUCT_ORACLE = new PublicKey("ALP8SdU9oARYVLgLR7LrqMNCYBnhtnQz1cj6bwgwQmgj");
export const MAINNET_SOL_PRICE_ORACLE = new PublicKey("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");
export const MAINNET_BTC_PRODUCT_ORACLE = new PublicKey("4aDoSXJ5o3AuvL7QFeR6h44jALQfTmUUCTVGDD6aoJTM");
export const MAINNET_BTC_PRICE_ORACLE = new PublicKey("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU");
export const MAINNET_ETH_PRODUCT_ORACLE = new PublicKey("EMkxjGC1CQ7JLiutDbfYb7UKb3zm9SJcUmr1YicBsdpZ");
export const MAINNET_ETH_PRICE_ORACLE = new PublicKey("JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB");
export const MAINNET_MINT = new PublicKey("8HGyAAB1yoM1ttS7pXjHMa3dukTFGQggnFFH3hJZgzQh"); // COPE
```