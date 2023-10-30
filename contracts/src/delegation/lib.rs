#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod delegation {
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::prelude::borrow::ToOwned;
    use ink::storage::Mapping;    
    use registry_traits::Lifecycle;
    use epoch_traits::Epochs;
    
    #[ink(storage)]
    pub struct Delegation {
        delegates: Mapping<AccountId, BTreeMap<AccountId, (Balance, BlockNumber)>>,
        // Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Delegate {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Undelegate {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Delegation {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(init_value: AccountId) -> Self {
            Self {
                domain_router: init_value,
                delegates: Default::default(),
            }
        }

        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

        fn get_min_threshold(&self) -> BlockNumber {
            BlockNumber::from(10u32)
        }

        #[ink(message, payable, selector = 0xBABEBABE)]
        pub fn delegate(&mut self, name: Hash, from: AccountId) {
            let payment = self.env().transferred_value();
            ink::env::debug_println!("delegate payment: {}", payment);
            if let Some((ref o, _b, _d)) = self.not_expired(name) {
                let epoch = self.get_current_epoch();
                // let bn = self.env().block_number();
                if let Some (mut old_value_map) = self.delegates.take(o.clone()) {
                    old_value_map
                    .entry(from.clone())
                    .and_modify(|ov| {
                        let nd = ov.0.saturating_add(payment.into());
                        *ov = (nd, ov.1)
                        // *ov = ov.saturating_add(payment.into());
                    })
                    .or_insert((payment.into(), epoch.clone()));
                    // .or_insert(payment.into());

                    self.delegates.insert(o.clone(), &old_value_map);

                } else {
                        let mut map = BTreeMap::<AccountId, (Balance, BlockNumber)>::new();
                        map.insert(from, (payment.into(), epoch.clone()));
                        // let _ = map.insert(from, payment.into());
                        self.delegates.insert(o.clone(), &map);
                }

                Self::env().emit_event(
                    Delegate {
                        name,
                        from,
                        value: payment.into(),
                    },
                );

            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn undelegate(&mut self, name: Hash) {
            let caller = self.env().caller();
            let epoch = self.get_current_epoch();
            // let bn = self.env().block_number();
            let min = self.get_min_threshold();
            if let Some((ref o, _b, _d)) = self.not_expired(name) {
                if let Some(a) = self.delegates.get(&o) {
                    if let Some(d) = a.get(&caller) {
                        let p = epoch - d.1;
                        if p < min {
                            ink::env::debug_println!(
                                "min threshold not met: epoch/on/min/p {:?}/{:?}/{:?}/{:?}",
                                epoch,
                                d.1,
                                min,
                                p
                            );
                            panic!("min threshold not met");
                        }
                    } else {
                        ink::env::debug_println!("investor not found");
                        panic!("investor not found");
                    }
                } else {
                    ink::env::debug_println!("delegate not found");
                    panic!("delegate not found");
                }
                let success = {
                    if let Some(a) = self.delegates.get(&o) {
                        if let Some(d) = a.get(&caller) {
                            let value = (*d).0;
                            // let value = *d;
                            match self.env().transfer(caller.clone(), value) {
                                // Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                //     panic!(
                                //         "requested transfer would have brought contract\
                                //         below subsistence threshold!"
                                //     )
                                // }
                                Err(_) => panic!("transfer failed!"),
                                Ok(_) => Some(value),
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                if let Some(value) = success {
                    if let Some(mut a) = self.delegates.take(&o) {
                        if let Some(_) = a.remove(&caller) {
                            Self::env().emit_event(
                                Undelegate {
                                    name,
                                    from: caller.clone(),
                                    value,
                                },
                            );
                        } else {
                            ink::env::debug_println!("failed to remove investor");
                            panic!("failed to remove investor");
                        }
                    } else {
                        ink::env::debug_println!("operator not found");
                        panic!("operator not found");
                    }
                } else {
                    ink::env::debug_println!("transfer failed");
                    panic!("transfer failed");
                }
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn get_delegate(
            &self,
            investor: AccountId,
            name: Hash,
        ) -> Option<(Balance, BlockNumber)> {
            if let Some((ref o, _b, _d)) = self.not_expired(name) {
                if let Some(a) = self.delegates.get(&o) {
                    a.get(&investor).cloned()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    impl Lifecycle for Delegation {
        /// Simply returns not expired test
        #[ink(message)]
        fn not_expired(&self, hash_name: Hash) -> Option<(AccountId, BlockNumber, u32)> {
            <ink::contract_ref!(Lifecycle)>::from(
                self.router_resolve_contract("registry".to_owned())
                    .expect("proper lifecycle resolution"),
            )
            .not_expired(hash_name)
        }

        /// Simply returns expired test
        #[ink(message)]
        fn expired(&self, hash_name: Hash) -> Option<(AccountId, BlockNumber, u32)> {
            <ink::contract_ref!(Lifecycle)>::from(
                self.router_resolve_contract("registry".to_owned())
                    .expect("proper lifecycle resolution"),
            )
            .expired(hash_name)
        }
    }

    impl Epochs for Delegation {
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


}
