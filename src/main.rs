use reqwest::*;
use std::collections::HashMap;

fn main(){
    // mini_get();
    // mini_post();
    mini_json_get();
    // mini_json_post();
    println!("Finish running");
}

#[tokio::main]
async fn mini_get() -> Result<()>{
    let _response = reqwest::get("http://127.0.0.1:8000")
    // let response = reqwest::get("https://docs.rs/reqwest/0.9.18/reqwest/index.html")
        .await?
        .text()
        .await?; 
    Ok(())
}

#[tokio::main]
async fn mini_json_get() -> Result<()>{
    let response = reqwest::get("http://127.0.0.1:8000")
        .await?
        .json()
        .await?;
    println!("body = {:?}", response);    
    Ok(())
}

#[tokio::main]
async fn mini_post() -> Result<()>{
    let client = reqwest::Client::new();
    let _res = client.post("http://127.0.0.1:8000")
        .body("Hello rust")
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn mini_json_post() -> Result<()>{
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let _res = client.post("http://127.0.0.1:8000")
        .json(&map)
        .send()
        .await?;
    Ok(())
}

// async fn mini_put() -> Result<()>{
//     let client = reqwest::Client::new();
//     let res = client.put().body("");
//     Ok(())
// }
