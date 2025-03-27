import { Component } from 'react';

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Portfolio.css";
import "@/app/css/Widgets.css";
import { open } from '@tauri-apps/plugin-shell';
import { Link } from 'react-router-dom';
import { fetch_common_subreddits } from '../networking/reddit';

export default class Opportunities extends Component {
    constructor(props) {
        super(props);
        this.state = {
            subreddits_data:  {},
        }
    }

    async componentDidMount() {
        this.setState({ subreddits_data: await fetch_common_subreddits() });
    }

    render() {
        const { subreddits_data } = this.state;
        return (
            <div className={"portfolio-whole"} data-tauri-drag-region>
                <div className={"homepage-header"} data-tauri-drag-region>
                    <h1 className={"portfolio-title"}>Educated Gambling</h1>
                    <div className={"homepage-nav"} data-tauri-drag-region>
                        <Link to="/home" className={"homepage-navButton"}>Home</Link>
                        
                    </div>
                </div>
                <div className={"portfolio-home"} data-tauri-drag-region>
                    <div className={"widgets-container"} data-tauri-drag-region style={{ height: "auto", flex: 7 }}>
                        {Object.keys(subreddits_data).map((subreddit) => {
                            const subreddit_data = subreddits_data[subreddit];
                            return <div>
                                <h2>r/{subreddit}</h2>
                                {subreddit_data.map((post, index) => {
                                    return <div className={"news-row"} key={index} style={{ cursor: "pointer" }} onClick={async () => {
                                        await open(post.url);
                                    }}>
                                        {post.title}
                                    </div>
                                })}
                            </div>
                        })}
                    </div>
                </div>
            </div>
        );
    }
}