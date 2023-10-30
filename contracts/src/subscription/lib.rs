#![cfg_attr(not(feature = "std"), no_std, no_main)]

// pub use self::subscription::SubscriberData;

#[ink::contract]
mod subscription {
    use epoch_traits::Epochs;
    use registry_traits::{Economics, Lifecycle, Ownership};
    use subscription_traits::{Subscriptions, SubscriberData};

    use ink::prelude::borrow::ToOwned;
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    // #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    // #[cfg_attr(
    //     feature = "std",
    //     derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    // )]

    // pub struct SubscriberData {
    //     pub balance: Balance,
    //     pub on: BlockNumber,
    // }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Subscription {
        /// Stores subscription value on the storage.
        subscription: Mapping<Hash, BTreeMap<AccountId, SubscriberData>>,
        // Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    #[ink(event)]
    pub struct Subscribe {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Unsubscribe {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Claimed {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        value: BTreeMap<AccountId, BlockNumber>,
        #[ink(topic)]
        total: Balance,
    }

    impl Subscription {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(init_value: AccountId) -> Self {
            Self {
                subscription: Default::default(),
                domain_router: init_value,
            }
        }

        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

        fn get_subscription_usage(&self, on: u32, curr: u32) -> Balance {
            self.get_current_rate() * (curr - on) as Balance
        }

        fn get_min_subscriber_period(&self) -> BlockNumber {
            BlockNumber::from(2u32)
        }

        // #[ink(message, payable)]
        // pub fn subscribe(&mut self, name: Hash, from: AccountId) {
        //     let payment = self.env().transferred_value();
        //     ink::env::debug_println!("subscribe payment: {}", payment);
        //     assert!(
        //         self.env().transferred_value() >= self.get_current_rate() * 30,
        //         "minimum subscription is 30 blocks"
        //     );
        //     // let on = self.env().block_number();
        //     let on = self.get_current_epoch();
        //     if let Some((ref _o, _b, _d)) = self.not_expired(name.clone()) {
        //         if let Some(mut old_value_map) = self.subscription.take(name.clone()) {
        //             old_value_map
        //                 .entry(from.clone())
        //                 .and_modify(|ov| {
        //                     let _b = (*ov).balance.saturating_add(payment.into());
        //                 })
        //                 .or_insert(SubscriberData {
        //                     balance: payment.into(),
        //                     on,
        //                 });
        //         } else {
        //             let mut map = BTreeMap::new();
        //             let _ = map.insert(
        //                 from,
        //                 SubscriberData {
        //                     balance: payment.into(),
        //                     on,
        //                 },
        //             );

        //             self.subscription.insert(name.clone(), &map);
        //         }

        //         Self::env().emit_event(Subscribe {
        //             name,
        //             from,
        //             value: payment.into(),
        //         });
        //     } else {
        //         ink::env::debug_println!("name not found");
        //         panic!("name not found");
        //     }
        // }

