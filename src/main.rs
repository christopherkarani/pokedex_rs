extern crate reqwest;
extern crate serde_json;

use std::borrow::Borrow;

static BASE_URL: &str = "https://pokeapi.co/api/v2";

fn main() {
    println!("Hello World");
    let endpoint = PokeAPIEndpoint::build();
    endpoint.get_pokemon("ditto")
}

/// Use this to build endpoints from the PokeApi
struct PokeAPIEndpoint {
    path: String,
}

impl PokeAPIEndpoint {
    /// create an instance of the endpoint builder
    /// uses the base url: `"https://pokeapi.co/api/v2"`
    fn build() -> PokeAPIEndpoint {
        PokeAPIEndpoint { path: String::from(BASE_URL), }
    }
    /// Use this to create an endpoint from the
    /// # Arguments
    /// * `endpoint` - the path of the API you want to access using the base URL of
    ///  "`https://pokeapi.co/api/v2"`
    fn make_pokeapi_endpoint(&self, endpoint: &str) -> PokeAPIEndpoint {
        let url_string = self.path.clone();
        let new_string = format!("{}{}", url_string, endpoint);
        PokeAPIEndpoint { path: new_string }
    }
    /// construct an endpoint with a "/pokemon" path simply give the name of the pokemon
    /// to begin the query
    /// # Arguments
    /// *`pokemon_name`- the name of the pokemon you want to look up
    ///
    fn construct_pokemon_enpoint(&self, pokemon_name: &str) -> PokeAPIEndpoint {
        let path = String::from("/pokemon");
        let endpoint = format!("{}/{}", path, pokemon_name);
        self.make_pokeapi_endpoint(endpoint.as_str())
    }

    fn get_pokemon(&self,name: &str) {
        let endpoint = self.construct_pokemon_enpoint(name);
        let endpoint_path = endpoint.path;

        let response = reqwest::get(endpoint_path).await;
        println!("getting pokemon");
        println!("Endpoint: {}", endpoint_path);
    }
}
