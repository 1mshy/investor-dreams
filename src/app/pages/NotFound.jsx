import { ThemeProvider } from '@mui/material';
import { Component } from 'react';
import { Link } from 'react-router-dom';
import { theme } from '@/app/mui/theme';
import { BackGroundPaper } from '@/app/mui/theme';

class NotFound extends Component {
    constructor(props) {
        super(props);
    }

    render() {
        const { tickerSymbol } = this.state;
        console.log(tickerSymbol)

        return (
            <ThemeProvider theme={theme}>
                <BackGroundPaper>
                    <h1>Investor Dreams</h1>
                    <h2>404: Page Not Found</h2>
                    <p>Sorry, the page you are looking for does not exist. You will be redirected to the homepage in a few seconds.</p>
                    <p>Fun fact! It is impossible to get here without altering the code. Good job, you broke everything.</p>
                    <Link to="/home">Click here to go back to the main page</Link>
                </BackGroundPaper>
            </ThemeProvider>
        );
    }
}

export default NotFound;