# sendgrid-rs
Unofficial Rust library for SendGrid API.

You must download an API key in order to use this library. I made it this way
to keep you more secure by using environment variables instead of including
sensitive information in your source code.

To create an API key for your SendGrid account see this [page](https://sendgrid.com/docs/API_Reference/Web_API_v3/API_Keys/index.html).

Then set that key as an environment variable as such (works with Bash and ZSH).

```shell
export SENDGRID_API_KEY="my.API.KEY"
```

To use this code in your project set this as a dependency in your Cargo.toml:
```shell
sendgrid = "0.1.0"
```

See the examples directory on how to use environment variables in Rust.

The library implements all of the functionality of other supported client libraries.
Documentation is in the works, but the example demonstrates the simplicity of it.

Please don't hesitate to contact me at the email listed in my profile. I will
try to help as quickly as I can. If you would like to contribute, contact me
as well.

# License
MIT
