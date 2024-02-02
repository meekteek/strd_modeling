use csv::ReaderBuilder;
use plotters::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
pub mod tia_price;

const TOTAL_TVL: f64 = 5000000.0;
const STRD_EMISSION: f64 = 50000.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut daily_tvl_increases = get_tvl_values();
    create_tvl_graph(&mut daily_tvl_increases)?;
    create_profit_graph(daily_tvl_increases)?;
    Ok(())
}

fn create_profit_graph(daily_tvl_increases: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("daily_returns.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Return on stakedTia", ("sans-serif", 50))
        .margin(25)
        .x_label_area_size(80)
        .y_label_area_size(100)
        .build_cartesian_2d(0.0..20.0, 0.0..30.0)?;

    chart
        .configure_mesh()
        .x_desc("Days after Deposit")
        .y_desc("Total return on investment %")
        .label_style(("sans-serif", 15))
        .draw()?;

    let staked_value = get_portfolio_value();
    let mut total_tvl = TOTAL_TVL;
    let tvl_iter = (0..=19).map(|x| {
        (x as f64, {
            total_tvl = total_tvl + daily_tvl_increases[x as usize];
            total_tvl
        })
    });
    let mut total_return = 0.0;
    // based on tia amount, calculate daily return and plot the daily return on graph
    let profit_iter = tvl_iter.map(|(x, total)| {
        (x, {
            let ratio_of_tvl = staked_value / total;
            let strd_rewards = ratio_of_tvl * STRD_EMISSION;
            total_return += ((strd_rewards * 3.0) / staked_value) * 100.0;
            total_return
        })
    });

    chart.draw_series(LineSeries::new(profit_iter, &RED))?;
    root.present()?;
    Ok(())
}
fn create_tvl_graph(daily_tvl_increases: &mut Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("tvl.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Possible stTia TVL", ("sans-serif", 50))
        .margin(25)
        .x_label_area_size(80)
        .y_label_area_size(100)
        .build_cartesian_2d(0.0..20.0, 3_000_000.0..50_000_000.0)?;

    chart
        .configure_mesh()
        .x_desc("Days after launch")
        .y_desc("Total TVL $$$")
        .label_style(("sans-serif", 15))
        .draw()?;

    let mut running_total_tvl = TOTAL_TVL;
    let tvl_iter = (0..=19).map(|x| {
        (x as f64, {
            running_total_tvl = running_total_tvl + daily_tvl_increases[x as usize];
            running_total_tvl
        })
    });
    chart.draw_series(LineSeries::new(tvl_iter, &RED))?;
    root.present()?;
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

pub fn get_portfolio_value() -> f64 {
    let file = File::open("data/input.txt").unwrap();
    let mut file_data = io::BufReader::new(file);
    let mut first_line = String::new();
    _ = file_data.read_line(&mut first_line).unwrap();
    let tia_amount = first_line
        .trim()
        .parse::<f64>()
        .expect("Unable to parse the value");
    let tia_value = tia_price::get_tia_price().unwrap();
    let staked_value = tia_amount * tia_value;
    // add tia and its value on a single line formatted
    println!("Tia staked: {tia_amount}, Tia value: ${staked_value}");
    staked_value
}
