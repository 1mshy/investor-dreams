/**
 * @fileoverview Technical analysis utility functions for stock indicators.
 * Contains functions for analyzing RSI (Relative Strength Index) values
 * and determining market conditions.
 */

/**
 * Analyzes an RSI value and returns a human-readable market condition.
 * 
 * @function
 * @param {number} rsi - The Relative Strength Index value (0-100)
 * @returns {string} Market condition description ("Oversold", "Somewhat Oversold", "Overbought", "Somewhat Overbought", or "Neutral")
 * @example
 * rsi_reading(25); // Returns "Oversold"
 * rsi_reading(50); // Returns "Neutral"
 */
export function rsi_reading(rsi) {
  if (rsi < 30) return "Oversold";
  if (rsi < 40) return "Somewhat Oversold";
  if (rsi > 70) return "Overbought";
  if (rsi > 60) return "Somewhat Overbought";
  return "Neutral";
}

/**
 * Determines if a stock is overbought based on its RSI value.
 * 
 * @function
 * @param {number} rsi - The Relative Strength Index value (0-100)
 * @returns {boolean} True if the RSI indicates overbought conditions (≥70)
 * @example
 * is_overbought(75); // Returns true
 * is_overbought(65); // Returns false
 */
export function is_overbought(rsi) {
  const OVERBOUGHT_THRESHOLD = 70;
  return rsi >= OVERBOUGHT_THRESHOLD;
}

/**
 * Determines if a stock is oversold based on its RSI value.
 * 
 * @function
 * @param {number} rsi - The Relative Strength Index value (0-100)
 * @returns {boolean} True if the RSI indicates oversold conditions (≤30)
 * @example
 * is_oversold(25); // Returns true
 * is_oversold(35); // Returns false
 */
export function is_oversold(rsi) {
  const OVERSOLD_THRESHOLD = 30;
  return rsi <= OVERSOLD_THRESHOLD;
}