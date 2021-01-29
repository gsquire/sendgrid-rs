use std::collections::HashMap;

use sendgrid::v3::*;

fn main() {
    let mut cool_header = HashMap::with_capacity(2);
    cool_header.insert(String::from("x-cool"), String::from("indeed"));
    cool_header.insert(String::from("x-cooler"), String::from("cold"));

    let p = Personalization::new(Email::new("test@example.com")).add_headers(cool_header);

    let m = Message::new(Email::new("g@gmail.com"))
        .set_subject("Subject")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value("Test"),
        )
        .add_personalization(p);

    let api_key = ::std::env::var("SG_API_KEY").unwrap();
    let sender = Sender::new(api_key);
    let code = sender.send(&m);
    println!("{:?}", code);
}
