import React from 'react';
import { Line } from 'react-chartjs-2';
import { Chart as ChartJS, Tooltip, LineElement, PointElement, CategoryScale, LinearScale, Title } from 'chart.js';

ChartJS.register(Tooltip, LineElement, PointElement, CategoryScale, LinearScale, Title);

const data = {
  labels: ['January', 'February', 'March', 'April', 'May'],
  datasets: [
    {
      label: 'My Dataset',
      data: [65, 59, 80, 81, 56],
      fill: false,
      backgroundColor: 'rgba(75,192,192,0.2)',
      borderColor: 'rgba(75,192,192,1)',
    },
  ],
};

const options = {
  plugins: {
    tooltip: {
      enabled: false, // Disable the default tooltip
      external: (context) => {
        console.log(context)
        // Custom tooltip function
        let tooltipModel = context.tooltip;
        let tooltipEl = document.getElementById('custom-tooltip');

        // Create tooltip element if it doesn't exist
        if (!tooltipEl) {
          tooltipEl = document.createElement('div');
          tooltipEl.id = 'custom-tooltip';
          tooltipEl.style.position = 'absolute';
          tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
          tooltipEl.style.color = 'white';
          tooltipEl.style.padding = '5px 10px';
          tooltipEl.style.borderRadius = '5px';
          tooltipEl.style.pointerEvents = 'none';
          document.body.appendChild(tooltipEl);
        }

        // Hide if no tooltip is present
        if (tooltipModel.opacity === 0) {
          tooltipEl.style.opacity = 0;
          return;
        }

        // Set custom text for the tooltip
        if (tooltipModel.body) {
          const { label, value } = tooltipModel.dataPoints[0];
          tooltipEl.innerHTML = `<strong>${label}:</strong> ${value}`;
        }

        // Position the tooltip
        const position = context.chart.canvas.getBoundingClientRect();
        tooltipEl.style.left = position.left + window.pageXOffset + tooltipModel.caretX + 'px';
        tooltipEl.style.top = position.top + window.pageYOffset + tooltipModel.caretY + 'px';
        tooltipEl.style.opacity = 1;
      },
    },
  },
  interaction: {
    mode: 'nearest',
    intersect: false,
  },
  scales: {
    x: {
      beginAtZero: true,
    },
    y: {
      beginAtZero: true,
    },
  },
};

const LineChartWithCustomTooltip = () => {
  return <Line data={data} options={options} />;
};

export default LineChartWithCustomTooltip;
