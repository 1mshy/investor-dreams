"use client"
import React, { Component } from 'react';
import Link from 'next/link';
import "../css/Homepage.css"
import { invoke } from '@tauri-apps/api';

export default class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            username: ""
        }
    }

    async componentDidMount() {
        invoke("get_username").then((response) => {
            this.setState({ username: response });
        });
    }

    render() {
        const { username } = this.state;
        return (
            <div className={"mainPage"}>
                <header className={"header"}>
                    <h1 className={"title"}>Investor Dreams</h1>
                    <nav className={"nav"}>
                        <Link href="/playground" className={"navButton"}>Top 10</Link>
                        <Link href="/" className={"navButton"}>Top 300</Link>
                    </nav>
                </header>
                <div className={"homepage-content"}>
                    <h2 className={"welcomeText"}>Welcome back {username}</h2>
                </div>
            </div>
        );
    }
}
