#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This contract demonstrates a safe approach to handling randomness in smart contracts using an external oracle.
/// It simulates a simple casino-like game where users can place bets and potentially win rewards.
/// 
/// The provided code is for educational purposes only and should not be used in production.
///
/// Key Features:
///
/// - Integration with a Randomness Oracle: Relies on an external oracle (DIA Oracle in this example) to provide
///   unpredictable randomness, ensuring fairness.
/// - Two-Phase Betting:
///   - register_bet: User registers a bet and pays a fee.
///   - resolve_bet: User resolves the bet based on the oracle's randomness to determine win/loss and receive rewards.
///
/// Contract Structure:
///
/// Storage:
/// bets: Mapping that stores bet details (bet id, round, user).
/// oracle: Contract reference to the DIA Oracle.
///
/// Functionality:
///
/// - get_random(key: u64) -> Option<Vec<u8>>: Fetches randomness for a given round from the oracle.
/// - register_bet() -> Result<(), Error>: Registers a new bet, assigning a future round number, and charging a fee.
/// - resolve_bet(bet_id: BetId) -> Result<(), Error>: Resolves a bet based on the oracle's randomness,
///   distributing rewards if applicable.
///
/// - get_id() -> BetId: Generates a unique bet identifier.
/// - pay_fee(user: User) -> Result<(), Error>: Handles fee payment.
/// - is_victorious(randomness: Vec<u8>) -> bool: Determines if the bet is a win based on randomness.
/// - pay_reward(user: User) -> Result<(), Error>: Pays out rewards to the user.

#[ink::contract]
mod casino {
    use dia_oracle_randomness_getter::RandomOracleGetter;
    use ink::contract_ref;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(Eq, PartialEq, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        FailedTransfer,
        BetResolutionTooEarly,
        Custom(Vec<u8>),
    }

    /// Example helper types
    pub type BetId = u128;
    pub type Round = u64;
    pub type User = AccountId;
    pub type BetDetails = (Round, User);

    #[ink(storage)]
    pub struct Casino {
        bets: Mapping<BetId, BetDetails>,
        // reference to the oracle contract
        oracle: contract_ref!(RandomOracleGetter),
    }

    impl Casino {
        #[ink(constructor)]
        pub fn new(oracle_address: AccountId) -> Self {
            Self {
                bets: Mapping::default(),
                oracle: oracle_address.into(),
            }
        }

        /// Gets a random value from an oracle.
        #[ink(message)]
        pub fn get_random(&self, key: u64) -> Option<Vec<u8>> {
            self.oracle.get_random_value_for_round(key)
        }

        /// A user will call the message to place a bet
        #[ink(message)]
        pub fn register_bet(&mut self) -> Result<(), Error> {
            let user = self.env().caller();
            // The player pays the fee to the Casino for playing.
            self.pay_fee(user)?;

            let bet_id = self.get_id(); // get a fresh BetId
            let current_round = self.oracle.get_latest_round();
            let round = current_round + 2; // we need to make sure this is a round in the future;
            let details = (round, user);
            self.bets.insert(bet_id, &details);

            Ok(())
        }

        /// Depending on the randomness, provided by the oracle, determines if a user is victorious and pays 
        /// them up in that case. 
        /// 
        /// The user needs to wait a couple of blocks after registering a bet before calling this message.
        #[ink(message)]
        pub fn resolve_bet(&mut self, bet_id: BetId) -> Result<(), Error> {
            let user = self.env().caller();
            let round = self.bets.get(bet_id).unwrap().0;
            let randomness = self.oracle.get_random_value_for_round(round);
            // Based on `randomness` determine if the bet was won or lost. Pay out rewards to the user, etc.
            match randomness {
                Some(randomness) => {
                    if self.is_victorious(randomness) {
                        self.pay_reward(user)?;
                    }
                },
                None => {
                    // After registering bet, user would need to wait a couple of blocks for randomness
                    return Err(Error::BetResolutionTooEarly);
                }
            }

            self.bets.remove(bet_id);

            Ok(())
        }

        fn get_id(&self) -> BetId {
            // implement id generation
            42
        }

        fn pay_fee(&self, user: User) -> Result<(), Error> {
            // implement fee payment
            Ok(())
        }

        fn is_victorious(&self, randomness: Vec<u8>) -> bool {
            // implement victory logics
            true
        }

        fn pay_reward(&self, user: User) -> Result<(), Error> {
            //implement reward payment
            Ok(())
        }
    }
}
