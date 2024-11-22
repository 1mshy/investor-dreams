import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Portfolio.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';
import { Button, Stack, TextField } from '@mui/material';
import SignInSide from '../mui/sign-in-side/SignInSide';
import ApiSide from '../mui/sign-in-side/ApiSide';
import TradingViewWidget from '@/components/widgets/TradingViewWidget';
import HeatMap from '@/components/widgets/HeatMap';
import NewsWidget from '@/components/widgets/NewsWidget';
import HeatMapPopup from '@/components/popups/HeatMapPopup';
import TickerTape from '@/components/tradingview/TickerTape';
import SymbolInfo from '@/components/tradingview/SymbolInfo';
import AdvancedChart from '@/components/tradingview/AdvancedChart';
import CompanyProfile from '@/components/tradingview/CompanyProfile';
import FundamentalData from '@/components/tradingview/FundamentalData';
import TechnicalAnalysis from '@/components/tradingview/TechnicalAnalysis';
import TopStories from '@/components/tradingview/TopStories';
import HotLists from '@/components/tradingview/HotLists';

export default class TradingView extends Component {
    constructor(props) {
        super(props);
        this.state = {
            heatmap: false,

        }
    }

    render() {
        const { heatmap } = this.state;

        // return <div data-tauri-drag-region>
        //     <h1 data-tauri-drag-region>Financial Dashboard</h1>
        //     <TickerTape />
        //     {/* <SymbolInfo /> */}
        //     {/* <AdvancedChart /> */}
        //     {/* <CompanyProfile /> */}
        //     {/* <FundamentalData /> */}
        //     {/* <TechnicalAnalysis /> */}
        //     <TopStories />
        // </div>

        return (
            <div style={{ height: "100vh", width: "100vw", display: "flex" }}>
                <Stack direction="column" style={{ height: "100vh", width: "100vw", display: "flex" }}>
                    <div style={{ minHeight: "1rem" }} data-tauri-drag-region />
                    <h1 data-tauri-drag-region>Global Markets</h1>
                    <div>
                        Heat map:
                        <Button onClick={() => this.setState({ heatmap: true })}>Open Heat Map</Button>
                        <HeatMapPopup open={heatmap} onClick={() => this.setState({ heatmap: false })} />
                    </div>
                    <div>
                        <NewsWidget />
                        <HotLists />
                    </div>
                </Stack>
            </div>
        );
    }
}