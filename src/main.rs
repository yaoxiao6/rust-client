use reqwest::*;

// #[tokio::main]
fn main(){
    mini_post();
    mini_get();
}

#[tokio::main]
async fn mini_get() -> Result<()>{
    let response = reqwest::get("https://127.0.0.1:8000")
        .await?
        .text()
        .await?;
    println!("{:?}", response);
    Ok(())
}

async fn mini_post() -> Result<()>{
    let client = reqwest::Client::new();
    let _res = client.post("https://127.0.0.1:8000")
        .body("Hello rust")
        .send()
        .await?;
    Ok(())
}

// async fn mini_put() -> Result<()>{
//     let client = reqwest::Client::new();
//     let res = client.put().body("");
//     Ok(())
// }