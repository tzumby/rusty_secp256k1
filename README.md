# RustySecp256k1

NIF library using `rustler` and a [pure Rust implementation of Secp256k1](https://github.com/paritytech/libsecp256k1).
This is not an exhaustive port of the library to Elixir but only meant to provide functionality for:

- Creating Public Key from Private Keys
- Decompressing Public Keys
- Performing point addition using Public Keys and Tweak Keys

For a complete libsecp256k1 implementation see [exthereum/libsecp256k1](https://github.com/exthereum/libsecp256k1) for a C-based NIF. 


## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `rusty_secp256k1` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:rusty_secp256k1, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/rusty_secp256k1](https://hexdocs.pm/rusty_secp256k1).

