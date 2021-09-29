# COPE roulette pro

This is a pro roulette player, which always win at the COPE roulette.

It's efficiency has been demonstrated through 207 plays, 207 wins.

https://twitter.com/PierreArowana/status/1441552108734476290

# How?

## Chapter I: Discovery

A friend sent me the COPE tweet about randomness https://twitter.com/cyrii_MM/status/1440394029154246663

My curiousity had been aroused, how so? I opened the front end to discover more. After betting one COPE, it was clear that everything was done in one transaction.

This is the first terminal red flag /!\\, if the outcome can be determined in the same transaction the user submits to play the roulette, then it is public and can be tricked.

So I headed to the front end code in chrome dev tools, thanks to COPE, the typescript sourcemap is present, making all this less effort.

https://6af3wmtg7qmmkixf5iwofrbfxj2usyxqdzh4xpkx2xkquz5ltdfa.arweave.net/8Au7Mmb8GMUi5eos4sQlunVJYvAeT8u9V9XVCmermMo/static/js/actions/instructions.ts

The instruction of interest is `rouletteInstruction`, which is the "play the roulette" one. https://solscan.io/tx/2zZd6xTgUwnHYCajhj1yUnGkqEYY4t9wuX8RW6kxgjbW5FTGCLcbqvQXFGzfgzR62rZezLavLvnwKrtmHMe8LXdf this is the transaction from my first click in the front end.

It takes the blockhash, a few pyth network price feeds (empirically random as demonstrated), the user wallet and token account, as well as a few other accounts.

It isn't very important how the number is drawn. We can read the transaction logs to see roughly what are the steps

```
Program rouQqKK4CKYgozmG8fuLTaAt7Crngw3dxsGnrWteuno invoke [1]
Program log: Instruction 4: Roulette
...
Program log: Roulette Outcome 4
Program log: Bet Enum: 0, size: 1
Program log: Reward 0
...
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: Transfer
```

## Chapter II: Roulette pro bootcamp

To become a pro roulette player you need to make one with it, no martingale involved this time.

With the help of the `rouletteInstruction` typescript code, the constants and the test transaction above, we can rebuild in rust. The code is commented to understand that part. Once we can play, we need to make sure we win. Let's assume we always play "Even" with the max amount, 100 COPE.

When the player wins, his token account gets credited 100 COPE, when he loses it gets debited 100 COPE.

We need an instruction after `rouletteInstruction` that will fail when the user token account is debited but succeed when it is credited because if an instruction fails, the entire transaction is reverted (no account is mutated).

Another requirement is to make it "inputless", we want to send a bundle of transactions to be done with it in a short amount of time.

The naive solution is to do a self transfer with the original amount of token, which would fail if player loses. But that requires waiting for the new token account state every single time we want to make a new transaction because the player balance should increase at every win. Or writing a program read the original state, play the roulette and compares the token account state, that is way too much work!

Instead, we keep the minimum COPE in the player token account adding an instruction to transfer 100 COPE to a "reserve token account" the player owns:

- Player wins => transfer successfully 100 COPE to reserve token account, Player still has exactly 100 COPE
- Player loses => tries to send 100 COPE to reserve token account, but fails since there isn't 100 COPE available after losing, reverting the transaction.

That's it, we can send that many times over, and wait for the treasury to be depleted.

There was ~20,700 COPE in the treasury, so it took ~207 successful transactions to drain it.

## Chapter III: Conclusion

An important step of the development cycle of a smart contract is to take some time to look for ways to break your own code, or get a teammate to do that. If there is any way to do so, head back to the design phase. This one was not a sneaky bug.
