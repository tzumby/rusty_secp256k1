defmodule RustySecp256k1.MixProject do
  use Mix.Project

  def project do
    [
      app: :rusty_secp256k1,
      version: "0.1.0",
      elixir: "~> 1.9",
      description: description(),
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      source_url: "https://github.com/tzumby/rusty_secp256k1",
      package: %{
        name: "rusty_secp256k1",
        licenses: ["Apache License 2.0"],
        links: %{"GitHub" => "https://github.com/tzumby/rusty_secp256k1"}
      },
      rustler_crates: [
        secp256k1: []
      ],
      name: "RustySecp256k1"
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp description do
    "NIF library using pure Rust libsecp256k1 implementation."
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, "~> 0.21", only: :dev, runtime: false},
      {:rustler, "~> 0.21.0"}
    ]
  end
end
