mod location;
mod outfit_intent;

use alexa_sdk::request::IntentType;
use alexa_sdk::{Request, Response};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn handle_help(_req: &Request) -> Result<Response, Error> {
    Ok(Response::simple(
        "Help",
        "Outfit Picker can help you pick a running outfit. Try saying \"find me an outfit\".",
    ))
}

fn handle_cancel(_req: &Request) -> Result<Response, Error> {
    Ok(Response::end())
}

async fn my_handler(req: Request) -> Result<Response, Error> {
    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::Cancel => handle_cancel(&req),
        IntentType::User(_) => outfit_intent::handler(&req),
        // TODO; handle other Amazon built-in intents
        _ => handle_cancel(&req),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let handler = lambda::handler_fn(my_handler);
    lambda::run(handler).await?;

    Ok(())
}
