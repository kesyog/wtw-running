mod error;
mod location;
mod outfit_intent;

use alexa_sdk::request::IntentType;
use alexa_sdk::{Request, Response};
use anyhow::{bail, Result};
use log::{error, warn};

fn handle_help(_req: &Request) -> Result<Response> {
    Ok(Response::simple(
        "Help 👟",
        "Outfit Picker can help you pick a running outfit. Try saying \"find me an outfit\".",
    ))
}

fn handle_fallback(_req: &Request) -> Result<Response> {
    Ok(Response::simple(
        "Try something else 🏃",
        "I didn't catch that. Try saying \"find me an outfit\" or \"what should I wear on my long \
        run?\".",
    ))
}

fn handle_cancel(_req: &Request) -> Result<Response> {
    Ok(Response::end())
}

async fn my_handler(req: Request) -> Result<Response> {
    match req.intent() {
        IntentType::Cancel | IntentType::Stop => handle_cancel(&req),
        IntentType::Fallback => handle_fallback(&req),
        IntentType::Help => handle_help(&req),
        IntentType::User(_) => outfit_intent::handler(&req),
        intent => {
            warn!("Unknown intent {:?}", intent);
            handle_fallback(&req)
        }
    }
    .or_else(|err| {
        error!("Error: {}", err);
        Ok(Response::simple(
            "Technical difficulties 💣",
            "We're having some technical difficulties right now. Please try again later.",
        ))
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;
    let handler = lambda::handler_fn(my_handler);
    match lambda::run(handler).await {
        Ok(()) => Ok(()),
        Err(e) => bail!(e),
    }
}
