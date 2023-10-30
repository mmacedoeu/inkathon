#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::epoch::Epoch;

#[ink::contract]
mod epoch {
    use epoch_traits::{Epochs, Offset, Period};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Epoch {
        offset: BlockNumber,
        period: BlockNumber,
    }

    impl Epoch {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(offset: BlockNumber, period: BlockNumber) -> Self {
            Self { offset, period }
        }
    }

    impl Epochs for Epoch {
        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn get_current_epoch(&self) -> u32 {
            let off = self.env().block_number() - self.offset;
            off / self.period
        }

        /// Simply returns the value of epoch since param.
        #[ink(message)]
        fn get_current_epoch_since(&self, since: BlockNumber) -> u32 {
            let s = (since - self.offset) / self.period;
            let off = (self.env().block_number() - self.offset) / self.period;
            off - s
        }

        /// Simply returns the current value of block inside epoch.
        #[ink(message)]
        fn get_current_block(&self) -> u32 {
            let off = self.env().block_number() - self.offset;
            off % self.period
        }
    }

    impl Offset for Epoch {
        /// set the offset from genesis where period begin.
        #[ink(message)]
        fn set_offset(&mut self, offset: BlockNumber) {
            self.offset = offset;
        }

        /// Simply returns the current value of offset.
        #[ink(message)]
        fn get_offset(&self) -> BlockNumber {
            self.offset
        }
    }

    impl Period for Epoch {
        /// set period for each epoch.
        #[ink(message)]
        fn set_period(&mut self, period: BlockNumber) {
            self.period = period;
        }

        /// Simply returns the current length of period.
        #[ink(message)]
        fn get_period_length(&self) -> BlockNumber {
            self.period
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let epoch = Epoch::default();
            assert_eq!(epoch.get_offset(), BlockNumber::from(0u32));
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let epoch = Epoch::new(BlockNumber::from(0u32), BlockNumber::from(10u32));
            assert_eq!(epoch.get_offset(), BlockNumber::from(0u32));
            assert_eq!(epoch.get_period_length(), BlockNumber::from(10u32));
        }
    }
}
