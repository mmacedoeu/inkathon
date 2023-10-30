#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod topic_meta {
    use ink::prelude::borrow::ToOwned;
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    use registry_traits::Ownership;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct TopicMeta {
        link: Mapping<Hash, String>,
        capabilities: Mapping<Hash, BTreeMap<Hash, String>>,
        // Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Link {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        link: String,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Capabilities {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        property: Hash,
        #[ink(topic)]
        value: String,
    }

    impl TopicMeta {
        #[ink(constructor)]
        pub fn new(init_value: AccountId) -> Self {
            Self {
                domain_router: init_value,
                link: Default::default(),
                capabilities: Default::default(),
            }
        }

        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

        #[ink(message, selector = 0xCAFEDEAD)]
        pub fn set_link(&mut self, name: Hash, link: String) {
            if self.is_owner(name) {
                self.set_link_unchecked(name, link);
            } else {
                ink::env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }

        fn set_link_unchecked(&mut self, name_hash: Hash, link: String) {
            ink::env::debug_println!("link name_hash: {:?}", name_hash);
            // if let Some((_a, _b, _d)) = self.expired(name_hash) {
            //     self.unregister_unchecked(name_hash);
            // } else {
            self.link.take(name_hash);
            self.link.insert(name_hash, &link);

            Self::env().emit_event(Link {
                name: name_hash,
                link,
            });
        }

        #[ink(message, selector = 0xCAFE)]
        pub fn set_capability(&mut self, name: Hash, property: Hash, value: String) {
            if self.is_owner(name) {
                self.set_capability_unchecked(name, property, value);
            } else {
                ink::env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }

        fn set_capability_unchecked(&mut self, name_hash: Hash, property: Hash, value: String) {
            ink::env::debug_println!("capability name_hash: {:?}", name_hash);
            // if let Some((_a, _b, _d)) = self.expired(name_hash) {
            //     self.unregister_unchecked(name_hash);
            // } else {
            if let Some(mut old_value) = self.capabilities.take(name_hash) {
                old_value.insert(property.clone(), value.clone());

            } else {
                let mut map = BTreeMap::default();
                let _ = map.insert(property.clone(), value.clone());
                self.capabilities.insert(name_hash, &map);
            }

            Self::env().emit_event(Capabilities {
                name: name_hash,
                property,
                value,
            });
        }

        #[ink(message)]
        pub fn get_capabilities(&self, name: Hash) -> BTreeMap<Hash, String> {
            let cap = self.capabilities.get(&name);
            if let Some(c) = cap {
                c
            } else {
                BTreeMap::default()
            }
        }

        #[ink(message)]
        pub fn get_link(&self, name: Hash) -> Option<String> {
            self.link.get(&name)
        }
    }

    impl Ownership for TopicMeta {
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
    //         let indexer_meta = TopicMeta::default();
    //         assert_eq!(true);
    //     }
    // }
}
