#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::registry::Registry;

#[ink::contract]
mod registry {
    use ink::env::hash::Blake2x256;
    // use ink_prelude::collections::BTreeMap;
    use epoch_traits::Epochs;
    use registry_traits::{Economics, Lifecycle, Ownership};

    use ink::prelude::borrow::ToOwned;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::Encode;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Registry {
        registry: Mapping<Hash, (AccountId, BlockNumber, u32)>,
        commit_name: Mapping<Hash, Hash>,
        commit: Mapping<Hash, BlockNumber>,
        // Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    /// Emitted whenever a new name is being registered.
    #[ink(event)]
    pub struct Register {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
    }

    /// Emitted whenever a new name is being registered.
    #[ink(event)]
    pub struct Unregister {
        #[ink(topic)]
        name: Hash,
    }

    impl Registry {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(init_value: AccountId) -> Self {
            Self {
                registry: Default::default(),
                commit_name: Default::default(),
                commit: Default::default(),
                domain_router: init_value,
            }
        }

        /// Simply returns the current Hash value of our `name`.
        #[ink(message)]
        pub fn get_hash(&self, name: String) -> Hash {
            Hash::from(self.env().hash_bytes::<Blake2x256>(name.as_bytes()))
        }

        #[ink(message)]
        pub fn valid(&self, name: String) -> bool {
            let l = name.as_bytes().len();
            ink::env::debug_println!("name size {} is {}", name, l);
            l <= 256 && l >= 2
        }

        #[ink(message)]
        pub fn available(&self, name: String) -> bool {
            if self.valid(name.clone()) {
                let h = self.get_hash(name);
                if let Some((_, ref b, ref d)) = self.registry.get(&h) {
                    let epoch = self.get_current_epoch();
                    let t = epoch - *b;
                    ink::env::debug_println!("available: {} == {}", t, *d);
                    *d < t
                } else {
                    true
                }
            } else {
                false
            }
        }

        #[ink(message, selector = 0x1EECBEEF)]
        pub fn make_commitment(&self, name: String, owner: AccountId, secret: u32) -> Hash {
            let mut out = [0; 32];
            let mut pimage: Vec<u8> = Vec::new();
            pimage.extend_from_slice(name.as_bytes());
            let enc_owner = owner.encode();
            pimage.extend_from_slice(&enc_owner[..]);
            pimage.extend_from_slice(&secret.to_be_bytes());
            ink::env::debug_println!("preimage: {:?}", pimage);
            ink::env::hash_bytes::<Blake2x256>(&pimage, &mut out);
            ink::env::debug_println!("commitment: {:?}", out);
            Hash::from(out)
        }

        fn not_expired_commit(&self, b: &BlockNumber) -> bool {
            let t = self.env().block_number() - *b;
            ink::env::debug_println!("not_expired_commit: {}", t);
            t <= 100
        }

        fn commit_check(&self, commitment: &Hash) {
            if let Some(ref b) = self.commit.get(commitment) {
                let valid = self.not_expired_commit(b);
                ink::env::debug_println!("commit still valid: {}", valid);
                assert!(valid, "commit still valid");
            }
            if let Some(name_hash) = self.commit_name.get(commitment) {
                let expired = self.expired(name_hash).is_some();
                ink::env::debug_println!("name available: {}", expired);
                assert!(expired, "name not available");
            }
        }

        fn commit_register(&self, commitment: &Hash) {
            if let Some(ref b) = self.commit.get(commitment) {
                let not_expired_commit = self.not_expired_commit(b);
                ink::env::debug_println!("commit not expired: {}", not_expired_commit);
                assert!(not_expired_commit, "commit expired");
            } else {
                ink::env::debug_println!("no commit");
                panic!("no commit")
            }
        }

        #[ink(message, payable, selector = 0xDEADBEEF)]
        pub fn commit(&mut self, commitment: Hash) {
            ink::env::debug_println!(
                "received commit payment: {}",
                self.env().transferred_value()
            );
            assert!(
                self.env().transferred_value() >= 10,
                "minimum payment is ten"
            );
            self.commit_check(&commitment);
            self.commit.insert(commitment, &self.env().block_number());
            ink::env::debug_println!("committed");
        }

        fn unlock_balance(&mut self, name_hash: Hash) {
            if let Some((ref a, _b, _d)) = self.registry.get(&name_hash) {
                match self.env().transfer(*a, 10) {
                    // Err(ink::env::Error::BelowSubsistenceThreshold) => {
                    //     panic!(
                    //         "requested transfer would have brought contract\
                    //         below subsistence threshold!"
                    //     )
                    // }
                    Err(_) => panic!("transfer failed!"),
                    Ok(_) => {}
                }
            }
            let _ = self.registry.take(&name_hash);
            // let mut cn_values = self.commit_name.iter();
            // let entry = cn_values.next();
            // let mut nh = Hash::default();
            // while entry.is_some() {
            //     if let Some(ref inner) = entry {
            //         if *inner.1 == name_hash {
            //             nh = *inner.0;
            //             break;
            //         }
            //     }
            // }
            let _ = self.commit_name.take(&name_hash);
        }

        #[ink(message)]
        pub fn rent_price(&self, name: String, duration: u32) -> Balance {
            let rp = name.as_bytes().len() as u32 * duration * 1_000;
            ink::env::debug_println!("rent_price {} for {} is {}", name, duration, rp);
            Balance::from(rp)
        }

        // #[ink(message)]
        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

        #[ink(message, payable, selector = 0xCAFEBABE)]
        pub fn register(&mut self, name: String, from: AccountId, duration: u32, secret: u32) {
            let p = self.env().transferred_value();
            ink::env::debug_println!("register payment: {}", p);
            let r = self.rent_price(name.clone(), duration);
            ink::env::debug_println!("rent price: {}", r);
            assert!(
                p >= r + 10,
                "payment was not enough for rent plus 10 (locked balance)"
            );
            let commitment = self.make_commitment(name.clone(), from, secret);
            ink::env::debug_println!("commitment: {:?}", commitment);
            self.commit_register(&commitment);
            ink::env::debug_println!("name: {:?}", name);
            let available = self.available(name.clone());
            ink::env::debug_println!("available: {:?}", available);
            assert!(available, "not available");
            let name_hash = self.get_hash(name);
            ink::env::debug_println!("name_hash: {:?}", name_hash);
            if let Some((_a, _b, _d)) = self.expired(name_hash) {
                self.unregister_unchecked(name_hash);
            }
            let epoch = self.get_current_epoch();
            self.registry.insert(name_hash, &(from, epoch, duration));
            self.commit_name.insert(commitment, &name_hash);
            self.commit.take(&commitment);
            self.env().emit_event(Register {
                name: name_hash,
                from,
            });
        }

        fn unregister_unchecked(&mut self, name: Hash) {
            ink::env::debug_println!("unregister payment: {}", self.env().transferred_value());
            self.unlock_balance(name);
            self.env().emit_event(Unregister { name });
        }

        #[ink(message, selector = 0xDEADBABE)]
        pub fn unregister(&mut self, name: Hash) {
            if self.is_owner(name) {
                self.unregister_unchecked(name);
            } else {
                ink::env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }
    }

    impl Epochs for Registry {
        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn get_current_epoch(&self) -> u32 {
            <ink::contract_ref!(Epochs)>::from(
                self.router_resolve_contract("epoch".to_owned())
                    .expect("proper epoch resolution"),
            )
            .get_current_epoch()
        }

        /// Simply returns the value of epoch since param.
        #[ink(message)]
        fn get_current_epoch_since(&self, since: BlockNumber) -> u32 {
            <ink::contract_ref!(Epochs)>::from(
                self.router_resolve_contract("epoch".to_owned())
                    .expect("proper epoch resolution"),
            )
            .get_current_epoch_since(since)
        }

        /// Simply returns the current block.
        #[ink(message)]
        fn get_current_block(&self) -> u32 {
            <ink::contract_ref!(Epochs)>::from(
                self.router_resolve_contract("epoch".to_owned())
                    .expect("proper epoch resolution"),
            )
            .get_current_block()
        }
    }

    impl Economics for Registry {
        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn get_current_rate(&self) -> Balance {
            Balance::from(10u32)
        }
    }

    impl Ownership for Registry {
        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn is_owner_from(&self, name: Hash, from: AccountId) -> bool {
            if let Some((ref o, _b, _d)) = self.registry.get(&name) {
                from == *o
            } else {
                false
            }
        }

        #[ink(message)]
        fn is_owner(&self, name: Hash) -> bool {
            if let Some((ref o, _b, _d)) = self.registry.get(&name) {
                self.env().caller() == *o
            } else {
                false
            }
        }
    }

    impl Lifecycle for Registry {
        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn not_expired(&self, name: Hash) -> Option<(AccountId, BlockNumber, u32)> {
            if let Some((ref a, ref b, ref d)) = self.registry.get(&name) {
                let epoch = self.get_current_epoch();
                let t = epoch - *b;
                ink::env::debug_println!("not_expired: {} == {}", t, *d);
                if t <= *d {
                    Some((a.clone(), b.clone(), d.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }

        /// Simply returns the current value of epoch.
        #[ink(message)]
        fn expired(&self, name: Hash) -> Option<(AccountId, BlockNumber, u32)> {
            if let Some((ref a, ref b, ref d)) = self.registry.get(&name) {
                let epoch = self.get_current_epoch();
                let t = epoch - *b;
                ink::env::debug_println!("expired: {} == {}", t, *d);
                if t > *d {
                    Some((a.clone(), b.clone(), d.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    // / Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // / module and test functions are marked with a `#[test]` attribute.
    // / The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let _ = Indexer::default();
    //         assert!(true);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn register_works() {
    //         // given
    //         let contract_balance = 100;
    //         let accounts = default_accounts();
    //         set_sender(accounts.alice);
    //         set_balance(contract_id(), contract_balance);
    //         let mut myns = Indexer::default();

    //         // when
    //         set_sender(accounts.eve);
    //         set_balance(accounts.eve, 100);

    //         assert!(myns.available("myname".to_owned()));
    //         let commitment = myns.make_commitment("myname".to_owned(), accounts.eve, 1);
    //         commit(&accounts.eve, &commitment);
    //         myns.commit(commitment);
    //         assert!(myns.available("myname".to_owned()));
    //         register("myname".to_owned(), &accounts.eve, 100, 1);
    //         myns.register("myname".to_owned(), accounts.eve, 100, 1);
    //         assert!(!myns.available("myname".to_owned()));
    //     }

    //     fn set_sender(sender: AccountId) {
    //         let callee =
    //             ink::env::account_id::<ink::env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
    //         test::push_execution_context::<Environment>(
    //             sender,
    //             callee,
    //             1000000,
    //             1000000,
    //             test::CallData::new(call::Selector::new([0x00; 4])), // dummy
    //         );
    //     }

    //     fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
    //         ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
    //             .expect("Off-chain environment should have been initialized already")
    //     }

    //     fn set_balance(account_id: AccountId, balance: Balance) {
    //         ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
    //             account_id, balance,
    //         )
    //         .expect("Cannot set account balance");
    //     }

    //     fn commit(account: &AccountId, commitment: &Hash) {
    //         // 0xDEADBEEF
    //         let mut data = ink::env::test::CallData::new(ink::env::call::Selector::new([
    //             0xDE, 0xAD, 0xBE, 0xEF,
    //         ]));
    //         data.push_arg(commitment);
    //         let mock_transferred_value = 100;

    //         // Push the new execution context which sets Eve as caller and
    //         // the `mock_transferred_value` as the value which the contract
    //         // will see as transferred to it.
    //         ink::env::test::push_execution_context::<ink::env::DefaultEnvironment>(
    //             *account,
    //             contract_id(),
    //             1000000,
    //             mock_transferred_value,
    //             data,
    //         );
    //     }

    //     fn register(name: String, from: &AccountId, duration: u32, secret: u32) {
    //         // 0xCAFEBABE
    //         let mut data = ink::env::test::CallData::new(ink::env::call::Selector::new([
    //             0xCA, 0xFE, 0xBA, 0xBE,
    //         ]));
    //         data.push_arg(&name);
    //         data.push_arg(from);
    //         data.push_arg(&duration);
    //         data.push_arg(&secret);
    //         let mock_transferred_value = 600010;

    //         // Push the new execution context which sets 'from' as caller and
    //         // the `mock_transferred_value` as the value which the contract
    //         // will see as transferred to it.
    //         ink::env::test::push_execution_context::<ink::env::DefaultEnvironment>(
    //             *from,
    //             contract_id(),
    //             1000000,
    //             mock_transferred_value,
    //             data,
    //         );
    //     }
    // }
}
