"use client"
import {Line} from 'react-chartjs-2';
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
import {useEffect, useRef} from 'react';

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

const PriceGraph = ({prices}) => {
    const chartRef = useRef(null);

    useEffect(() => {
        const chart = chartRef.current;
        if (chart) {
            const ctx = chart.ctx;
            const gradient = ctx.createLinearGradient(0, 0, 0, 400);
            gradient.addColorStop(0, 'rgba(75, 192, 192, 0.2)');
            gradient.addColorStop(1, 'rgba(75, 192, 192, 0)');

            chart.data.datasets[0].backgroundColor = gradient;
            chart.update();
        }
    }, []);

    const data = {
        labels: prices,
        datasets: [
            {
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

    return <div style={{width: '300px', height: '200px'}}> {/* Adjust the width and height as needed */}
        <Line ref={chartRef} data={data} options={options}/>
    </div>
};

export default PriceGraph;
