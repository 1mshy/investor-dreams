"use client";

import { Component } from 'react';
import MediumStockWidget from './MediumStockWidget';
import MiniStockWidget from "./MiniStockWidget";
import PopupWidget from './PopupWidget';

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
    this.start_size = this.state.size;

    this.setSize = this.setSize.bind(this);
  }

  setSize(value) {
    this.setState({ size: value });
  }

  render() {
    const { size } = this.state;
    const is_big = size === 'big';
    const is_medium = size === 'medium';
    const is_mini = size === 'mini';

    return (
      <>
        {is_mini && <MiniStockWidget
          {...this.props}
          onClick={() => { this.setSize("big") }}
        />}
        {is_medium && <MediumStockWidget
          {...this.props}
          onClick={() => { this.setSize("big") }}
        />}
        <PopupWidget
          {...this.props}
          onClick={() => { this.setSize(this.start_size) }}
          open={is_big}
        />
      </>
    );
  }
}
