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
            PokeRegion::Kanto => "kanto".to_string(),
            PokeRegion::Johto => "johto".to_string(),
            PokeRegion::Hoenn => "hoenn".to_string(),
            PokeRegion::Unova => "unove".to_string(),
            PokeRegion::Kalos => "kalos".to_string(),
            PokeRegion::Sinnoh => "sinnoh".to_string()
        }
    }
}



struct Language {
    name: String,
    url: String
}
struct Description {
    description: String
}

struct PokeDexName {
    name: String,
    language: Language
}


struct PokemonResource {
    name: String,
    url: String
}
struct PokeDexEntries {
    entry_number: i32,
    pokemon_species: PokemonResource
}

struct NamedResource {
    name: String,
    url: String
}
struct PokeDexResponse {
    id: i32,
    name: String,
    is_main_series: bool,
    names: Vec<PokeDexName>,
    descriptions: Vec<Description>,
    entries: Vec<PokeDexEntries>,
    version_groups: Vec<NamedResource>
}


#[tokio::main]
async fn main()   {
    println!("Hello World");
    let endpoint_manager = PokeAPIEndpoint::build();
    let pokedex_entry = get_pokedex(PokeRegion::Kanto, endpoint_manager).await;
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