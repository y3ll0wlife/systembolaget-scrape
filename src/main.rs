mod database;
mod database_models;
mod systembolaget_models;

use std::env;

use database::Database;
use dotenv::dotenv;
use tokio::main;

use crate::systembolaget_models::SystembolagetSearchResponse;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    const URL: &str = "https://api-systembolaget.azure-api.net/sb-api-ecommerce";
    const PATH: &str = "v1/productsearch/search";

    let client = reqwest::Client::new();
    let mut database = Database::init().await;
    database.create_tables().await?;
    //let result = database.fetch_all_products().await;
    let max = 500;

    for i in 0..=max {
        let url: String = format!(
            "{}/{}?page={}&size=30&sortBy=Score&sortDirection=Ascending",
            URL, PATH, i
        );
        let resp = client
            .get(&url)
            .header(
                "Ocp-Apim-Subscription-Key",
                &env::var("SYSTEMBOLAGET_API_KEY")
                    .expect("'SYSTEMBOLAGET_API_KEY' is not configured"),
            )
            .send()
            .await?;

        let response = resp.json::<SystembolagetSearchResponse>().await?;

        for product in response.products {
            //println!("{:#?}", product);
            database.insert_product(product).await?;
        }
        println!("{}/{}", i, max);
    }

    Ok(())
}
