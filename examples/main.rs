extern crate sendgrid;
extern crate time;

use sendgrid::mail::Mail;
use sendgrid::sg_client::SGClient;

fn main() {
    let mut env_vars = std::env::vars();
    let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY");
    let api_key: String;
    match api_key_check {
        Some(key) => api_key = key.1,
        None => panic!("Must supply API key in environment variables to use!"),
    }

    let sg = SGClient::new(api_key);

    let mut mail_info = Mail::new();
    mail_info.add_to("you@example.com");
    mail_info.add_from("some@some.com");
    mail_info.add_subject("Rust is rad");
    mail_info.add_html("<h1>Hello from SendGrid!</h1>");
    mail_info.add_from_name("Test");
    mail_info.add_header("x-cool", "indeed");

    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);
    mail_info.add_x_smtpapi(x_smtpapi);

    match sg.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {}", body),
    };
}
