use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    energy: f32,
    cross_section: f32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON
    
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/shimwell/example_rust_json_parsing/main/example.json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

    Ok(())
}