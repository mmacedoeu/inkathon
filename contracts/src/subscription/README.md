# Subscription

## Description

The topic subscription module allows a subscriber to subscribe to a topic and receive content from a publisher. The subscriber gets a fee from the publisher for broadcasting the content. The subscriber with more staking increases the chances of being selected as a broadcaster during the next epoch. The subscriber needs to provide put a deposit for staking which could be slashed if the subscriber misbehaves.

## Functions

fn subscribe(&mut self, name: Hash, from: AccountId) // subscribe to a topic 'name' and 'from' account. Requires a deposit for staking.

fn unsubscribe(&mut self, name: Hash) // unsubscribe from a topic 'name' if already subscribed and caller is the subscriber.

fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData> // get subscription data for a topic 'name' and 'from' account.

fn claim_fees(&mut self, name: Hash) // claim fees, if any, for a topic 'name' if the caller is the subscriber.
