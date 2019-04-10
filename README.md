# terribledb

It's a really bad database. In fact, it's arguably not even a database at all.
It should be a library that uses persistent local state to shard data across
the various application nodes in a data center. This should theoretically
reduce the operational overhead of this. It builds upon existing key-value
stores by providing first-class support for common aggregation operations.

The idea is to build a system that is extremely resilient against transient
communication outages as well as transient node outages, which are the two most
common types of outages I have encountered in production distributed systems.

## TODO

* [x] Listening on IO
* [ ] Communications protocol
* [ ] Multithreaded listener
* [ ] Service Discovery
* [ ] Key-Value store locally
* [ ] Distributed Key-Value Store
* [ ] Aggregations
* [ ] Separate this out into a library and a local daemon that provides an interface to the library

# Testing

After running with `terribledb loop` connect on udp with `nc -u localhost
1234`. You can cause the server to terminate by sending a message with the
utf-8 string `stahp`. Whitespace is automatically trimmed.
