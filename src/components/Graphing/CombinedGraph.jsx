import React, { Component } from 'react';
import IndicatorGraph from './IndicatorGraph';
import StockGraph from './StockGraph';
/**
 * combines the stock graph and the indicator graph
 */
class CombinedGraph extends Component {
    constructor(props) {
        super(props);
    }

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
