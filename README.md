## kvs_protocol

Simple protocol to interact with the KVS engine.

This crate includes, parser, serializer and deserializer for kvs_protocol.

To test parsers, you can start TCP listener in [src/bin/main.rs](./src/bin/main.rs)

```bash
cargo run
```

then, you can send the messages to see the output:

```bash
echo "+:get a:" | nc localhost 8080
echo "+:set a b:" | nc localhost 8080
echo "+:rm a :" | nc localhost 8080
```
