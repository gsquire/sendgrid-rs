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
    mail_info.add_to("garrettsquire@gmail.com");
    mail_info.add_from("garrett.squire@sendgrid.net");
    mail_info.add_subject("Rust is rad");
    mail_info.add_text("What's up?");

    let mut x_smtpapi = String::new();
    x_smtpapi.push_str("{\"unique_args\":{\"test\":7}}");
    mail_info.add_x_smtpapi(x_smtpapi);

    sg.send(mail_info);
}
