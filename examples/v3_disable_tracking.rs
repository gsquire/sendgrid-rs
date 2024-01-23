use sendgrid::v3::{
    ClickTrackingSetting, Content, Email, Message, OpenTrackingSetting, Personalization, Sender,
    SubscriptionTrackingSetting, TrackingSettings,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} FROM TO", &args[0]);
        std::process::exit(1);
    }

    let from_email = &args[1];
    let from_name = "Sender";
    let to_email = &args[2];
    let to_name = "Address";
    let subject = "Hello from untracked";
    let html = r#"<h1>Hello!</h1>
    Welcome!<p>
    Visit <a href="https://crates.io/crates/sendgrid">sendgrid</a>
    "#;

    let api_key = std::env::var("SENDGRID_API_KEY").expect("Need to set SENDGRID_API_KEY environment variable");

    let person = Personalization::new(Email::new(to_email).set_name(to_name));

    let message = Message::new(Email::new(from_email).set_name(from_name))
        .set_subject(subject)
        .add_content(Content::new().set_content_type("text/html").set_value(html))
        .set_tracking_settings(TrackingSettings {
            click_tracking: Some(ClickTrackingSetting {
                enable: Some(false),
                enable_text: None,
            }),
            subscription_tracking: Some(SubscriptionTrackingSetting {
                enable: Some(false),
            }),
            open_tracking: Some(OpenTrackingSetting {
                enable: Some(false),
                substitution_tag: None,
            }),
        })
        .add_personalization(person);

    let sender = Sender::new(api_key.to_owned());
    match sender.blocking_send(&message) {
        Ok(res) => println!("sent {}", res.status()),
        Err(err) => eprintln!("err: {err}",),
    }
}

