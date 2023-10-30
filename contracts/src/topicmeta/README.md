# Topic meta data

This directory contains meta data for topics. The meta data is used by brokers to determine behavior and how to setup the publisher data source when broadcasting messages. It also provides information by means of the `link`.

## Functionalities

fn set_link(&mut self, name: Hash, link: String) // associate the link for the topic name if it is registered and caller is the owner

fn get_link(&self, name: Hash) -> Option<String> // get the link for the topic name if it is registered

fn set_capability(&mut self, name: Hash, property: Hash, value: String) // set the capability for the topic name if it is registered and caller is the owner

fn get_capabilities(&self, name: Hash) -> BTreeMap<Hash, String> // get the capability for the topic name if it is registered

## Examples

check sandbox.ts for examples
