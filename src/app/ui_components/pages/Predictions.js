"use client"

import { theme } from "@/app/mui/theme";
import { Button, ThemeProvider } from '@mui/material';
import { Component } from "react";

import "@/app/css/Playground.css";
import "../../css/Homepage.css";

export default class Predictions extends Component {
    constructor(props) {
        super(props);
    }

    render() {
        return <ThemeProvider theme={theme}>
            <div className={"playground"}>
                <Button>
                    Prediction
                </Button>
            </div>
        </ThemeProvider>
    }
}