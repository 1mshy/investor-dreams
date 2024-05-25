"use client"
import React, { Component } from 'react';

/**
 * css imports
 */
import "@/app/css/Widgets.css"
import { redirect } from 'next/navigation';

export default class Home extends Component {
    constructor(props) {
        super(props);
    }

    componentDidMount() {
        redirect("/main")
    }


    render() {
            <div></div>
    }
}