import React, { useState, useEffect, useContext } from "react";
import StockWidget from "@/components/widgets/StockWidget";
import { has_favourites, top_favourite_changes } from "@/app/funcs/favourites";
import { nasdaq_sorted_by } from "@/app/networking/stock_api";
import { invoke } from "@tauri-apps/api/core";
import { Link, useNavigate } from "react-router-dom";
import HeatMapPopup from "@/components/popups/HeatMapPopup";
import NewsWidget from "@/components/widgets/NewsWidget";
import { Button, IconButton } from "@mui/material";
import SettingsIcon from "@mui/icons-material/Settings";
import { SettingsContext } from "@/app/settings/SettingsContext";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
// when using `"withGlobalTauri": true`, you may use
// const { isPermissionGranted, requestPermission, sendNotification, } = window.__TAURI__.notification;


import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";

// Memoize StockWidget to prevent unnecessary re-renders
const MemoizedStockWidget = React.memo(StockWidget);

const Home = () => {
  const { settings } = useContext(SettingsContext);
  const [username, setUsername] = useState("");
  const [top_3_changes, setTop3Changes] = useState([]);
  const [bottom_3_changes, setBottom3Changes] = useState([]);
  const [top_favs, setTopFavs] = useState([]);
  const [heatmap, setHeatmap] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      try {
        // Only access browser-specific APIs here
        const username = await invoke("get_username");
        setUsername(username);

        const top_favs = await top_favourite_changes();
        setTopFavs(top_favs);

        const top_500 = (await nasdaq_sorted_by("marketCap")).slice(0, 500);
        const top_500_change = await nasdaq_sorted_by("pctchange", top_500);
        setTop3Changes(top_500_change.slice(0, 3));
        setBottom3Changes(
          top_500_change
            .slice(top_500_change.length - 4, top_500_change.length - 1)
            .reverse(),
        );
                // Do you have permission to send a notification?
        let permissionGranted = await isPermissionGranted();

        // If not we need to request it
        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === 'granted';
        }
        console.log(permissionGranted);
        // Once permission has been granted we can send the notification
        if (permissionGranted) {
          sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });
        }
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    };

    fetchData();
  }, []);

  return (
    <div className={"homepage-mainPage"}>
      <div className={"homepage-header"} data-tauri-drag-region>
        <h1 className={"homepage-title"} style={{ display: "inline-flex" }}>
          Welcome, {username}
        </h1>
        <div className={"homepage-nav"}>
          <Link to="/playground" className={"homepage-navButton"}>
            Playground
          </Link>
          <Link to="/portfolio" className={"homepage-navButton"}>
            Portfolio
          </Link>
          <Link to="/analysis" className={"homepage-navButton"}>
            Analysis
          </Link>
          <IconButton component={Link} to="/settings">
            <SettingsIcon />
          </IconButton>
        </div>
      </div>
      <div className={"homepage-content"} data-tauri-drag-region>
        {has_favourites() && (
          <div className={"homepage-columns"}>
            <h3>Favourites:</h3>
            <div className={"homepage-favourties"}>
              {top_favs.map((ticker_symbol) => (
                <MemoizedStockWidget
                  key={ticker_symbol}
                  symbol={ticker_symbol}
                  size={
                    settings.Home_Page?.settings?.default_widget_size.value ||
                    "small"
                  }
                />
              ))}
            </div>
          </div>
        )}
        <div className={"homepage-columns"}>
          <h3>Best Performing</h3>
          <div className={"top3-list"}>
            {top_3_changes.map((ticker_symbol) => (
              <MemoizedStockWidget
                key={ticker_symbol}
                symbol={ticker_symbol}
                size={
                  settings.Home_Page?.settings?.default_widget_size.value ||
                  "small"
                }
              />
            ))}
          </div>
        </div>
        <div className={"homepage-columns"}>
          <h3>Worst performing</h3>
          <div className={"bottom3-list"}>
            {bottom_3_changes.map((ticker_symbol) => (
              <MemoizedStockWidget
                key={ticker_symbol}
                symbol={ticker_symbol}
                size={
                  settings.Home_Page?.settings?.default_widget_size.value ||
                  "small"
                }
              />
            ))}
          </div>
        </div>
        {settings.TradingView_Widget?.settings?.show_tradingview_on_home
          .value && (
          <div className={"homepage-columns"}>
            <h3>Extra</h3>
            <div className={"homepage-favourties"}>
              <Button onClick={() => setHeatmap(true)}>Open Heat Map</Button>
              <HeatMapPopup open={heatmap} onClick={() => setHeatmap(false)} />
              <NewsWidget />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Home;
