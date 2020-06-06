use alexa_sdk::Request;
use anyhow::{anyhow, Result};
use log::info;
use openweather::LocationSpecifier;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostalCodeResponse {
    #[serde(rename = "postalCode")]
    postal_code: String,
    #[serde(rename = "countryCode")]
    country_code: String,
}

impl From<PostalCodeResponse> for LocationSpecifier {
    fn from(src: PostalCodeResponse) -> Self {
        LocationSpecifier::ZipCode {
            zip: src.postal_code,
            // Super brittle to use the Alexa country code directly with OpenWeatherMap
            // Works for the US so good enough for now
            country: src.country_code,
        }
    }
}

fn get_geolocation(req: &Request) -> Option<LocationSpecifier> {
    let coordinate = req.context.geolocation.as_ref()?.coordinate?;
    let lat = coordinate.latitude_degrees;
    let lon = coordinate.longitude_degrees;
    info!("Location source: Geolocation");
    Some(LocationSpecifier::Coordinates { lat, lon })
}

pub fn get_location(req: &Request) -> Result<LocationSpecifier> {
    if let Some(loc) = get_geolocation(req) {
        return Ok(loc);
    }

    let api_endpoint = req
        .context
        .system
        .api_endpoint
        .as_ref()
        .ok_or_else(|| anyhow!("No api_endpoint given"))?;
    let api_access_token = req
        .context
        .system
        .api_access_token
        .as_ref()
        .ok_or_else(|| anyhow!("No access token given"))?;
    let auth_header = format!("Bearer {}", api_access_token);
    let device_id = &req
        .context
        .system
        .device
        .as_ref()
        .ok_or_else(|| anyhow!("No device given"))?
        .device_id;
    let uri = format!(
        "{api_endpoint}/v1/devices/{device_id}/settings/address/countryAndPostalCode",
        api_endpoint = api_endpoint,
        device_id = device_id
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&uri)
        .header("Accept", "application/json")
        .header("Authorization", auth_header)
        .send()?;
    if response.status() != 200 {
        return Err(anyhow!("Received error code {}", response.status()));
    }

    let loc: LocationSpecifier =
        serde_json::from_str::<PostalCodeResponse>(&response.text()?)?.into();
    info!("Location source: Get Country/Region and Postal Code API");
    Ok(loc)
}
