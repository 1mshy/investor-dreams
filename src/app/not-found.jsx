import React from "react";
import { Link } from "react-router-dom";

export default function NotFound() {
    return <div>
        <h1>Investor Dreams</h1>
        <h2>404: Page Not Found</h2>
        <p>Sorry, the page you are looking for does not exist. You will be redirected to the homepage in a few seconds.</p>
        <Link to="/home">Click here to go back to the main page</Link>
    </div >
}