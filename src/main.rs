use csv::ReaderBuilder;
use std::fs::File;
pub mod graph;
pub mod tia_price;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let daily_tvl_increases = get_tvl_values();
    create_graphs(daily_tvl_increases)?;
    Ok(())
}

pub fn create_graphs(tvl_increases: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    graph::create_tvl_graph(tvl_increases.clone())?;
    graph::create_profit_graph(tvl_increases)?;
    Ok(())
}
pub fn get_tvl_values() -> Vec<f64> {
    let file_path = "data/milkyway-zone.csv";
    let mut rdr = ReaderBuilder::new().from_reader(File::open(file_path).unwrap());
    let mut values = Vec::new();
    for (index, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        if index >= 5 {
            if let Ok(val) = record.get(3).unwrap_or("0").parse::<f64>() {
                values.push(val);
            }
        }
    }
    let mut daily_tvl_increases = values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<f64>>();
    daily_tvl_increases.truncate(20);
    daily_tvl_increases
}
