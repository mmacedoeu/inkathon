#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod router {
    use azns_integration::contract_ref::AznsRouter;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Router {
        names: Mapping<String, AccountId>,
    }

    impl Router {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn insert(&mut self, name: String, account_id: AccountId) -> Option<u32> {
            self.names.insert(name, &account_id)
        }
    }

    impl AznsRouter for Router {
        #[ink(message)]
        fn get_all_registries(&self) -> Vec<AccountId> {
            Vec::new()
        }

        #[ink(message)]
        fn get_registry(&self, tld: String) -> Option<AccountId> {
            self.names.get(&tld)
        }

        #[ink(message)]
        fn get_address(&self, domain: String) -> Result<AccountId, u8> {
            self.names.get(&domain).ok_or(0)
        }

        #[ink(message)]
        fn get_primary_domains(
            &self,
            _account: AccountId,
            _tld: Option<String>,
        ) -> Vec<(AccountId, String)> {
            Vec::new()
        }
    }
}
