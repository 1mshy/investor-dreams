/**
 * 
 * @param {Number[]} prices list of prices
 * @param {Number} period period to calculate RSI (default 14)
 * @returns {Number[]} list of RSI values
 */
export function calculateRSI(prices, period = 14) {
  if (!prices || prices.length < period) {
    throw new Error("Not enough data to calculate RSI.");
  }

  // Step 1: Calculate daily price changes
  const changes = prices.slice(1).map((price, i) => price - prices[i]);

  // Step 2: Separate gains and losses
  const gains = changes.map(change => (change > 0 ? change : 0));
  const losses = changes.map(change => (change < 0 ? Math.abs(change) : 0));

  // Step 3: Calculate SMA of gains and losses for the first period
  const avgGain = gains.slice(0, period).reduce((a, b) => a + b, 0) / period;
  const avgLoss = losses.slice(0, period).reduce((a, b) => a + b, 0) / period;

  // Step 4: Smooth averages using SMA for subsequent periods
  const smoothedRSI = [];
  let currentAvgGain = avgGain;
  let currentAvgLoss = avgLoss;

  for (let i = period; i < gains.length; i++) {
    currentAvgGain = ((currentAvgGain * (period - 1)) + gains[i]) / period;
    currentAvgLoss = ((currentAvgLoss * (period - 1)) + losses[i]) / period;

    // Step 5: Calculate RS and RSI
    const rs = currentAvgLoss === 0 ? 0 : currentAvgGain / currentAvgLoss;
    const rsi = currentAvgLoss === 0 ? 100 : 100 - (100 / (1 + rs));

    smoothedRSI.push(rsi);
  }

  return smoothedRSI;
}

export function calculateRSIFast(prices, period) {
  const changes = prices.slice(1).map((price, i) => price - prices[i]);
  const gains = changes.map(change => (change > 0 ? change : 0));
  const losses = changes.map(change => (change < 0 ? Math.abs(change) : 0));

  const avgGain = gains.slice(0, period).reduce((a, b) => a + b, 0) / period;
  const avgLoss = losses.slice(0, period).reduce((a, b) => a + b, 0) / period;

  const rs = avgLoss === 0 ? 0 : avgGain / avgLoss;
  return avgLoss === 0 ? 100 : 100 - 100 / (1 + rs);
}

export function monteCarloRSI(prices, numSimulations = 1000, forecastDays = 10, period = 14) {
  if (prices.length < period) {
    throw new Error("Not enough data to calculate RSI.");
  }

  // Helper function: Calculate RSI for a series of prices
  function calculateRSI(prices, period) {
    const changes = prices.slice(1).map((price, i) => price - prices[i]);
    const gains = changes.map(change => (change > 0 ? change : 0));
    const losses = changes.map(change => (change < 0 ? Math.abs(change) : 0));

    const avgGain = gains.slice(0, period).reduce((a, b) => a + b, 0) / period;
    const avgLoss = losses.slice(0, period).reduce((a, b) => a + b, 0) / period;

    const rs = avgLoss === 0 ? 0 : avgGain / avgLoss;
    return avgLoss === 0 ? 100 : 100 - 100 / (1 + rs);
  }

  // Step 1: Calculate historical daily returns
  const returns = prices.slice(1).map((price, i) => (price - prices[i]) / prices[i]);

  // Step 2: Calculate mean and standard deviation of returns
  const meanReturn = returns.reduce((a, b) => a + b, 0) / returns.length;
  const stdDevReturn = Math.sqrt(
    returns.map(r => (r - meanReturn) ** 2).reduce((a, b) => a + b, 0) / (returns.length - 1)
  );

  // Step 3: Generate simulated price paths
  const simulations = [];
  for (let i = 0; i < numSimulations; i++) {
    let simulatedPrices = [...prices];
    for (let j = 0; j < forecastDays; j++) {
      const randomShock = meanReturn + stdDevReturn * Math.random() * 2 - 1;
      const nextPrice = simulatedPrices[simulatedPrices.length - 1] * (1 + randomShock);
      simulatedPrices.push(nextPrice);
    }
    simulations.push(simulatedPrices);
  }

  // Step 4: Calculate RSI for the last simulated day of each path
  const finalRSIs = simulations.map(simulatedPrices => calculateRSI(simulatedPrices, period));

  // Step 5: Analyze results
  const averageRSI = finalRSIs.reduce((a, b) => a + b, 0) / finalRSIs.length;
  const rsiDistribution = finalRSIs.reduce((acc, rsi) => {
    const bucket = Math.floor(rsi / 5) * 5; // Group by ranges of 5 (e.g., 30-35)
    acc[bucket] = (acc[bucket] || 0) + 1;
    return acc;
  }, {});

  return {
    averageRSI,
    rsiDistribution,
    simulations: finalRSIs
  };
}

// Example usage
const prices = [44, 44.15, 44.29, 44.21, 44.17, 44.30, 44.29, 44.40, 44.39, 44.51, 44.49, 44.65, 44.73, 44.74, 44.90];
const results = monteCarloRSI(prices);

console.log("Average Predicted RSI:", results.averageRSI);
console.log("RSI Distribution:", results.rsiDistribution);
console.log("Simulated RSIs:", results.simulations);


/**
 * Generic rsi reading that states weather the rsi is overbought or oversold
 * @param {Number} rsi relative strength index value
 * @returns {String} overbought, oversold or neutral ratings
 */
export function rsi_reading(rsi) {
  if (rsi < 30) return "Oversold";
  if (rsi < 40) return "Somewhat Oversold";
  if (rsi > 70) return "Overbought";
  if (rsi > 60) return "Somewhat Overbought";
  return "Neutral";
}

/**
 * 
 * @param {Number} rsi relative strength index value 
 * @returns {Boolean} true if overbought, false otherwise
 */
export function is_overbought(rsi) {
  const OVERBOUGHT_THRESHOLD = 70;
  return rsi >= OVERBOUGHT_THRESHOLD;
}

/**
 * 
 * @param {Number} rsi relative strength index value 
 * @returns {Boolean} true if oversold, false otherwise
 */
export function is_oversold(rsi) {
  const OVERSOLD_THRESHOLD = 30;
  return rsi <= OVERSOLD_THRESHOLD;
}