import React, { useEffect } from 'react';
import "../app/css/Widgets.css";
import { clear_cache, retrieve, store } from './funcs/cache';
import { log } from './funcs/logger';
import { createBrowserRouter, Navigate, RouterProvider } from 'react-router-dom';
import { ToastContainer } from 'react-toastify';
import Portfolio from './pages/Portfolio';
import Home from './pages/Home';
import Playground from './pages/Playground';
import Analysis from './pages/Analysis';
import NotFound from './pages/NotFound';
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
      path: "/analysis",
      element: <Analysis />,
    },
    {
      path: "/portfolio",
      element: <Portfolio />,
    },
    {
      path: "*",
      element: <NotFound />,
    }
  ]);

  const current_version = "1.0.9";
  useEffect(() => {
    const set_version = retrieve("current_version");
    if (!set_version || set_version !== current_version) {
      log("New version detected, clearing cache", set_version, current_version);
      clear_cache();
      store("current_version", current_version);
    }
  }, []);

  return (
    <div>
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
      <RouterProvider router={router} />
    </div>
  );
};

export default BasePage;