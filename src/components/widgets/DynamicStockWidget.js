"use client";

import { Component } from 'react';
import BigStockWidget from './BigStockWidget';
import MediumStockWidget from './MediumStockWidget';
import MiniStockWidget from "./MiniStockWidget";
import PopupWidget from './PopupWidget';


/**
 * @param props.isPositive {boolean}
 * @type {IStyledComponent<"web", Substitute<import("react").DetailedHTMLProps<import("react").HTMLAttributes<HTMLDivElement>, HTMLDivElement>, BaseObject>> & BaseObject & {}}
 * @desc affects colour if positive or negative (green or red)
 */
export const StockChange = (props) => {
    return <div style={{ color: props.isPositive ? '#4caf50' : '#e74c3c' }}>
        {...props.children}
    </div>
}
/**
 * @param {string} symbol
 * @param {string} name
 * @param {string} exchange
 * @param {number} price
 * @param {number} percent_change
 * @param {string} date
 * @param {Array<number>} historical_prices
 * @param {string} size - "big" or "medium" or "mini"
 * @desc this stock widget is able to combine all the stock widgets into one, making them transitionable
 * @example you can open a big stock widget from a mini stock widget by clicking on it
 */
export class DynamicStockWidget extends Component {
  constructor(props) {
    super(props);
    this.state = {
      size: props.size ? props.size : 'medium',
    };

    this.setSize = this.setSize.bind(this);
  }

  setSize(value) {
    this.setState({ size: value });
  }

  render() {
    const { symbol, name, price, percent_change, date, historical_prices } = this.props;
    const { size } = this.state;
    const is_big = size === 'big';
    const is_medium = size === 'medium';
    const is_mini = size === 'mini';

    return (
      <>
        {is_mini && <MiniStockWidget 
          symbol={symbol} 
          name={name} 
          price={price} 
          percent_change={percent_change} 
          onClick={() => { this.setSize("big") }} 
        />}
        {is_medium && <MediumStockWidget 
          symbol={symbol} 
          name={name} 
          price={price} 
          percent_change={percent_change} 
          date={date} 
          historical_prices={historical_prices} 
          onClick={() => { this.setSize("big") }} 
        />}
        <PopupWidget 
          symbol={symbol} 
          name={name} 
          price={price} 
          percent_change={percent_change} 
          date={date} 
          historical_prices={historical_prices} 
          onClick={() => { this.setSize(this.props.size) }} 
          open={is_big} 
        />
      </>
    );
  }
}
