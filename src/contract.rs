use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::env::AccountId;
use crate::events::{Transfer, Mint, Burn};
use crate::access::AccessControl;

#[ink::contract]
pub mod contract {
    use super::*;

    #[ink(storage)]
    pub struct MyToken {
        balances: Mapping<AccountId, u64>,
        total_supply: u64,
        max_supply: u64,
        owner: AccountId,
    }

    impl MyToken {
        #[ink(constructor)]
        pub fn new(initial_supply: u64, max_supply: u64) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                balances: Mapping::default(),
                total_supply: initial_supply,
                max_supply,
                owner: caller,
            };
            instance.balances.insert(&caller, &initial_supply);
            Self::env().emit_event(Mint {
                to: caller,
                value: initial_supply,
            });
            instance
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, amount: u64) -> Result<(), &'static str> {
            self.only_owner()?;
            if self.total_supply + amount > self.max_supply {
                return Err("Cannot mint beyond max supply");
            }
            let current_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(&to, &(current_balance + amount));
            self.total_supply += amount;
            Self::env().emit_event(Mint { to, value: amount });
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: u64) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let current_balance = self.balances.get(&caller).unwrap_or_default();
            if current_balance < amount {
                return Err("Insufficient balance to burn");
            }
            self.balances.insert(&caller, &(current_balance - amount));
            self.total_supply -= amount;
            Self::env().emit_event(Burn {
                from: caller,
                value: amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u64) -> Result<(), &'static str> {
            let from = self.env().caller();
            let from_balance = self.balances.get(&from).unwrap_or_default();

            if from_balance < amount {
                return Err("Insufficient balance");
            }

            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(&from, &(from_balance - amount));
            self.balances.insert(&to, &(to_balance + amount));
            Self::env().emit_event(Transfer { from, to, value: amount });
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u64 {
            self.total_supply
        }

        #[ink(message)]
        pub fn max_supply(&self) -> u64 {
            self.max_supply
        }

        fn only_owner(&self) -> Result<(), &'static str> {
            AccessControl::only_owner(self.env().caller(), self.owner)
        }
    }
}
