import { ThemeProvider } from "@mui/material";
import React, { useEffect } from "react";
import { Link, useNavigate, useLocation } from "react-router-dom";
import { theme } from "@/app/mui/theme";
import { BackGroundPaper } from "@/app/mui/theme";

const NotFound = () => {
  const navigate = useNavigate();
  const location = useLocation();

  // Log the current location for debugging purposes
  console.log("Current location:", location);

  useEffect(() => {
    // Redirect to the homepage after 5 seconds
    const timeout = setTimeout(() => {
      navigate("/home");
    }, 5000);

    // Cleanup timeout on unmount
    return () => clearTimeout(timeout);
  }, [navigate]);

  return (
    <div>
      <ThemeProvider theme={theme}>
        <BackGroundPaper>
          <h1 data-tauri-drag-region>Investor Dreams</h1>
          <h2 data-tauri-drag-region>404: Page Not Found</h2>
          <p>
            Sorry, the page you are looking for does not exist. You will be
            redirected to the homepage in a few seconds.
          </p>
          <p>
            Fun fact! It is impossible to get here without altering the code.
            Good job, you broke everything.
          </p>
          <p>Current Location: {location.pathname}</p>
          <Link to="/home">Click here to go back to the main page</Link>
        </BackGroundPaper>
      </ThemeProvider>
    </div>
  );
};

export default NotFound;
