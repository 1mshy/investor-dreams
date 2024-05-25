"use client"
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
        if(size === 'big') {
            dimensions = { width: '50%', height: '50%'};
        }

        const data = {
            labels: prices,
            datasets: [
                {
                    backgroundColor: function(context) {
                        const chart = context.chart;
                        const { ctx, chartArea } = chart;
            
                        if (!chartArea) {
                            // This case happens on initial chart render
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
                    dots: false,
                    fill: true,
                    pointRadius: 0, // Remove the dots
                    pointHoverRadius: 0, // Show dot on hover
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.4,
                }
            ]
        };

        const options = {
            responsive: true,
            plugins: {
                legend: {
                    display: false,
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                },
            },
            scales: {
                x: {
                    display: false, // Hide the x-axis labels
                },
                y: {
                    display: false,
                    beginAtZero: false,
                }
            }
        };

        return (
            <div style={dimensions}> {/* Adjust the width and height as needed */}
                <Line data={data} options={options} />
            </div>
        );
    }
}

export default PriceGraph;
