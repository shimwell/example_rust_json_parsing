use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    energy: Vec<f32>,
    cross_section: Vec<f32>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON
    
    let todos: Todo = reqwest::get("https://raw.githubusercontent.com/shimwell/example_rust_json_parsing/main/example.json")
        .await?
        .json()
        .await?;

    println!("{:#?}", todos.cross_section);
    println!("{:#?}", todos.energy);

    Ok(())
}