        // #[ink(message)]
        // pub fn unsubscribe(&mut self, name: Hash) {
        //     let caller = self.env().caller();
        //     // let bn = self.env().block_number();
        //     let epoch = self.get_current_epoch();
        //     let min = self.get_min_subscriber_period();
        //     if let Some(a) = self.subscription.get(&name) {
        //         if let Some(d) = a.get(&caller) {
        //             ink::env::debug_println!("epoch/min/on: {:?}/{:?}/{:?}", epoch, min, d.on);
        //             if (epoch - d.on) < min {
        //                 ink::env::debug_println!("min threshold not met");
        //                 panic!("min threshold not met");
        //             }
        //         } else {
        //             ink::env::debug_println!("subscriber not found");
        //             panic!("subscriber not found");
        //         }
        //     } else {
        //         ink::env::debug_println!("name not found");
        //         panic!("name not found");
        //     }
        //     let success = {
        //         if let Some(a) = self.subscription.get(&name) {
        //             if let Some(d) = a.get(&caller) {
        //                 let value = (*d).balance;
        //                 let usage = self.get_subscription_usage((*d).on, epoch);
        //                 let ret = value - usage;
        //                 if ret > 0 {
        //                     match self.env().transfer(caller.clone(), ret) {
        //                         // Err(ink_env::Error::BelowSubsistenceThreshold) => {
        //                         //     panic!(
        //                         //         "requested transfer would have brought contract\
        //                         //         below subsistence threshold!"
        //                         //     )
        //                         // }
        //                         Err(_) => panic!("transfer failed!"),
        //                         Ok(_) => {
        //                             if let Some((ref o, _b, _d)) = self.not_expired(name.clone()) {
        //                                 match self.env().transfer(o.clone(), usage) {
        //                                     // Err(ink_env::Error::BelowSubsistenceThreshold) => {
        //                                     //     panic!(
        //                                     //         "requested transfer would have brought contract\
        //                                     //         below subsistence threshold!"
        //                                     //     )
        //                                     // }
        //                                     Err(_) => panic!("transfer failed!"),
        //                                     Ok(_) => Some(ret),
        //                                 }
        //                             } else {
        //                                 ink::env::debug_println!("name not found");
        //                                 panic!("name not found");
        //                             }
        //                         }
        //                     }
        //                 } else {
        //                     Some(0)
        //                 }
        //             } else {
        //                 None
        //             }
        //         } else {
        //             None
        //         }
        //     };
        //     if let Some(value) = success {
        //         if let Some(mut a) = self.subscription.take(&name) {
        //             if let Some(_) = a.remove(&caller) {
        //                 self.subscription.insert(name, &a);
        //                 Self::env().emit_event(Unsubscribe {
        //                     name,
        //                     from: caller.clone(),
        //                     value,
        //                 });
        //             } else {
        //                 ink::env::debug_println!("failed to remove subscriber");
        //                 panic!("failed to remove subscriber");
        //             }
        //         } else {
        //             ink::env::debug_println!("name not found");
        //             panic!("name not found");
        //         }
        //     } else {
        //         ink::env::debug_println!("transfer failed");
        //         panic!("transfer failed");
        //     }
        // }

        // #[ink(message)]
        // pub fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData> {
        //     self.subscription
        //         .get(&name)
        //         .map(|data| data.get(&from).cloned())
        //         .flatten()
        // }

        // #[ink(message)]
        // pub fn claim_fees(&mut self, name: Hash) {
        //     if self.is_owner(name) {
        //         self.claim_fees_unchecked(name);
        //     } else {
        //         ink::env::debug_println!("not the owner");
        //         panic!("not the owner");
        //     }
        // }

        pub fn claim_fees_unchecked(&mut self, name: Hash) {
            let bn = { self.get_current_epoch() };
            let claimed = if let Some(a) = self.subscription.get(&name) {
                let mut total = Balance::from(0u128);
                let mut value = BTreeMap::new();
                for (sub_id, sub_data) in a.iter() {
                    let usage = self.get_subscription_usage(sub_data.on, bn);
                    total = total.saturating_add(usage);
                    let t = bn - sub_data.on;
                    value.insert(sub_id.clone(), t);
                }
                Claimed { name, value, total }
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            };
            if let Some(mut a) = self.subscription.take(&name) {
                for (_, sub_data) in a.iter_mut() {
                    (*sub_data).on = bn;
                }
                self.subscription.insert(name, &a);
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
            match self.env().transfer(self.env().caller(), claimed.total) {
                // Err(ink_env::Error::BelowSubsistenceThreshold) => {
                //     panic!(
                //         "requested transfer would have brought contract\
                //         below subsistence threshold!"
                //     )
                // }
                Err(_) => panic!("transfer failed!"),
                Ok(_) => (),
            }
            Self::env().emit_event(claimed);
        }
    }

