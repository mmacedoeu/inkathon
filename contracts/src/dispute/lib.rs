#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod dispute {
    use epoch_traits::Epochs;
    use ink::prelude::borrow::ToOwned;
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use subscription_traits::{SubscriberData, Subscriptions};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dispute {
        dispute: BTreeMap<Hash, BTreeMap<AccountId, (Hash, u32)>>,
        reputation: Mapping<Hash, u16>,
        judge: Vec<AccountId>,
        veridic: Mapping<Hash, BTreeMap<AccountId, bool>>,
        // Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    #[ink(event)]
    pub struct Raised {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        cid: Hash,
    }

    #[ink(event)]
    pub struct WithdrawDispute {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        cid: Hash,
    }

    impl Dispute {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(judger: AccountId, init_value: AccountId) -> Self {
            let mut judgers = Vec::default();
            judgers.push(judger);
            Self {
                dispute: Default::default(),
                reputation: Default::default(),
                veridic: Default::default(),
                domain_router: init_value,
                judge: judgers,
            }
        }

        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

        fn is_judge(&self) -> bool {
            let caller = self.env().caller();
            self.judge.iter().any(|s| *s == caller)
        }

        #[ink(message)]
        pub fn insert_judge(&mut self, judge: AccountId) {
            let is_judge = self.is_judge();
            if is_judge {
                self.judge.push(judge);
            } else {
                ink::env::debug_println!("not a judge");
                panic!("not a judge");
            }
        }

        fn get_dispute_expiration(&self) -> u32 {
            10 // length in epochs
        }

        #[ink(message)]
        pub fn get_reputation(&self, hash_name: Hash) -> Option<u16> {
            self.reputation.get(&hash_name)
        }

        #[ink(message)]
        pub fn get_cid(&self, hash_name: Hash, subscriber: AccountId) -> Option<(Hash, u32)> {
            self.dispute
                .get(&hash_name)
                .map(|data| data.get(&subscriber))
                .flatten()
                .cloned()
        }

        #[ink(message)]
        pub fn cid_exists(&self, cid: Hash) -> bool {
            let epoch = self.get_current_epoch();
            let expire = self.get_dispute_expiration();
            self.dispute
                .values()
                .map(|data| {
                    data.values().filter_map(|(ref inner_cid, ref on)| {
                        if *inner_cid == cid && (epoch - on) < expire {
                            Some(true)
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .next()
                .is_some()
        }

        #[ink(message, selector = 0xDEADBABE)]
        pub fn raise_dispute(&mut self, name: Hash, cid: Hash) {
            let caller = self.env().caller();
            if let Some(data) = self.get_subscription(name.clone(), caller) {
                self.unchecked_raise_dispute(name, cid, data.clone());
            } else {
                ink::env::debug_println!("not subscribed");
                panic!("not subscribed");
            }
        }

        fn unchecked_raise_dispute(&mut self, hash_name: Hash, cid: Hash, _data: SubscriberData) {
            let epoch = self.get_current_epoch();
            let caller = self.env().caller();
            let expire = self.get_dispute_expiration();

            self.dispute
                .entry(hash_name.clone())
                .and_modify(|old_value_map| {
                    old_value_map
                        .entry(caller.clone())
                        .and_modify(|ov| {
                            let (ref _cid, ref on) = ov;
                            if epoch - on > expire {
                                *ov = (cid.clone(), epoch.clone());
                            } else {
                                panic!("dispute already in place");
                            }
                        })
                        .or_insert((cid.clone(), epoch.clone()));
                })
                .or_insert({
                    let mut map = BTreeMap::new();
                    let _ = map.insert(caller, (cid.clone(), epoch.clone()));
                    map
                });
            Self::env().emit_event(Raised {
                name: hash_name,
                from: caller,
                cid,
            });
        }

        #[ink(message, selector = 0xCAFEBABE)]
        pub fn withdraw_dispute(&mut self, name: Hash) {
            let caller = self.env().caller();
            let checked = if let Some(_data) = self.get_subscription(name.clone(), caller) {
                if let Some(disputes) = self.dispute.get(&name) {
                    disputes.get(&caller).cloned()
                } else {
                    ink::env::debug_println!("name not found");
                    panic!("name not found")
                }
            } else {
                ink::env::debug_println!("not subscribed");
                panic!("not subscribed");
            };

            if let Some(data) = checked {
                let (ref cid, ref _on) = data;
                self.unchecked_withdraw_dispute(name, cid.clone());
            } else {
                ink::env::debug_println!("caller not found");
                panic!("caller not found");
            }
        }

        fn unchecked_withdraw_dispute(&mut self, hash_name: Hash, cid: Hash) {
            let caller = self.env().caller();
            if let Some(a) = self.dispute.get_mut(&hash_name) {
                if let Some(_) = a.remove(&caller) {
                    let _ = self.veridic.take(&cid);
                    Self::env().emit_event(WithdrawDispute {
                        name: hash_name,
                        from: caller.clone(),
                        cid,
                    });
                } else {
                    ink::env::debug_println!("failed to remove caller");
                    panic!("failed to remove caller");
                }
            } else {
                ink::env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn submit_vote(&mut self, cid: Hash, vote: bool) {
            let is_judge = self.is_judge();
            if is_judge {
                let cid_exists = self.cid_exists(cid.clone());
                if cid_exists {
                    let caller = self.env().caller();
                    if let Some(mut old_value_map) = self.veridic.take(cid.clone()) {
                        old_value_map
                            .entry(caller.clone())
                            .and_modify(|ov| {
                                *ov = vote;
                            })
                            .or_insert(vote);
                        self.veridic.insert(cid.clone(), &old_value_map);
                    } else {
                        let mut map = BTreeMap::new();
                        let _ = map.insert(caller, vote);
                        self.veridic.insert(cid.clone(), &map);
                    }
                } else {
                    ink::env::debug_println!("invalid cid");
                    panic!("invalid cid");
                }
            } else {
                ink::env::debug_println!("not a judge");
                panic!("not a judge");
            }
        }
    }

    impl Epochs for Dispute {
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

    impl Subscriptions for Dispute {
        /// Simply returns the SubscriberData for given topic Hash.   
        #[ink(message)]
        fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData> {
            <ink::contract_ref!(Subscriptions)>::from(
                self.router_resolve_contract("subscription".to_owned())
                    .expect("proper subscription resolution"),
            )
            .get_subscription(name, from)
        }

        /// Subscribe to topic Hash.
        #[ink(message, payable)]
        fn subscribe(&mut self, name: Hash, from: AccountId) {
            <ink::contract_ref!(Subscriptions)>::from(
                self.router_resolve_contract("subscription".to_owned())
                    .expect("proper subscription resolution"),
            )
            .subscribe(name, from)
        }

        /// Unsubscribe to topic Hash.
        #[ink(message)]
        fn unsubscribe(&mut self, name: Hash) {
            <ink::contract_ref!(Subscriptions)>::from(
                self.router_resolve_contract("subscription".to_owned())
                    .expect("proper subscription resolution"),
            )
            .unsubscribe(name)
        }

        /// if any transfer any received fees for given topic Hash.
        #[ink(message)]
        fn claim_fees(&mut self, name: Hash) {
            <ink::contract_ref!(Subscriptions)>::from(
                self.router_resolve_contract("subscription".to_owned())
                    .expect("proper subscription resolution"),
            )
            .claim_fees(name)
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
    //         // let dispute = Dispute::default();
    //         assert!(true);
    //     }
    // }
}
