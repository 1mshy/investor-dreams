import React from "react";
import ReactDOM from "react-dom/client";
import BasePage from "./app/app";

import "@/app/css/globals.css";
import { ToastContainer } from "react-toastify";
import 'react-toastify/dist/ReactToastify.css';

ReactDOM.createRoot(document.getElementById("application")).render(
        <BasePage />
);
