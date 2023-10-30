# Delegation

## Defi

Allows a third party to delegate resources to a subscriber in exchange for a fee. A subscriber with more staking increases the chances of being selected as a broadcaster during the next epoch.

## Usage

* fn delegate(&mut self, name: Hash, from: AccountId) // Delegates resources to a subscriber on topic 'name' for the given account in from

* fn undelegate(&mut self, name: Hash) // Undelegates resources from a subscriber on topic 'name' for the given account in caller

* fn get_delegate(&self, investor: AccountId, name: Hash) -> Option<(Balance, BlockNumber)> // Returns the delegatee Balance and the block numbers duration of the delegation
