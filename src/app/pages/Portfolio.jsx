import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Portfolio.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';
import StockWidget from '@/components/widgets/StockWidget';

export default class Portfolio extends Component {
    constructor(props) {
        super(props);
        this.state = {
            book: {
                AAPL: {
                    buys: [{
                        book_value: 13123,
                        timestamp: Date.now(),
                    }, {
                        book_value: 3123,
                        timestamp: Date.now(),
                    }],
                    sells: [{
                        book_value: 3123,
                        timestamp: Date.now(),
                    },
                    {
                        book_value: 3123,
                        timestamp: Date.now(),
                    }],
                },
            }
        }
    }

    render() {
        const { book } = this.state;
        return (
            <div className={"portfolio-whole"} data-tauri-drag-region>
                <div className={"homepage-header"} data-tauri-drag-region>
                    {/* <div style={{ display: "inline-flex" }}>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div> */}
                    <h1 className={"portfolio-title"}>Portfolio</h1>
                    <div className={"homepage-nav"} data-tauri-drag-region>
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
                <div className={"portfolio-home"} data-tauri-drag-region>
                  

                    <div className={"widgets-container"} data-tauri-drag-region style={{ height: "auto", flex: 7 }}>
                        {Object.keys(book).map((symbol) => {
                            return <StockWidget symbol={symbol} size="small" key={symbol} />
                        })}
                    </div>
                </div>
            </div>
        );
    }
}