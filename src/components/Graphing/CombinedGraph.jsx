/**
 * @fileoverview Combined graph component that displays both stock price and indicators.
 * Combines StockGraph and IndicatorGraph components in a vertical layout.
 */

import React, { Component } from 'react';
import IndicatorGraph from './IndicatorGraph';
import StockGraph from './StockGraph';

/**
 * A component that combines stock price and technical indicator graphs.
 * Renders a StockGraph on top and an optional IndicatorGraph below.
 * 
 * @component
 * @extends {Component}
 * @param {Object} props - Component props
 * @param {string} props.symbol - Stock symbol to display
 * @param {string} [props.size] - Size of the graph ("big", "full", or default)
 * @param {string} [props.timeset] - Time period to display
 * @param {Array} [props.indicators] - Array of technical indicators to show
 * @returns {JSX.Element} Combined graph display
 * @example
 * <CombinedGraph 
 *   symbol="AAPL" 
 *   size="full" 
 *   timeset="1Y"
 *   indicators={["RSI"]} 
 * />
 */
class CombinedGraph extends Component {
    constructor(props) {
        super(props);
    }

    /**
     * Renders the combined graph components.
     * 
     * @returns {JSX.Element} Flex container with StockGraph and optional IndicatorGraph
     */
    render() {
        return (
            <div style={{ display: 'flex', flexDirection: 'column' }}>
                <div style={{ flex: 7, width: '100%' }}>
                    <StockGraph {...this.props} />
                </div>
                {this.props.indicators && <div style={{ flex: 3, width: '100%' }}>
                    <IndicatorGraph {...this.props} />
                </div>}
            </div>
        );
    }
}

export default CombinedGraph;
