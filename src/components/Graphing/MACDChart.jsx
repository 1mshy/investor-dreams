import React from 'react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement, // Import BarElement
  Title,
  Tooltip,
  Legend,
} from 'chart.js';

// Register Chart.js components
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement, // Register BarElement
  Title,
  Tooltip,
  Legend
);

const MACDChart = () => {
  // Dummy data for the MACD chart
  const data = {
    labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'], // X-axis labels (timeframe)
    datasets: [
      {
        label: 'MACD Line',
        data: [1, 2, 1.5, 2.5, 3, 2.8], // MACD Line values
        borderColor: 'rgba(75, 192, 192, 1)',
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        tension: 0.3, // Smoothing
      },
      {
        label: 'Signal Line',
        data: [1.1, 1.8, 1.4, 2.3, 2.7, 2.5], // Signal Line values
        borderColor: 'rgba(255, 99, 132, 1)',
        backgroundColor: 'rgba(255, 99, 132, 0.2)',
        tension: 0.3, // Smoothing
      },
      {
        label: 'Histogram',
        data: [0.1, -0.2, 0.1, 0.2, 0.3, 0.3], // Histogram values
        type: 'bar', // Use bar chart for the histogram
        backgroundColor: (context) => {
          // Conditional coloring for histogram bars
          const value = context.raw;
          return value > 0 ? 'rgba(75, 192, 192, 0.8)' : 'rgba(255, 99, 132, 0.8)';
        },
      },
    ],
  };

  // Chart options
  const options = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: 'MACD Chart',
      },
    },
    scales: {
      x: {
        title: {
          display: true,
          text: 'Time',
        },
      },
      y: {
        title: {
          display: true,
          text: 'Value',
        },
      },
    },
  };

  return <Line data={data} options={options} />;
};

export default MACDChart;
