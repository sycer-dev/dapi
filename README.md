# dapi  
a cli app to interact with the discord api
[![Actions](https://img.shields.io/github/workflow/status/sycer-dev/dapi/dapi?style=flat)](https://github.com/sycer-dev/dapi/actions)
<!-- [![Crate](https://img.shields.io/crates/v/dapi.svg?style=flat)](https://crates.io/crates/dapi)
[![Downloads](https://img.shields.io/crates/d/dapi.svg?style=flat)](https://crates.io/crates/dapi) -->

## installation
Download the pre-build binary at https://github.com/sycer-dev/dapi/releases/latest
And if you have the rust toolchain already installed,
```sh
cargo install --git https://github.com/sycer-dev/dapi --branch main
```    

## setup
```sh
dapi config token set discord_token_here
```

## usage
```sh
dapi users @me
dapi users 492374435274162177
dapi build message --content "Hello World\!"  | dapi msg post 581635926757998613
echo '{ "content": "hello" } | dapi req post /channels/581635926757998613/messages
```
