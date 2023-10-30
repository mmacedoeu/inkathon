#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! The trait is extracted into a separate crate to show how to do cross-contract
//! calls only with traits without importing the contract.

use ink::env::{DefaultEnvironment, Environment};

type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
type Balance = <DefaultEnvironment as Environment>::Balance;
type AccountId = <DefaultEnvironment as Environment>::AccountId;
type Hash = <DefaultEnvironment as Environment>::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
)]

pub struct SubscriberData {
    pub balance: Balance,
    pub on: BlockNumber,
}

/// manage data for subscription.
#[ink::trait_definition]
pub trait Subscriptions {
    /// Subscribe to topic Hash.
    #[ink(message, payable)]
    fn subscribe(&mut self, name: Hash, from: AccountId);

    /// Unsubscribe to topic Hash.
    #[ink(message)]
    fn unsubscribe(&mut self, name: Hash);

    /// Simply returns the SubscriberData for given topic Hash.    
    #[ink(message)]
    fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData>;

    /// if any transfer any received fees for given topic Hash.
    #[ink(message)]
    fn claim_fees(&mut self, name: Hash);    
}