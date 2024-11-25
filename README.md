![](https://github.com/vrchatapi/vrchatapi.github.io/blob/main/static/assets/img/lang/lang_rust_banner_1500x300.png?raw=true)

# VRChat API Library for Rust

A Rust client to interact with the unofficial VRChat API. Supports all REST calls specified in https://github.com/vrchatapi/specification.

## Disclaimer

This is the official response of the VRChat Team (from Tupper more specifically) on the usage of the VRChat API.

> Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:
> * We do not provide documentation or support for the API.
> * Do not make queries to the API more than once per 60 seconds.
> * Abuse of the API may result in account termination.
> * Access to API endpoints may break at any given time, with no warning.

As stated, this documentation was not created with the help of the official VRChat team. Therefore this documentation is not an official documentation of the VRChat API and may not be always up to date with the latest versions. If you find that a page or endpoint is not longer valid please create an issue and tell us so we can fix it.

## Getting Started

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:
```
    vrchatapi = "^1.0"
```

See the [examples/example.rs](https://github.com/vrchatapi/vrchatapi-rust/blob/main/examples/example.rs) for getting started.

## Contributing

Contributions are welcome, but do not add features that should be handled by the OpenAPI specification.

Join the [Discord server](https://discord.gg/Ge2APMhPfD) to get in touch with us.
