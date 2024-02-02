use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    celestia: CurrencyInfo,
}

#[derive(Deserialize, Debug)]
struct CurrencyInfo {
    usd: f64,
}

#[tokio::main]
pub async fn get_tia_price() -> Result<f64, Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=celestia&vs_currencies=usd";
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    println!("Celestia price in USD: {}", response.celestia.usd);

    Ok(response.celestia.usd)
}
