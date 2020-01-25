extern crate reqwest;
extern crate serde_json;

static BASE_URL: &str = "https://pokeapi.co/api/v2";

fn main() {
    println!("Hello World")
}

/// Use this to build enpoints from the PokeApi
struct PokeAPIEndpoint {
    path: String,
}

impl PokeAPIEndpoint {
    /// create an instance of the endpoint builder
    /// uses the base url: `"https://pokeapi.co/api/v2"`
    fn build() -> PokeAPIEndpoint {
        PokeAPIEndpoint {
            path: String::from(BASE_URL),
        }
    }
    /// Use this to create an endpoint from the
    /// # Arguments
    /// * `endpoint` - the path of the API you want to access using the base URL of
    ///  "`https://pokeapi.co/api/v2"`
    fn make_pokeapi_endpoint(&self, endpoint: String) -> PokeAPIEndpoint {
        let url_string = self.path.clone();
        let new_string = format!("{}/{}", url_string, endpoint);
        PokeAPIEndpoint { path: new_string }
    }
    /// construct an enpoint with a "/pokemon" path simply give the name of the pokemon
    /// to begin the querry
    /// # Arguments
    /// *`pokemon_name`- the name of the pokemon you want to look up
    ///
    fn construct_pokemon_enpoint(&self, pokemon_name: String) -> PokeAPIEndpoint {
        let path = String::from("/pokemon");
        let endpoint = format!("{}/{}", path, pokemon_name);
        self.make_pokeapi_endpoint(endpoint)
    }
}
