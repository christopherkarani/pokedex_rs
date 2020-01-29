extern crate reqwest;
extern crate serde_json;
mod pokeapi_endpoint;
use crate::pokeapi_endpoint::PokeAPIEndpoint;
use tokio::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    println!("Hello World");
    let endpoint = PokeAPIEndpoint::build();
    let future = get_pokemon("ditto", endpoint).await;
    println!("{}", future);

    Ok(())
}

async fn get_pokemon(name: &str, endpoint: PokeAPIEndpoint) -> String {
    let endpoint = endpoint.get_pokemon(name).get_path();
    let response = reqwest::get(endpoint.as_str()).await.unwrap();

    let mut result_string = String::new();

    if  response.status().is_success() {
        result_string = "everythings good".to_string();
    } else {
        result_string = "Everythings gone wrong".to_string();
    }

    result_string
}