# Disputes

Even for the most well designed systems, there will be times when a dispute arises. This is a natural part of any system, and is not a sign of failure. The key is to have a process in place to handle disputes in a fair and transparent manner.

## Judges and Defi

Earn rewards by judging disputes. The more accurate your judgements, the more rewards you earn. The less accurate your judgements, the less rewards you earn. If you are a bad judge, you will lose your stake.

## Topic reputation

Earn reputation by making good judgements. The more reputation you have, the more your judgements count. If you have a bad reputation, your judgements will count for less.

Topic reputation is a measure of good behaviour by publishers. It is a measure of how well a publisher has behaved in the past. The more good behaviour, the higher the reputation. The more bad behaviour, the lower the reputation.

## Functionalities

fn get_reputation(&self, hash_name: Hash) -> Option<u16> // get the reputation for the topic name if it is registered

fn get_cid(&self, hash_name: Hash, subscriber: AccountId) -> Option<(Hash, u32)> // get the cid, external document reference, for the topic name if it is registered and blocknumber when it was raised

fn cid_exists(&self, cid: Hash) -> bool // check if the cid exists

fn raise_dispute(&mut self, name: Hash, cid: Hash) // raise a dispute for a topic name with a cid, external document reference

fn withdraw_dispute(&mut self, name: Hash) // withdraw a dispute for a topic name

fn submit_vote(&mut self, cid: Hash, vote: bool) // submit a vote for a cid if you are a judge

## TODO

- [ ] Allow more than one dispute per AccountId
- [ ] Parameterize the constants from a DAO
- [ ] Improve the judge selection process
- [ ] Improve the resolution process
