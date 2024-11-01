import React from 'react';
import Slider from '@mui/material/Slider';

function valuetext(value) {
    return `${Math.pow(10, value)}°C`;
}

const marks = [
    { value: Math.log10(1_000_000), label: '1M' },
    { value: Math.log10(10_000_000), label: '10M' },
    { value: Math.log10(100_000_000), label: '100M' },
    { value: Math.log10(1_000_000_000), label: '1B' },
    { value: Math.log10(10_000_000_000), label: '10B' },
    { value: Math.log10(100_000_000_000), label: '100B' },
    { value: Math.log10(1_000_000_000_000), label: '1T' },
    { value: Math.log10(10_000_000_000_000), label: '10T' },
    { value: Math.log10(100_000_000_000_000), label: '100T' }
];
/**
 * 
 * @param {Object} props
 * @param {Function} props.callback - function will be called back with the following data from the range slider: [left_number, right_number]
 * @returns 
 */
export default function RangeSlider(props) {
    const minimum = 1_000_000;
    const maximum = 100_000_000_000_000;
    const [value, setValue] = React.useState([Math.log10(100_000_000), Math.log10(maximum)]);

    const handleChange = (event, newValue) => {
        setValue(newValue);
        const left_value = Math.pow(10, newValue[0]);
        const right_value = Math.pow(10, newValue[1]);
        if (props.callback) props.callback(left_value, right_value);
    };

    return (
        <Slider
            value={value}
            onChange={handleChange}
            valueLabelDisplay="off"
            aria-labelledby="range-slider"
            getAriaValueText={(value) => valuetext(Math.pow(10, value))}
            marks={marks}
            min={Math.log10(minimum)}
            max={Math.log10(maximum)}
            sx={{ width: 200, }} // Adjust the width to make the slider smaller
        />

    );
}