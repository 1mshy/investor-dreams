import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Portfolio.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';
import { TextField } from '@mui/material';
import SignInSide from '../mui/sign-in-side/SignInSide';
import ApiSide from '../mui/sign-in-side/ApiSide';

export default class Setup extends Component {
    constructor(props) {
        super(props);
        this.state = {}
    }

    render() {
        return (
            // <div className={"portfolio-whole"} data-tauri-drag-region>
            //     <div className={"homepage-header"} data-tauri-drag-region>
            //         {/* <div style={{ display: "inline-flex" }}>
            //             <StockSearch label="" variant="standard" fullWidth />
            //         </div> */}
            //         <h1 className={"portfolio-title"}>Setup</h1>
            //         <div className={"homepage-nav"} data-tauri-drag-region>
            //             {/* <Link to="/home" className={"homepage-navButton"}>Home</Link> */}
            //             {/* <Link href="/playground" className={"homepage-navButton"}>Pages</Link>
            //             <Link
            //                 href={{
            //                     pathname: '/tickers',
            //                     query: { ticker_symbol: 'AAPL' },
            //                 }}
            //                 passHref
            //             >
            //                 trest ticker</Link> */}
            //             {/* <Button onClick={clear_application_data}>Clear Application Data</Button> */}
            //         </div>
            //     </div>
            //     <div className={"portfolio-home"} data-tauri-drag-region>
            //         <div className={"widgets-container"}  style={{ height: "auto", flex: 7 }}>
            //             <div>
            //             <p>
            //             It seems this build does not have any pre-defined api keys.
            //             Please input the api key(s) below
            //             </p>
            //             </div>
            //             <br/>
            //             <div>
            //             <TextField /></div>
            //         </div>
            //     </div>
            // </div>
            <ApiSide />
        );
    }
}