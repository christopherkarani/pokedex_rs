extern crate reqwest;
extern crate serde_json;
extern crate tokio;


mod pokeapi_endpoint;
mod poke_error;
use crate::pokeapi_endpoint::PokeAPIEndpoint;
use crate::poke_error::PokeError;
use std::fmt;
use std::future::Future;

use reqwest::Error;

enum PokeRegion {
    National,
    Kanto,
    Johto,
    Hoenn,
    Sinnoh,
    Unova,
    Kalos
}

impl PokeRegion {
    fn get_string(&self) -> String {
        match self {
            PokeRegion::National => "national".to_string(),
            _ => "Nothing".to_string()
        }
    }
}



#[tokio::main]
async fn main()   {
    println!("Hello World");
    let endpoint_manager = PokeAPIEndpoint::build();
    let pokedex_entry = get_pokedex(PokeRegion::National, endpoint_manager).await;
    println!("{}", pokedex_entry);
}

async fn get_pokemon(name: &str, endpoint: PokeAPIEndpoint) -> String {
    let endpoint = endpoint.get_pokemon(name).get_path();
    println!("Endpoint: {}", endpoint);
    let response = reqwest::get(endpoint.as_str()).await.unwrap().text().await.unwrap();

    response
}

async fn get_pokedex(region: PokeRegion, endpoint_manager: PokeAPIEndpoint) -> String {
    let endpoint_manager = endpoint_manager.construct_pokedex_entry_endpoint(region.get_string().as_str());
    let endpoint = endpoint_manager.get_path();
    println!("Endpoint: {}", endpoint);
    let body = reqwest::get(endpoint.as_str()).await.unwrap().text().await.unwrap();
    body
}