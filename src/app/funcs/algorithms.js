
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