import React, { Component } from 'react';
import { Line } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    Filler,
} from 'chart.js';
import PercentageFormat from './PercentageFormat';
import { format_currency, format_percentage, unformat_number } from '@/app/funcs/tools';
import { percentage_change } from '@/app/funcs/stock_api';
import { user_settings } from '@/app/config/settings';

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

class PriceGraph extends Component {
    constructor(props) {
        super(props);
    }

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

        const data = {
            labels: prices,
            datasets: [
                {
                    backgroundColor: function (context) {
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
                    },
                    label: 'Price',
                    data: prices,
                    fill: true,
                    pointRadius: 0,
                    pointHoverRadius: 0,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.4, // Smooth line tension
                }
            ]
        };

        const options = {
            responsive: true,
            // might solve issues with formatting in the future
            // maintainAspectRatio: !this.props.small,
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
                    external: (context) => {
                        const tooltipModel = context.tooltip;
                        let tooltipEl = document.getElementById('custom-tooltip');

                        // Create tooltip element if it doesn't exist
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
                            tooltipEl.style.transition = 'opacity 0.2s ease, left 0.2s ease, top 0.2s ease'; // Smooth transition
                            document.body.appendChild(tooltipEl);
                        }

                        // Hide if no tooltip is present
                        if (tooltipModel.opacity === 0) {
                            tooltipEl.style.opacity = 0;
                            return;
                        }

                        // Set custom text for the tooltip
                        if (tooltipModel.body) {
                            const { prices } = this.props;
                            const value = tooltipModel.dataPoints[0].raw;
                            const current_price = prices[prices.length - 1];
                            const percent_change = percentage_change(unformat_number(current_price), unformat_number(value));
                            const are_different_numbers = unformat_number(current_price) !== unformat_number(value)
                            tooltipEl.innerHTML = `<div><strong>Price:</strong> ${format_currency(value)}</div>`;
                            console.log(user_settings.show_relative_prices_on_graph)
                            if (user_settings.show_relative_prices_on_graph.value && are_different_numbers)
                                tooltipEl.innerHTML += `<div><strong>Relatively:</strong> ${format_percentage(!isNaN(percent_change) ? percent_change : 0)}</div>`;
                            const formatDate = (dateString) => {
                                const date = new Date(dateString);
                                date.setHours(12)
                                date.setUTCDate(date.getUTCDate() + 1) // apparently I need to use this for EST
                                const options = { year: 'numeric', month: 'long', day: 'numeric' };
                                return date.toLocaleDateString('en-US', options);
                            };

                            if (this.props.historical_data) {
                                const index = tooltipModel.dataPoints[0].dataIndex;
                                const date = this.props.historical_data[prices.length - 1 - index].datetime;
                                const formatted_date = formatDate(date)
                                tooltipEl.innerHTML += `<div><strong>Date:</strong> ${formatted_date}</div>`;
                            }
                        }

                        // Smoothly move the tooltip
                        const position = context.chart.canvas.getBoundingClientRect();
                        tooltipEl.style.left = position.left + window.pageXOffset + tooltipModel.caretX + 'px';
                        tooltipEl.style.top = position.top + window.pageYOffset + tooltipModel.caretY + 'px';
                        tooltipEl.style.opacity = 1;
                    },
                },
            },
        };

        return (
            <div style={dimensions}>
                <Line data={data} options={options} />
            </div>
        );
    }
}

export default PriceGraph;
