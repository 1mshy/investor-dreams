import React from "react";
import ReactDOM from "react-dom/client";
import BasePage from "./app/app";
import { SettingsProvider } from './app/settings/SettingsContext';

import "@/app/css/globals.css";
import 'react-toastify/dist/ReactToastify.css';

ReactDOM.createRoot(document.getElementById("application")).render(
    <React.StrictMode>
        <SettingsProvider>
            <BasePage />
        </SettingsProvider>
    </React.StrictMode>
);
