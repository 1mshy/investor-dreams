import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Portfolio.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';
import { Stack, TextField } from '@mui/material';
import SignInSide from '../mui/sign-in-side/SignInSide';
import ApiSide from '../mui/sign-in-side/ApiSide';
import TradingViewWidget from '@/components/TradingViewWidget';

export default class TradingView extends Component {
    constructor(props) {
        super(props);
        this.state = {}
    }

    render() {
        return (
            <div style={{ height: "100vh", width: "100vw", display: "flex" }}>
                <Stack direction="column" style={{ height: "100vh", width: "100vw", display: "flex" }}>
                    <div style={{ minHeight: "1rem" }} data-tauri-drag-region>
                        
                    </div>
                    <TradingViewWidget />
                </Stack>
            </div>
        );
    }
}