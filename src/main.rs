extern crate sendgrid;

use sendgrid::{mail, sg_client};

fn main() {
    let mut env_vars = std::env::vars();
    let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY");
    let api_key: String;
    match api_key_check {
        Some(key) => api_key = key.1,
        None => panic!("Must supply API key in environment variables to use!")
    }

    let sg = sg_client::SGClient::new(api_key);

    let mut mail_info = mail::Mail::new();
    mail_info.add_to("garrettsquire@gmail.com");
    mail_info.add_from("garrett.squire@sendgrid.net");
    mail_info.add_subject("Test");
    mail_info.add_html("<h1>Cool</h1>");

    sg.send(mail_info);
}
