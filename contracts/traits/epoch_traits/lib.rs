#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! The trait is extracted into a separate crate to show how to do cross-contract
//! calls only with traits without importing the contract.

use ink::env::{DefaultEnvironment, Environment};

type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

/// get stats around current Epoch.
#[ink::trait_definition]
pub trait Epochs {
    /// Simply returns the current value of epoch.
    #[ink(message)]
    fn get_current_epoch(&self) -> u32;

    /// Simply returns the value of epoch since param.
    #[ink(message)]
    fn get_current_epoch_since(&self, since: BlockNumber) -> u32;

    /// Simply returns the current value of block inside epoch.    
    #[ink(message)]
    fn get_current_block(&self) -> u32;
}

/// Offset around current Epoch.
#[ink::trait_definition]
pub trait Offset {
    /// set the offset from genesis where period begin.
    #[ink(message)]
    fn set_offset(&mut self, offset: BlockNumber);

    /// Simply returns the current value of offset.
    #[ink(message)]
    fn get_offset(&self) -> BlockNumber;
}

/// Period around current Epoch.
#[ink::trait_definition]
pub trait Period {
    /// set period for each epoch.
    #[ink(message)]
    fn set_period(&mut self, period: BlockNumber);

    /// Simply returns the current length of period.
    #[ink(message)]
    fn get_period_length(&self) -> BlockNumber;
}
