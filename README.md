## kvs_protocol

Simple protocol to interact with the KV engine.

This crate includes parser, serializer and deserializer for the custom protocol for the KV engine.

To test parsers, you can start TCP listener in [src/bin/main.rs](./src/bin/main.rs)

```bash
cargo run
```

then, you can send the command(s) to see the output:

```bash
echo "+:get a:" | nc localhost 8080
echo "+:set a b:" | nc localhost 8080
echo "+:rm a :" | nc localhost 8080
echo "+:rm a :+:get a:+:set a b:" | nc localhost 8080
```

## Protocol

Each command starts with `+:` and ends with `:`.
There are three commands available:
- `get <k>` to get the key `<k>`
- `set <k> <v>` to set the key `<k>` to the value `<v>`
- `rm <k>` to remove the key `<k>`


Each log file will include a sequence of commands that the server walked through.