use rand::Rng;
use rand_distr::{num_traits::abs, Distribution, Normal};
use tauri::command;

use crate::algorithms;

#[tauri::command]
pub async fn rsi(prices: Vec<f64>, period: usize) -> Result<Vec<f64>, String> {
    if prices.len() < period {
        return Err("Not enough data to calculate RSI.".to_string());
    }

    let changes: Vec<f64> = prices.windows(2).map(|w| w[1] - w[0]).collect();
    let gains: Vec<f64> = changes
        .iter()
        .map(|&change| if change > 0.0 { change } else { 0.0 })
        .collect();
    let losses: Vec<f64> = changes
        .iter()
        .map(|&change| if change < 0.0 { change.abs() } else { 0.0 })
        .collect();

    let period_f64 = period as f64;
    let avg_gain: f64 = gains.iter().take(period).sum::<f64>() / period_f64;
    let avg_loss: f64 = losses.iter().take(period).sum::<f64>() / period_f64;
    let mut smooth_rsi: Vec<f64> = vec![f64::NAN, period_f64 - 1.0];
    let mut current_avg_gain = avg_gain;
    let mut current_avg_loss = avg_loss;

    for i in period..gains.len() {
        current_avg_gain = (current_avg_gain * (period_f64 - 1.0) + gains[i]) / period_f64;
        current_avg_loss = (current_avg_loss * (period_f64 - 1.0) + losses[i]) / period_f64;

        let rs = if current_avg_loss == 0.0 {
            0.0
        } else {
            current_avg_gain / current_avg_loss
        };
        let rsi = if current_avg_loss == 0.0 {
            100.0
        } else {
            100.0 - (100.0 / (1.0 + rs))
        };
        smooth_rsi.push(rsi)
    }
    Ok(smooth_rsi)
}

#[tauri::command]
pub async fn monte_carlo_rsi(
    prices: Vec<f64>,
    num_simulations: usize,
    forecast_days: usize,
    period: usize,
) -> Result<(f64, Vec<(i32, usize)>, Vec<f64>), String> {
    if prices.len() < period {
        return Err("Not enough data to calculate RSI.".to_string());
    }

    // Helper function to calculate RSI
    fn calculate_rsi(prices: &[f64], period: usize) -> f64 {
        let changes: Vec<f64> = prices.windows(2).map(|w| w[1] - w[0]).collect();
        let gains: Vec<f64> = changes
            .iter()
            .map(|&c| if c > 0.0 { c } else { 0.0 })
            .collect();
        let losses: Vec<f64> = changes
            .iter()
            .map(|&c| if c < 0.0 { -c } else { 0.0 })
            .collect();

        let avg_gain: f64 = gains.iter().take(period).sum::<f64>() / period as f64;
        let avg_loss: f64 = losses.iter().take(period).sum::<f64>() / period as f64;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }

    // Step 1: Calculate historical daily returns
    let returns: Vec<f64> = prices.windows(2).map(|w| (w[1] - w[0]) / w[0]).collect();

    // Step 2: Calculate mean and standard deviation of returns
    let mean_return: f64 = returns.iter().copied().sum::<f64>() / returns.len() as f64;
    let std_dev_return: f64 = (returns
        .iter()
        .map(|&r| (r - mean_return).powi(2))
        .sum::<f64>()
        / (returns.len() - 1) as f64)
        .sqrt();

    // Step 3: Simulate price paths
    let mut rng = rand::thread_rng();
    let mut simulations = Vec::new();
    let normal_dist = Normal::new(mean_return, std_dev_return).unwrap();

    for _ in 0..num_simulations {
        let mut simulated_prices = prices.clone();
        for _ in 0..forecast_days {
            let random_shock = normal_dist.sample(&mut rng);
            let last_price = *simulated_prices.last().unwrap();
            let next_price = last_price * (1.0 + random_shock);
            simulated_prices.push(next_price);
        }
        simulations.push(simulated_prices);
    }

    // Step 4: Calculate RSI for each simulated path
    let final_rsi_values: Vec<f64> = simulations
        .iter()
        .map(|simulated_prices| calculate_rsi(simulated_prices, period))
        .collect();

    // Step 5: Analyze results
    let average_rsi: f64 = final_rsi_values.iter().sum::<f64>() / final_rsi_values.len() as f64;

    // Group RSI values into ranges for distribution
    let mut rsi_distribution = std::collections::HashMap::new();
    for &rsi in &final_rsi_values {
        let bucket = (rsi as i32 / 5) * 5; // Group by ranges of 5
        *rsi_distribution.entry(bucket).or_insert(0) += 1;
    }

    let rsi_distribution: Vec<(i32, usize)> = rsi_distribution.into_iter().collect();

    Ok((average_rsi, rsi_distribution, final_rsi_values))
}
