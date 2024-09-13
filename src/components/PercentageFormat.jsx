import { useEffect, useState } from 'react';
import "../app/css/Formatting.css"

/**
 * 
 * @param {Number} percent_change change in percent that will be shown
 * @param {String} timeset the time period that the percentage change is over 
 * @desc affects colour if positive or negative (green or red) and adds + or - to the percentage
 */
const PercentageFormat = ({ percent_change, timeset }) => {
    const [isPositive, setIsPositive] = useState(percent_change >= 0);
    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(percent_change >= 0);
    }, [percent_change]);
    return <div className={"percent-container"} style={{ color: isPositive ? '#4caf50' : '#e74c3c' }}>
        <div >
            {`${isPositive ? "+" : ""}${Number(percent_change).toFixed(2)}%`}
        </div>
        <div className={"term"}>
           {timeset ? timeset : ""}
        </div>
    </div>
}

export default PercentageFormat;