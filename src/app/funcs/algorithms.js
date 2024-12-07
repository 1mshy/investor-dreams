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
/**
 * Generic rsi reading that states weather the rsi is overbought or oversold
 * @param {Number} rsi relative strength index value
 * @returns {String} overbought, oversold or neutral ratings
 */
export function rsi_reading(rsi) {
  if(rsi < 30) return "Oversold";
  if(rsi < 40) return "Somewhat Oversold";
  if(rsi > 70) return "Overbought";
  if(rsi > 60) return "Somewhat Overbought";
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