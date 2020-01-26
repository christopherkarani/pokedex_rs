extern crate reqwest;
extern crate serde_json;
mod pokeapi_endpoint;



fn main() {
    println!("Hello World");
    let endpoint = PokeAPIEndpoint::build();
    endpoint.get_pokemon("ditto")
}
