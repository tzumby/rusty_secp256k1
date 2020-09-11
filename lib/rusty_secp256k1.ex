defmodule RustySecp256k1 do
  use Rustler, otp_app: :rusty_secp256k1, crate: :secp256k1

  def ec_pubkey_create(_private_key, _compression), do: :erlang.nif_error(:nif_not_loaded)

  def ec_pubkey_tweak_add(_key, _tweak), do: :erlang.nif_error(:nif_not_loaded)

  def ec_pubkey_decompress(_key), do: :erlang.nif_error(:nif_not_loaded)

  def ec_sign(_message, _private_key), do: :erlang.nif_error(:nif_not_loaded)
end
