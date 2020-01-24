extern crate reqwest;
extern crate serde_json;

static BASE_URL: &str = "https://pokeapi.co/api/v2";

fn main() {
    println!("Hello, world!");
    let new_endpoint = request(String::from("pokemon/ditto"));
    println!("{}", new_endpoint);
}

fn request(endpoint: String) -> String {
    let url_string = String::from(BASE_URL);
    let new_string = format!("{}/{}", url_string, endpoint);
    new_string
}
