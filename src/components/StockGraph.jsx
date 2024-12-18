import { user_settings } from '@/app/config/settings';
import { get_all_prices, get_five_year_prices, get_month_prices, get_ten_year_prices, get_year_prices, get_ytd_prices } from '@/app/funcs/historical_pricing';
import { fetch_widget_data, fetch_yahoo_timeset, percentage_change } from '@/app/funcs/stock_api';
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
import React, { useEffect, useState } from 'react';
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

const StockGraph = ({ symbol, size, timeset = "D" }) => {
    const [historical_data, set_historical_data] = useState([]);
    const [chart_data, set_chart_data] = useState(null);
    const [chart_options, set_chart_options] = useState(null);
    const [dimensions, set_dimensions] = useState({ width: '300px', height: '200px' });

    const calculateDimensions = (size) => {
        switch (size) {
            case 'big':
                return { width: '50%', height: '50%' };
            case 'full':
                return { width: '100%', height: '100%' };
            default:
                return { width: '300px', height: '200px' };
        }
    };

    const setupChart = (prices) => {
        const data = {
            labels: prices,
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
                    label: 'Price',
                    data: prices,
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
                    external: (context) => {
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
                            tooltipEl.style.transition = 'opacity 0.2s ease, left 0.2s ease, top 0.2s ease';
                            document.body.appendChild(tooltipEl);
                        }

                        if (tooltipModel.opacity === 0) {
                            tooltipEl.style.opacity = 0;
                            return;
                        }

                        if (tooltipModel.body) {
                            const value = tooltipModel.dataPoints[0].raw;
                            const current_price = prices[prices.length - 1];
                            const percent_change = percentage_change(unformat_number(current_price), unformat_number(value));
                            const formatDate = (dateString) => new Date(dateString).toLocaleDateString('en-US', {
                                year: 'numeric', month: 'long', day: 'numeric',
                            });

                            const index = tooltipModel.dataPoints[0].dataIndex;
                            const date = historical_data[prices.length - 1 - index]?.datetime;

                            console.log(index)
                            tooltipEl.innerHTML = `
                                <div><strong>Price:</strong> ${format_currency(value)}</div>
                                ${user_settings.show_relative_prices_on_graph.value ? `<div><strong>Relatively:</strong> ${format_percentage(percent_change || 0)}</div>` : ''}
                                ${date ? `<div><strong>Date:</strong> ${formatDate(date)}</div>` : ''}
                            `;
                        }

                        const position = context.chart.canvas.getBoundingClientRect();
                        tooltipEl.style.left = position.left + window.pageXOffset + tooltipModel.caretX + 'px';
                        tooltipEl.style.top = position.top + window.pageYOffset + tooltipModel.caretY + 'px';
                        tooltipEl.style.opacity = 1;
                    },
                },
            },
        };

        set_chart_data(data);
        set_chart_options(options);
    };

    const fetchData = async () => {
        try {
            if (symbol) {
                const yahoo_timeset = await fetch_yahoo_timeset(symbol, timeset);
                const historical = yahoo_timeset.data;
                set_historical_data(historical);
                switch (timeset) {
                    case "D":
                        setupChart(get_all_prices(historical));
                        break;
                    case "M":
                        setupChart(get_month_prices(historical));
                        break;
                    case "YTD":
                        setupChart(get_ytd_prices(historical));
                        break;
                    case "Y":
                        setupChart(get_year_prices(historical));
                        break;
                    case "5Y":
                        setupChart(get_five_year_prices(historical));
                        break;
                    case "10Y":
                        setupChart(get_ten_year_prices(historical));
                        break;
                    default:
                        setupChart(get_all_prices(historical));
                        break;
                }

            }
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    };

    useEffect(() => {
        fetchData();
        set_dimensions(calculateDimensions(size));
    }, [symbol, size, timeset]);

    if (!chart_data || !chart_options) return <div>Loading Chart...</div>;

    return (
        <div style={dimensions}>
            <Line data={chart_data} options={chart_options} />
        </div>
    );
};

export default StockGraph;
