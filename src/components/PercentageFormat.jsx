/**
 * @fileoverview Percentage format display component.
 * Renders percentage changes with color coding and optional time period.
 */

import { useEffect, useState } from "react";
import "../app/css/Formatting.css";

/**
 * Displays a formatted percentage change with color coding.
 * Green for positive changes, red for negative changes.
 * 
 * @component
 * @param {Object} props - Component props
 * @param {number} props.percent_change - The percentage change value to display
 * @param {string} [props.timeset] - Optional time period for the percentage change
 * @returns {JSX.Element} Formatted percentage display with optional time period
 * @example
 * <PercentageFormat percent_change={-2.5} timeset="1D" />
 */
const PercentageFormat = ({ percent_change, timeset }) => {
  const [isPositive, setIsPositive] = useState(percent_change >= 0);

  useEffect(() => {
    setIsPositive(percent_change >= 0);
  }, [percent_change]);

  if(isNaN(percent_change)) return <div></div>;

  return (
    <div
      className={"percent-container"}
      style={{ color: isPositive ? "#4caf50" : "#e74c3c" }}
    >
      <div>
        {`${isPositive ? "+" : ""}${Number(percent_change).toFixed(2)}%`}
      </div>
      <div className={"term"}>{timeset ? timeset : ""}</div>
    </div>
  );
};

export default PercentageFormat;
