import { Badge } from "@mui/material";
import { Component } from "react";
import { is_market_open } from "../funcs/tools";


export const get_badge_ = theme => ({
    margin: {
        margin: theme.spacing.unit * 2
    },
    customBadge: {
        backgroundColor: "#00AFD7",
        color: "white"
    }
});

export function get_badge_colouring(colour) {
    return {
        backgroundColor: "#00AFD7",
        color: "white"
    }
}

export const MarketColouredBadge = (props) => {
    return (
        <Badge badgeContent=""
            sx={{
                '& .MuiBadge-badge': {
                    backgroundColor: is_market_open() ? '#4caf50' : '#e74c3c', // Custom color
                },
            }}
        >
            {props.children}
        </Badge>)
}