import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';

export default class Portfolio extends Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div className={"homepage-mainPage"} >
                <div className={"homepage-header"} data-tauri-drag-region>
                    {/* <div style={{ display: "inline-flex" }}>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div> */}
                    <div className={"homepage-nav"} >
                        <Link to="/home" className={"homepage-navButton"}>Home</Link>
                        {/* <Link href="/playground" className={"homepage-navButton"}>Pages</Link>
                        <Link
                            href={{
                                pathname: '/tickers',
                                query: { ticker_symbol: 'AAPL' },
                            }}
                            passHref
                        >
                            trest ticker</Link> */}
                        {/* <Button onClick={clear_application_data}>Clear Application Data</Button> */}
                    </div>
                
                </div>
            </div>
        );
    }
}