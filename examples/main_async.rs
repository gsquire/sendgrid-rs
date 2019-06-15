extern crate sendgrid;
extern crate futures;
extern crate tokio;

use futures::future::Future;
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};
use sendgrid::errors::SendgridError;

fn main() {

    let mut env_vars = std::env::vars();
    let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY");
    let api_key: String;
    match api_key_check {
        Some(key) => api_key = key.1,
        None => panic!("Must supply API key in environment variables to use!"),
    }

    let sg = SGClient::new(api_key);

    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

    let mail_info = Mail::new()
        .add_to(Destination {
            address: "you@example.com",
            name: "you there",
        })
        .add_from("some@some.com")
        .add_subject("Rust is rad")
        .add_html("<h1>Hello from SendGrid!</h1>")
        .add_from_name("Test")
        .add_header("x-cool".to_string(), "indeed")
        .add_x_smtpapi(&x_smtpapi);

    let sg_future = sg
        .send(mail_info)
        .map_err(|_| ())
        .map(|mail_response| {
            println!("{}", mail_response);
        });

    tokio::run(sg_future);
}
