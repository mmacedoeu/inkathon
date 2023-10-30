#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! The trait is extracted into a separate crate to show how to do cross-contract
//! calls only with traits without importing the contract.

use ink::env::{DefaultEnvironment, Environment};

type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
type Balance = <DefaultEnvironment as Environment>::Balance;
type AccountId = <DefaultEnvironment as Environment>::AccountId;
type Hash = <DefaultEnvironment as Environment>::Hash;

/// get stats around current Economics.
#[ink::trait_definition]
pub trait Economics {
    /// Simply returns the current rate.
    #[ink(message)]
    fn get_current_rate(&self) -> Balance;

}

/// get stats around current Ownership.
#[ink::trait_definition]
pub trait Ownership {
    /// Simply returns is owner for param "from".
    #[ink(message)]
    fn is_owner_from(&self, name: Hash, from: AccountId) -> bool;

    /// Simply returns is owner    
    #[ink(message)]
    fn is_owner(&self, name: Hash) -> bool;

}

#[ink::trait_definition]
pub trait Lifecycle {
    /// Simply returns not expired test
    #[ink(message)]
    fn not_expired(&self, hash_name: Hash) -> Option<(AccountId, BlockNumber, u32)>;

    /// Simply returns expired test
    #[ink(message)]
    fn expired(&self, hash_name: Hash) -> Option<(AccountId, BlockNumber, u32)>;
}