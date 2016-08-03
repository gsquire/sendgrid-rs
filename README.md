# sendgrid-rs
Unofficial Rust library for the SendGrid API.

[![Build Status](https://travis-ci.org/gsquire/sendgrid-rs.svg?branch=master)](https://travis-ci.org/gsquire/sendgrid-rs)

sendgrid-rs implements all of the functionality of other supported SendGrid client libraries.
To use sendgrid-rs you must first create a SendGrid account and generate an API key. To create an API key for your SendGrid account, use the account management interface or see the [SendGrid API Documentation](https://sendgrid.com/docs/API_Reference/Web_API_v3/API_Keys/index.html).

sendgrid-rs is available on [crates.io](https://crates.io/crates/sendgrid) and can be included in your Cargo.toml as follows:

```toml
[dependencies]
sendgrid = "0.4"
```

## Build Dependencies

This library utilises [hyper](https://crates.io/crates/hyper), which in turn requires the OpenSSL headers to be available during compilation. For more information on how to configure OpenSSL, see: [rust-openssl](https://github.com/sfackler/rust-openssl)

## Example

An example of using this library can be found in the examples directory. This example code expects to find your SendGrid API key in the process environment. In shells such as Bash or ZSH this can be set as follows:

```shell
export SENDGRID_API_KEY="SG.my.api.key"
```

## Documentation
[Documentation](https://gsquire.github.io/doc/sendgrid/sendgrid)

Please don't hesitate to contact me at the email listed in my profile. I will
try to help as quickly as I can. If you would like to contribute, contact me
as well.

## Mentions
Thanks to [meehow](https://github.com/meehow) for his contributions to improve the library.

## License
MIT
