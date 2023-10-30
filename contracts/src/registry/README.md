# Registry

The registry is a service that stores information about the available topics and their publishers and subscribers. It is used by the broker node to determine which topics to broadcast messages from.

## Functionalities

* Functionality to register a topic

fn get_hash(&self, name: String) -> Hash // calculate the hash of the topic name
fn valid(&self, name: String) -> bool // check if the topic name is valid
fn available(&self, name: String) -> bool // check if the topic name is available for registration
fn make_commitment(&self, name: String, owner: AccountId, secret: u32) -> Hash // calculate the commitment of the topic name in order to prevent front-running the registration
fn commit(&mut self, commitment: Hash) // commit the commitment to the registry as a pending registration valid for a few blocks in order to prevent front-running the registration
fn rent_price(&self, name: String, duration: u32) -> Balance // calculate the rent price for the topic name for the specified duration
fn register(&mut self, name: String, from: AccountId, duration: u32, secret: u32) // register the topic name for the specified duration if a commitment was made for it
fn unregister(&mut self, name: Hash) // unregister the topic name if it is registered and caller is the owner

## TODO List

* [ ] Implement the renenwal of the registration
* [ ] Delegates rates and constants to the DAO manager and the marketplaces

## Example

Check sandbox.ts for a complete example of the registry module.
