# coingecko

Rust library for the CoinGecko V3 API (formerly known as `coingecko-rs` crate)

<p align="center">
    <img height="auto" width="300px" src="logo.png" />
</p>

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
coingecko = "1.1.3"
# If you want to use rustls-tls
# coingecko = { version = "1.1.3", features = ["rustls-tls"] }
```

## Features

- Supports all API endpoints
- Responses are fully typed using `serde_json`
- Date params using `chrono`
- Market order enum params

## Documentation

[docs.rs](https://docs.rs/coingecko)
