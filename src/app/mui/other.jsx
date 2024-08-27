import React, { useEffect, useState } from 'react';
import { TextField, InputAdornment, Select, MenuItem, Badge } from '@mui/material';

import { is_market_open } from "../funcs/tools";

export const MarketColouredBadge = (props) => {
    return (
        <Badge badgeContent=""
            sx={{
                '& .MuiBadge-badge': {
                    backgroundColor: is_market_open() ? '#4caf50' : '#e74c3c', // Custom color
                    zIndex: 0,
                },
            }}
        >
            {props.children}
        </Badge>)
}

const suffixes = [
    { label: ' ', factor: 1 },
    { label: 'K', factor: 1_000 },
    { label: 'M', factor: 1_000_000 },
    { label: 'B', factor: 1_000_000_000 },
    { label: 'T', factor: 1_000_000_000_000 },
];


export const CurrencyTextField = (props) => {
    const { on_change } = props;
    const [value, set_value] = useState(props.value || '');
    const [suffix_index, set_suffix_index] = useState(0);

    const handle_value_change = (event) => {
        let input_value = parseFloat(event.target.value) || '';
        console.log(input_value)
        if (input_value) {
            let new_suffix_index = suffix_index;

            while (input_value >= 1000 && new_suffix_index < suffixes.length - 1) {
                input_value /= 1000;
                new_suffix_index += 1;
            }
            console.log(input_value)
            set_value(input_value);
            set_suffix_index(new_suffix_index);

            const factored_value = input_value * suffixes[new_suffix_index].factor;
            console.log(factored_value)
            // on_change(factored_value);
        } else {
            set_value('');
            // on_change('');
        }
    };

    const handle_suffix_change = (event) => {
        const new_suffix_index = suffixes.findIndex(
            (suffix) => suffix.label === event.target.value
        );
        set_suffix_index(new_suffix_index);
    };

    useEffect(() => {
        console.log(value)
        console.log(suffix_index)
        if (value >= 1000 && suffix_index < 2) {
            let input_value = value;
            let new_suffix_index = suffix_index;

            while (input_value >= 1000 && new_suffix_index < suffixes.length - 1) {
                input_value /= 1000;
                new_suffix_index += 1;
            }
            console.log(input_value)
            set_value(input_value);
            set_suffix_index(new_suffix_index);
        }
        console.log(value)
    }, []);

    useEffect(() => {
        const factored_value = value * suffixes[suffix_index].factor;
        on_change(factored_value)
    }, [value, suffix_index]);

    return (
        <TextField
            // {...props}
            value={value}
            onChange={handle_value_change}
            InputProps={{
                endAdornment: (
                    <InputAdornment position="end">
                        <Select
                            value={suffixes[suffix_index].label}
                            onChange={handle_suffix_change}
                            variant="standard"
                            disableUnderline
                        >
                            {suffixes.map((suffix) => (
                                <MenuItem key={suffix.label} value={suffix.label}>
                                    {suffix.label}
                                </MenuItem>
                            ))}
                        </Select>
                    </InputAdornment>
                ),
            }}
        />
    );
};