use dotenv;
use http::StatusCode;
use reqwest::{blocking::Client, Error};
use reqwest::header;
use serde_json::json;
use std::env;

struct User {
    name: String,
    email: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let api_key = env::var("SG_API_KEY").unwrap();

    let sender = User {
        name: String::from(env::var("S_NAME").unwrap()),
        email: String::from(env::var("S_EMAIL").unwrap()),
    };

    let recipient = User {
        name: String::from(env::var("R_NAME").unwrap()),
        email: String::from(env::var("R_EMAIL").unwrap()),
    };

    let body = json!(
        {
            "personalizations": [{
                "to": [{
                    "email": recipient.email,
                    "name": recipient.name
                }]
            }],
            "from": {
                "email": sender.email,
                "name": sender.name
            },
            "subject": "Let's Send an Email With Rust and SendGrid",
            "content": [
                {
                    "type": "text/plain",
                    "value": "Here is your AMAZING email!"
                },
                {
                    "type": "text/html",
                    "value": "Here is your <strong>AMAZING</strong> email!"
                },
            ]
        }
    );

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send()?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}. Body content was: {:?}",
            response.status(),
            response.text()
        ),
    }

    Ok(())
}