    impl Subscriptions for Subscription {
        /// Subscribe to topic Hash.
        #[ink(message, payable)]
        fn subscribe(&mut self, name: Hash, from: AccountId) {
            let payment = self.env().transferred_value();
            ink::env::debug_println!("subscribe payment: {}", payment);
            assert!(
                self.env().transferred_value() >= self.get_current_rate() * 30,
                "minimum subscription is 30 blocks"
            );
            // let on = self.env().block_number();
            let on = self.get_current_epoch();
            if let Some((ref _o, _b, _d)) = self.not_expired(name.clone()) {
                if let Some(mut old_value_map) = self.subscription.take(name.clone()) {
                    old_value_map
                        .entry(from.clone())
                        .and_modify(|ov| {
                            let _b = (*ov).balance.saturating_add(payment.into());
                        })
                        .or_insert(SubscriberData {
                            balance: payment.into(),
                            on,
                        });
                } else {
                    let mut map = BTreeMap::new();
                    let _ = map.insert(
                        from,
                        SubscriberData {
                            balance: payment.into(),
                            on,
                        },
                    );

                    self.subscription.insert(name.clone(), &map);
                }

                Self::env().emit_event(Subscribe {
                    name,
                    from,
                    value: payment.into(),
                });
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        /// Unsubscribe to topic Hash.
        #[ink(message)]
        fn unsubscribe(&mut self, name: Hash) {
            let caller = self.env().caller();
            // let bn = self.env().block_number();
            let epoch = self.get_current_epoch();
            let min = self.get_min_subscriber_period();
            if let Some(a) = self.subscription.get(&name) {
                if let Some(d) = a.get(&caller) {
                    ink::env::debug_println!("epoch/min/on: {:?}/{:?}/{:?}", epoch, min, d.on);
                    if (epoch - d.on) < min {
                        ink::env::debug_println!("min threshold not met");
                        panic!("min threshold not met");
                    }
                } else {
                    ink::env::debug_println!("subscriber not found");
                    panic!("subscriber not found");
                }
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
            let success = {
                if let Some(a) = self.subscription.get(&name) {
                    if let Some(d) = a.get(&caller) {
                        let value = (*d).balance;
                        let usage = self.get_subscription_usage((*d).on, epoch);
                        let ret = value - usage;
                        if ret > 0 {
                            match self.env().transfer(caller.clone(), ret) {
                                // Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                //     panic!(
                                //         "requested transfer would have brought contract\
                                //         below subsistence threshold!"
                                //     )
                                // }
                                Err(_) => panic!("transfer failed!"),
                                Ok(_) => {
                                    if let Some((ref o, _b, _d)) = self.not_expired(name.clone()) {
                                        match self.env().transfer(o.clone(), usage) {
                                            // Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                            //     panic!(
                                            //         "requested transfer would have brought contract\
                                            //         below subsistence threshold!"
                                            //     )
                                            // }
                                            Err(_) => panic!("transfer failed!"),
                                            Ok(_) => Some(ret),
                                        }
                                    } else {
                                        ink::env::debug_println!("name not found");
                                        panic!("name not found");
                                    }
                                }
                            }
                        } else {
                            Some(0)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(value) = success {
                if let Some(mut a) = self.subscription.take(&name) {
                    if let Some(_) = a.remove(&caller) {
                        self.subscription.insert(name, &a);
                        Self::env().emit_event(Unsubscribe {
                            name,
                            from: caller.clone(),
                            value,
                        });
                    } else {
                        ink::env::debug_println!("failed to remove subscriber");
                        panic!("failed to remove subscriber");
                    }
                } else {
                    ink::env::debug_println!("name not found");
                    panic!("name not found");
                }
            } else {
                ink::env::debug_println!("transfer failed");
                panic!("transfer failed");
            }
        }

        /// Simply returns the current value of subscription.
        #[ink(message)]
        fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData> {
            self.subscription
                .get(&name)
                .map(|data| data.get(&from).cloned())
                .flatten()
        }

        /// Simply returns the current value of subscription.
        #[ink(message)]
        fn claim_fees(&mut self, name: Hash) {
            if self.is_owner(name) {
                self.claim_fees_unchecked(name);
            } else {
                ink::env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }
    }

    impl Epochs for Subscription {
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

    impl Economics for Subscription {
        /// Simply returns the current rate.
        #[ink(message)]
        fn get_current_rate(&self) -> Balance {
            <ink::contract_ref!(Economics)>::from(
                self.router_resolve_contract("registry".to_owned())
                    .expect("proper economics resolution"),
            )
            .get_current_rate()
        }
    }

    impl Ownership for Subscription {
        #[ink(message)]
        fn is_owner_from(&self, name: Hash, from: AccountId) -> bool {
            <ink::contract_ref!(Ownership)>::from(
                self.router_resolve_contract("registry".to_owned())
                    .expect("proper ownership resolution"),
            )
            .is_owner_from(name, from)
        }

        #[ink(message)]
        fn is_owner(&self, name: Hash) -> bool {
            self.is_owner_from(name.clone(), self.env().caller().clone())
        }
    }

    impl Lifecycle for Subscription {
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
    //         let subscription = Subscription::default();
    //         assert!(true);
    //     }
    // }
}
