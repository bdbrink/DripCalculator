use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod data_provider;
mod drip_engine;
mod models;

use data_provider::YahooFinanceProvider;
use drip_engine::DripCalculator;
use models::{Investment, PerformanceComparison};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 DRIP Calculator - Dividend Reinvestment Plan Analyzer\n");

    // Example usage
    let ticker = "AAPL";
    let initial_investment = 10000.0;
    let years_back = 15;

    let end_date = chrono::Local::now().naive_local().date();
    let start_date = end_date - Duration::days(365 * years_back);

    println!("Analyzing {} from {} to {}", ticker, start_date, end_date);
    println!("Initial investment: ${:.2}\n", initial_investment);

    // Initialize data provider
    let provider = YahooFinanceProvider::new();

    // Fetch historical data
    println!("📊 Fetching historical data...");
    let stock_data = provider.fetch_historical_data(ticker, start_date, end_date).await?;
    let spy_data = provider.fetch_historical_data("SPY", start_date, end_date).await?;
    let qqq_data = provider.fetch_historical_data("QQQ", start_date, end_date).await?;

    // Calculate DRIP performance
    println!("💰 Calculating DRIP returns...\n");
    let calculator = DripCalculator::new();
    
    let stock_result = calculator.calculate_drip(&stock_data, initial_investment)?;
    let spy_result = calculator.calculate_drip(&spy_data, initial_investment)?;
    let qqq_result = calculator.calculate_drip(&qqq_data, initial_investment)?;

    // Display results
    print_results(ticker, &stock_result, &spy_result, &qqq_result);

    Ok(())
}

fn print_results(
    ticker: &str,
    stock: &Investment,
    spy: &Investment,
    qqq: &Investment,
) {
    println!("═══════════════════════════════════════════════════════");
    println!("                    FINAL RESULTS                      ");
    println!("═══════════════════════════════════════════════════════\n");

    println!("📈 {} Performance:", ticker);
    println!("   Final Value:        ${:.2}", stock.final_value);
    println!("   Total Shares:       {:.4}", stock.total_shares);
    println!("   Total Return:       {:.2}%", stock.total_return_pct);
    println!("   Annualized Return:  {:.2}%", stock.annualized_return_pct);
    println!("   Dividends Received: ${:.2}\n", stock.total_dividends);

    println!("📊 SPY (S&P 500) Performance:");
    println!("   Final Value:        ${:.2}", spy.final_value);
    println!("   Total Shares:       {:.4}", spy.total_shares);
    println!("   Total Return:       {:.2}%", spy.total_return_pct);
    println!("   Annualized Return:  {:.2}%\n", spy.annualized_return_pct);

    println!("🚀 QQQ (Nasdaq-100) Performance:");
    println!("   Final Value:        ${:.2}", qqq.final_value);
    println!("   Total Shares:       {:.4}", qqq.total_shares);
    println!("   Total Return:       {:.2}%", qqq.total_return_pct);
    println!("   Annualized Return:  {:.2}%\n", qqq.annualized_return_pct);

    println!("═══════════════════════════════════════════════════════");
    println!("                   COMPARATIVE ANALYSIS                ");
    println!("═══════════════════════════════════════════════════════\n");

    let outperformance_spy = stock.final_value - spy.final_value;
    let outperformance_qqq = stock.final_value - qqq.final_value;

    println!("{} vs SPY: ${:.2} ({:.2}%)", 
             ticker, 
             outperformance_spy,
             (outperformance_spy / spy.final_value) * 100.0);
    
    println!("{} vs QQQ: ${:.2} ({:.2}%)", 
             ticker, 
             outperformance_qqq,
             (outperformance_qqq / qqq.final_value) * 100.0);
}