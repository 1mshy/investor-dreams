import { ThemeProvider } from '@mui/material';
import { Component } from 'react';
import { SoftPaper, theme } from '../../mui/theme';

class NotFound extends Component {
    constructor(props) {
        super(props);
    }

    render() {
        const { tickerSymbol } = this.state;
        console.log(tickerSymbol)

        return (
            <ThemeProvider theme={theme}>
                <SoftPaper>
                    <h1>
                        404: Page not found
                    </h1>
                </SoftPaper>
            </ThemeProvider>
        );
    }
}

export default NotFound;