use std::collections::HashMap;

use sendgrid::errors::SendgridError;
use sendgrid::v3::*;

#[tokio::main]
async fn main() -> Result<(), SendgridError> {
    let mut cool_header = HashMap::new();
    cool_header.insert(String::from("x-cool"), String::from("indeed"));
    cool_header.insert(String::from("x-cooler"), String::from("cold"));

    let p = Personalization::new()
        .add_to(Email::new().set_email("test@test.fr"))
        .add_headers(cool_header);

    let m = Message::new()
        .set_from(Email::new().set_email("g@gmail.com"))
        .set_subject("Subject")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value("Test"),
        )
        .add_personalization(p);

    let mut env_vars = ::std::env::vars();
    let api_key = env_vars.find(|v| v.0 == "SG_API_KEY").unwrap();
    let sender = Sender::new(api_key.1);
    let resp = sender
        .send(&m).await?;
    println!("status: {}", resp.status());

    Ok(())
}
