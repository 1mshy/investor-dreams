/**
 * @fileoverview Technical indicator graph component using Chart.js.
 * Displays technical indicators like RSI in an interactive line chart.
 */

import React, { Component } from 'react';
import { user_settings } from '@/app/config/settings';
import {
    get_all_prices,
    get_five_year_prices,
    get_month_prices,
    get_ten_year_prices,
    get_year_prices,
    get_ytd_prices,
} from '@/app/funcs/historical_pricing';
import { fetch_yahoo_timeset, percentage_change } from '@/app/funcs/stock_api';
import { format_currency, format_number, format_percentage, unformat_number } from '@/app/funcs/tools';
import {
    CategoryScale,
    Chart as ChartJS,
    Filler,
    Legend,
    LinearScale,
    LineElement,
    PointElement,
    Title,
    Tooltip,
} from 'chart.js';
import { Line } from 'react-chartjs-2';
import { invoke } from '@tauri-apps/api/core';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler);

/**
 * A React component that renders technical indicators using Chart.js.
 * Currently supports RSI (Relative Strength Index) visualization.
 * 
 * @component
 * @extends {Component}
 * @param {Object} props - Component props
 * @param {string} props.symbol - Stock ticker symbol
 * @param {string} [props.size='small'] - Chart size ('small', 'big', 'full')
 * @param {string} [props.timeset='1Y'] - Time period to display
 * @param {string[]} props.indicators - Array of indicator names to display
 */
class IndicatorGraph extends Component {
    /**
     * Initializes the component with empty state.
     * 
     * @param {Object} props - Component props
     */
    constructor(props) {
        super(props);
        this.state = {
            historical_data: [],
            chart_data: null,
            chart_options: null,
            dimensions: { width: '300px', height: '200px' },
            indicators: {},
        };
    }

    /**
     * Calculates chart dimensions.
     * 
     * @returns {Object} Width and height dimensions
     */
    calculateDimensions() {
        return { width: '100%', height: '100%' };
    }

    /**
     * Sets up the chart with indicator data.
     * Currently calculates and displays RSI.
     * 
     * @async
     * @param {Array<number>} prices - Array of historical prices
     */
    async setupChart(prices) {
        const smoothend_prices = prices.map((price, index) => 
            isNaN(price) ? (prices[index - 1] + prices[index + 1]) / 2 : price
        );
        
        const rsi_values = await invoke("rsi", {
            prices: smoothend_prices, 
            period: 14,
        });

        const data = {
            labels: rsi_values,
            datasets: [
                {
                    backgroundColor: (context) => {
                        const chart = context.chart;
                        const { ctx, chartArea } = chart;
                        if (!chartArea) return '';
                        const gradient = ctx.createLinearGradient(0, chartArea.top, 0, chartArea.bottom);
                        gradient.addColorStop(0, 'rgba(75, 192, 192, 0.2)');
                        gradient.addColorStop(0.5, 'rgba(75, 192, 192, 0.1)');
                        gradient.addColorStop(1, 'rgba(75, 192, 192, 0)');
                        return gradient;
                    },
                    label: this.props.indicators[0],
                    data: rsi_values,
                    fill: true,
                    pointRadius: 0,
                    pointHoverRadius: 0,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.4,
                    spanGaps: true,
                },
            ],
        };

        const options = {
            responsive: true,
            scales: {
                x: { display: false },
                y: { display: false, beginAtZero: false },
            },
            plugins: {
                legend: { display: false },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    enabled: false,
                    external: this.renderCustomTooltip.bind(this),
                },
            },
        };

