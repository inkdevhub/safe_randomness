# Safe Randomness in Smart Contracts: A Tutorial

This tutorial aims to clarify the importance of secure randomness generation in smart contracts. We demonstrate why the commonly used approach is unsafe and introduce a safer alternative using an oracle.

## Motivation

### Unsafe Pattern:

Problem: Consider a simple Casino contract with a coin_toss function (implementation details are irrelevant). The play function allows users to play by paying a fee. The outcome of the coin_toss determines whether the user wins or loses.
```Rust
fn coin_toss(&mut self) -> bool {
	//Some logic to implement randomness
}

#[ink(message)]
pub fn play(&mut self) {
	// The player pays the fee to the Casino for playing.

    let toss = self.coin_toss();
    if toss {
    	// player wins -- they get some token
    } else {
    	// player loses
    }
}
```

### Why is coin_toss unsafe?

The issue lies in attempting to generate random numbers within the contract itself. Common methods like using blockhash, timestamp, or internal state are all ultimately predictable and manipulable. This predictability allows users to exploit the system as explained earlier.

While users pay the fee for playing, they can exploit this pattern. Instead of directly playing the game, a malicious user can create a contract that checks the outcome of play without actually playing. If they lose, the transaction is reverted, costing only gas fees. If they win, they keep the winnings. This essentially allows the user to gamble risk-free, exploiting the predictable nature of coin_toss.

```Rust
#[ink(message)]
pub fn play_to_win(&mut self) {
	// call the `play` message from the `Casino` contract
	let won = // code that checks if there was a win, for instance checking if balance of a particular token increased
	if !won {
		panic!("Hehe");
	}
}
```

## Safe Randomness with Oracles

The solution involves utilizing a randomness oracle, a dedicated external service that provides unpredictable and verifiable randomness on-chain. The oracle would push some piece of safe randomness on chain via a smart contract. 
For example, the [DIA oracle](https://github.com/diadata-org/dia-oracle-anchor/blob/main/example-randomness-oracle/example.rs)

Take a look at the [example implementation.](./lib.rs)

First off, a user registers a bet with `register_bet()` message. It would take a bet fee from a user and save the bet data to the storage.

The message fetches the randomness for that specific round from the oracle:
```Rust
#[ink(message)]
pub fn get_random(&self, key: u64) -> Option<Vec<u8>> {
    self.oracle.get_random_value_for_round(key)
}
```
Based on the retrieved randomness, the win/loss is determined, rewards are distributed, and the bet is removed by `resolve_bet()` message after waiting for a couple of blocks.

This approach requires two transactions from the user: register_bet and later resolve_bet. While less user-friendly, it ensures fairness and prevents users from exploiting the system.

## Conclusion

Utilizing a randomness oracle is essential for generating secure and unpredictable randomness within smart contracts. This tutorial sheds light on the pitfalls of the unsafe pattern and demonstrates a safer alternative that leverages the power of oracles. Remember, always choose well-established and secure oracles for your smart contract applications.