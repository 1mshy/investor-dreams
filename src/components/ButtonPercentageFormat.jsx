/**
 * @fileoverview Button wrapper for percentage format display component.
 * Provides a clickable button that displays formatted percentage changes.
 */

import { Button } from "@mui/material";
import "../app/css/Formatting.css";
import PercentageFormat from "./PercentageFormat";

/**
 * A button component that displays a formatted percentage change.
 * Wraps the PercentageFormat component in a Material-UI Button.
 * 
 * @component
 * @param {Object} props - Component props
 * @param {number} props.percent_change - The percentage change value to display
 * @param {string} props.timeset - Time period for the percentage change (e.g., "1D", "1W", "1M")
 * @param {Function} props.func - Click handler function for the button
 * @returns {JSX.Element} Button containing formatted percentage display
 * @example
 * <ButtonPercentageFormat 
 *   percent_change={5.25} 
 *   timeset="1D" 
 *   func={() => console.log('clicked')} 
 * />
 */
const ButtonPercentageFormat = ({ percent_change, timeset, func }) => {
  return (
    <div>
      <Button onClick={func}>
        <PercentageFormat percent_change={percent_change} timeset={timeset} />
      </Button>
    </div>
  );
};

export default ButtonPercentageFormat;
