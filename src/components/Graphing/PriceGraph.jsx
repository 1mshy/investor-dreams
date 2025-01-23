/**
 * @fileoverview Price graph component using Chart.js.
 * Displays stock price data in a line chart with customizable dimensions
 * and interactive tooltips.
 */

import { user_settings } from '@/app/config/settings';
import { percentage_change } from '@/app/funcs/stock_api';
import { format_currency, format_percentage, unformat_number } from '@/app/funcs/tools';
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
import React, { Component } from 'react';
import { Line } from 'react-chartjs-2';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    Filler
);

/**
 * A component that renders a price chart using Chart.js.
 * Supports different sizes and displays interactive tooltips with price information.
 * 
 * @component
 * @extends {Component}
 * @param {Object} props - Component props
 * @param {Array<number>} props.prices - Array of price values to display
 * @param {string} [props.size] - Size of the chart ('big', 'full', or default)
 * @param {Array} [props.historical_data] - Historical price data with dates
 * @returns {JSX.Element} The rendered price chart
 * @example
 * <PriceGraph 
 *   prices={[100, 101, 99, 102]} 
 *   size="big"
 *   historical_data={[{datetime: '2023-01-01', close: 100}, ...]}
 * />
 */
class PriceGraph extends Component {
    /**
     * Creates an instance of PriceGraph.
     * 
     * @param {Object} props - Component props
     */
    constructor(props) {
        super(props);
    }

    /**
     * Renders the price graph component.
     * 
     * @returns {JSX.Element} The rendered chart
     */
    render() {
        const { prices, size } = this.props;
        let dimensions = { width: '300px', height: '200px' };
        switch (size) {
            case 'big':
                dimensions = { width: '50%', height: '50%' };
                break;
            case 'full':
                dimensions = { width: '100%', height: '100%' };
                break;
        }

        /**
         * Creates a gradient background for the chart area.
         * 
         * @param {Object} context - The chart context
         * @returns {CanvasGradient|string} The gradient or empty string if no chart area
         */
        const createGradientBackground = function (context) {
            const chart = context.chart;
            const { ctx, chartArea } = chart;

            if (!chartArea) {
                return '';
            }
            const gradient = ctx.createLinearGradient(0, chartArea.top, 0, chartArea.bottom);
            gradient.addColorStop(0, 'rgba(75, 192, 192, 0.2)');
            gradient.addColorStop(0.5, 'rgba(75, 192, 192, 0.1)');
            gradient.addColorStop(1, 'rgba(75, 192, 192, 0)');
            return gradient;
        };

        const data = {
            labels: prices,
            datasets: [
                {
                    backgroundColor: createGradientBackground,
                    label: 'Price',
                    data: prices,
                    fill: true,
                    pointRadius: 0,
                    pointHoverRadius: 0,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.4,
                }
            ]
        };

        const options = {
            responsive: true,
            scales: {
                x: {
                    display: false,
                },
                y: {
                    display: false,
                    beginAtZero: false,
                },
            },
            plugins: {
                legend: {
                    display: false,
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    enabled: false,
                    external: this.renderCustomTooltip.bind(this),
                },
            },
        };

        return (
            <div style={dimensions}>
                <Line data={data} options={options} />
            </div>
        );
    }

    /**
     * Renders a custom tooltip for the price chart.
     * 
     * @param {Object} context - Chart tooltip context
     */
    renderCustomTooltip(context) {
        const tooltipModel = context.tooltip;
        let tooltipEl = document.getElementById('custom-tooltip');

        if (!tooltipEl) {
            tooltipEl = document.createElement('div');
            tooltipEl.style.zIndex = '10000000';
            tooltipEl.id = 'custom-tooltip';
            tooltipEl.style.position = 'absolute';
            tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
            tooltipEl.style.color = 'white';
            tooltipEl.style.padding = '5px 10px';
            tooltipEl.style.borderRadius = '5px';
            tooltipEl.style.pointerEvents = 'none';
            tooltipEl.style.transition = 'opacity 0.2s ease, left 0.2s ease, top 0.2s ease';
            document.body.appendChild(tooltipEl);
        }

        if (tooltipModel.opacity === 0) {
            tooltipEl.style.opacity = 0;
            return;
        }

        if (tooltipModel.body) {
            const { prices } = this.props;
            const value = tooltipModel.dataPoints[0].raw;
            const current_price = prices[prices.length - 1];
            const percent_change = percentage_change(unformat_number(current_price), unformat_number(value));
            const are_different_numbers = unformat_number(current_price) !== unformat_number(value);
            
            let tooltipContent = `<div><strong>Price:</strong> ${format_currency(value)}</div>`;
            
            if (user_settings.show_relative_prices_on_graph.value && are_different_numbers) {
                tooltipContent += `<div><strong>Relatively:</strong> ${format_percentage(!isNaN(percent_change) ? percent_change : 0)}</div>`;
            }

            if (this.props.historical_data) {
                const formatDate = (dateString) => {
                    const date = new Date(dateString);
                    date.setHours(12);
                    return date.toLocaleDateString('en-US', {
                        year: 'numeric',
                        month: 'long',
                        day: 'numeric'
                    });
                };

                const index = tooltipModel.dataPoints[0].dataIndex;
                const date = this.props.historical_data[prices.length - 1 - index].datetime;
                tooltipContent += `<div><strong>Date:</strong> ${formatDate(date)}</div>`;
            }

            tooltipEl.innerHTML = tooltipContent;
        }

        const position = context.chart.canvas.getBoundingClientRect();
        tooltipEl.style.left = position.left + window.pageXOffset + tooltipModel.caretX + 'px';
        tooltipEl.style.top = position.top + window.pageYOffset + tooltipModel.caretY + 'px';
        tooltipEl.style.opacity = 1;
    }
}

export default PriceGraph;
