import { Badge } from "@mui/material";
import { Component } from "react";


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

export class MarketColouredBadge extends Component {
    constructor(props) {
        super(props);
        this.state = {
            colour: "green"
        }
    }



    render() {
        return <Badge color="secondary" badgeContent=" "
            // classes={{ badge: get_badge_colouring(this.state.colour) }}
        >
            {this.props.children}
        </Badge>
    }
}