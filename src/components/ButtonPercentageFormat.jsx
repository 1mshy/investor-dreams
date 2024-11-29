import { Button } from "@mui/material";
import "../app/css/Formatting.css";
import PercentageFormat from "./PercentageFormat";

/**
 *
 * @param {Number} percent_change change in percent that will be shown
 * @param {String} timeset the time period that the percentage change is over
 * @param {function} func the function that will be called when the button is clicked
 * @desc wrapper for percentage format that has a button around it
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
