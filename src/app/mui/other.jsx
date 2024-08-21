import { Badge } from "@mui/material";
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