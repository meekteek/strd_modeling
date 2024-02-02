use crate::tia_price;
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    series::LineSeries,
    style::{RED, WHITE},
};
use std::{
    fs::File,
    io::{self, BufRead},
};

const TOTAL_TVL: f64 = 10000000.0;
const STRD_EMISSION: f64 = 50000.0;

pub fn create_profit_graph(
    daily_tvl_increases: Vec<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("daily_returns.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Return on stakedTia", ("sans-serif", 50))
        .margin(25)
        .x_label_area_size(80)
        .y_label_area_size(100)
        .build_cartesian_2d(0.0..20.0, 0.0..25.0)?;

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
            total_tvl = total_tvl + 3.0 * daily_tvl_increases[x as usize].abs();
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
pub fn create_tvl_graph(daily_tvl_increases: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("Tia staked: {tia_amount}, Tia value: ${staked_value}");
    staked_value
}