        this.setState({ chart_data: data, chart_options: options });
    }

    /**
     * Renders a custom tooltip for the indicator chart.
     * 
     * @param {Object} context - Chart tooltip context
     */
    renderCustomTooltip(context) {
        const tooltipModel = context.tooltip;
        let tooltipEl = document.getElementById('custom-tooltip');

        if (!tooltipEl) {
            tooltipEl = document.createElement('div');
            tooltipEl.id = 'custom-tooltip';
            tooltipEl.style.position = 'absolute';
            tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
            tooltipEl.style.color = 'white';
            tooltipEl.style.padding = '5px 10px';
            tooltipEl.style.borderRadius = '5px';
            tooltipEl.style.pointerEvents = 'none';
            tooltipEl.style.transition = 'opacity 0.2s ease';
            document.body.appendChild(tooltipEl);
        }

        if (tooltipModel.opacity === 0) {
            tooltipEl.style.opacity = 0;
            return;
        }

        if (tooltipModel.body) {
            const value = tooltipModel.dataPoints[0].raw;
            const formatDate = (dateString) =>
                new Date(dateString).toLocaleDateString('en-US', {
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric',
                });
            const formatTime = (dateString) =>
                new Date(dateString).toLocaleTimeString('en-US', {
                    hour: 'numeric',
                    minute: 'numeric',
                    second: 'numeric',
                });
            const index = tooltipModel.dataPoints[0].dataIndex;
            const date = this.state.historical_data[this.state.historical_data.length - 1 - index]?.datetime;
            tooltipEl.innerHTML = `
                <div><strong>Value:</strong> ${format_number(value)}</div>
                ${date ? `<div><strong>Date:</strong> ${formatDate(date)}</div>` : ''}
                ${this.props.timeset === "D" ? `<div>${formatTime(date)}</div>` : ''}
            `;
        }

        const position = context.chart.canvas.getBoundingClientRect();
        tooltipEl.style.left = position.left + window.pageXOffset + tooltipModel.caretX + 'px';
        tooltipEl.style.top = position.top + window.pageYOffset + tooltipModel.caretY + 'px';
        tooltipEl.style.opacity = 1;
    }

    /**
     * Fetches historical data and calculates indicators.
     * 
     * @async
     */
    async fetchData() {
        const { symbol, timeset } = this.props;
        if (!symbol) return;

        try {
            const yahoo_timeset = await fetch_yahoo_timeset(symbol, timeset);
            const historical = yahoo_timeset.data;
            this.setState({ historical_data: historical });

            switch (timeset) {
                case 'D':
                    this.setupChart(get_all_prices(historical));
                    break;
                case 'M':
                    this.setupChart(get_month_prices(historical));
                    break;
                case 'YTD':
                    this.setupChart(get_ytd_prices(historical));
                    break;
                case 'Y':
                    this.setupChart(get_year_prices(historical));
                    break;
                case '5Y':
                    this.setupChart(get_five_year_prices(historical));
                    break;
                case '10Y':
                    this.setupChart(get_ten_year_prices(historical));
                    break;
                default:
                    this.setupChart(get_all_prices(historical));
                    break;
            }
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    }

    /**
     * Lifecycle method: Fetches data after component mounts.
     */
    componentDidMount() {
        const { size } = this.props;
        this.fetchData();
        this.setState({ dimensions: this.calculateDimensions(size) });
    }

    /**
     * Lifecycle method: Updates data when props change.
     * 
     * @param {Object} prevProps - Previous component props
     */
    componentDidUpdate(prevProps) {
        const { symbol, size, timeset } = this.props;
        if (prevProps.symbol !== symbol || prevProps.size !== size || prevProps.timeset !== timeset) {
            this.fetchData();
            this.setState({ dimensions: this.calculateDimensions(size) });
        }
    }

    /**
     * Renders the indicator chart.
     * 
     * @returns {JSX.Element} The rendered chart or loading indicator
     */
    render() {
        const { chart_data, chart_options, dimensions } = this.state;

        if (!chart_data || !chart_options) return <div>Loading Chart...</div>;

        return (
            <div style={dimensions}>
                <Line data={chart_data} options={chart_options} />
            </div>
        );
    }
}

export default IndicatorGraph;
