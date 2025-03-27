import React, { useEffect, useState } from 'react';
import { createBrowserRouter, Navigate, RouterProvider } from 'react-router-dom';
import { ToastContainer } from 'react-toastify';
import "../app/css/Widgets.css";
import { clear_cache, complex_retrieve, complex_store } from './networking/cache';
import Background from './mui/Background';
import Analysis from './pages/Analysis';
import Home from './pages/Home';
import NotFound from './pages/NotFound';
import Opportunities from './pages/Opportunities';
import Playground from './pages/Playground';
import Portfolio from './pages/Portfolio';
import Setup from './pages/Setup';
import TradingView from './pages/TradingView';
import { SettingsProvider } from './settings/SettingsContext';
import WidgetSettingsPanel from './settings/WidgetSettingsPanel';
const BasePage = () => {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <Navigate to="/home" />,
    },
    {
      path: "/home",
      element: <Home />,
    },
    {
      path: "/playground",
      element: <Playground />,
    },
    {
      path: "/analysis",
      element: <Analysis />,
    },
    {
      path: "/opportunities",
      element: <Opportunities />,
    },
    {
      path: "/portfolio",
      element: <Portfolio />,
    },
    {
      path: "/tradingview",
      element: <TradingView />,
    },
    {
      path: "/settings",
      element: <WidgetSettingsPanel />,
    },
    {
      path: "*",
      element: <NotFound />,
      errorElement: <div>bad</div>,
    }
  ]);

  const current_version = "1.0.20";
  const [errors, set_errors] = useState(true);
  const [has_checked, set_has_checked] = useState(false);
  useEffect(() => {
    const become_async = async () => {
      const set_version = await complex_retrieve("current_version");
      if (!set_version || set_version !== current_version) {
        console.log("New version detected, clearing cache", set_version, current_version);
        clear_cache();
        await complex_store("current_version", current_version);
      }
      // api keys no longer needed
      // let keys = format_api_keys(await invoke("get_api_keys")); // checks for keys built into the binary as a string
      // if (!keys) {
      //   keys = await complex_retrieve("user_api_keys"); // checks for keys set by the user, already in an array
      //   if (!keys) {
      //     console.log(keys)
      //     console.error(`No twelve data api key found!!!`)
      //     set_has_checked(true)
      //     return
      //   }
      // }
      // set_api_keys(keys)
      set_errors(false)
      set_has_checked(true)
    };
    become_async();
  }, []);

  return (
    <SettingsProvider>
      <Background router={router}>
        <ToastContainer
          position="bottom-right"
          autoClose={3000}
          limit={4}
          hideProgressBar={false}
          newestOnTop
          closeOnClick
          rtl={false}
          pauseOnFocusLoss
          draggable
          pauseOnHover
          theme="dark"
        />
        {!errors && <RouterProvider router={router} />}
        {errors && has_checked && <div> <Setup /> </div>}
      </Background>
    </SettingsProvider>
  );
};

export default BasePage;