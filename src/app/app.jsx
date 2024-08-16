"use client"
import React, { useEffect } from 'react';
/**
 * global css imports
 */
import "../app/css/Widgets.css"
import { clear_cache, retrieve, store } from './funcs/cache';
import { log } from './funcs/logger';
import { createBrowserRouter, Navigate, redirect, RouterProvider, useNavigate } from 'react-router-dom';
import Home from './ui_components/pages/Home';
import Playground from './ui_components/pages/Playground';
import Predictions from './ui_components/pages/Predictions';
import NotFound from './not-found';

const BasePage = () => {
    const router = createBrowserRouter([
        {
            path: "/",
            element: <Navigate to="/home" />,
          },
          {
            path: "/home",
            element: <Home />,
        errorElement: <NotFound />,
          },
          {
            path: "/playground",
            element: <Playground />,
          },
          {
            path: "/predictions",
            element: <Predictions />,
          },

      ]);
    // useEffect(() => {
    //     invoke("set_base_size").then((response) => {
    //         console.log("All windows:", response);
    //     });
    // });
    console.log("home page loading")
    const current_version = "1.0.4"
    useEffect(() => {
        const set_version = retrieve("current_version")
        if (!set_version || set_version !== current_version) {
            log("New version detected, clearing cache", set_version, current_version)
            clear_cache()
            store("current_version", current_version)
        }
        // router.push("/home");
    }, []);

    return <div style={{color: "black"}}>
        <RouterProvider router={router}/>
    </div>;
};

export default BasePage;
