import React, { useState, useRef } from "react";
import MediumStockWidget from "./MediumStockWidget";
import MiniStockWidget from "./MiniStockWidget";
import PopupWidget from "./PopupWidget";
import SmallStockWidget from "./SmallStockWidget";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {String} exchange
 * @param {Number} price
 * @param {Number} percent_change
 * @param {String} date
 * @param {Array<number>} historical_prices
 * @param {String} size - "big" or "medium" or "mini"
 * @desc this stock widget is able to combine all the stock widgets into one, making them transitionable
 * @example you can open a big stock widget from a mini stock widget by clicking on it
 */
export function DynamicStockWidget({ size: initialSize = "medium", ...props }) {
  const [size, setSize] = useState(initialSize);
  const startSize = useRef(initialSize);

  const isBig = size === "big";
  const isMedium = size === "medium";
  const isSmall = size === "small";
  const isMini = size === "mini";

  const expand = () => setSize("big");
  const shrink = () => setSize(startSize.current);

  return (
    <>
      {isMini && <MiniStockWidget {...props} onClick={expand} />}
      {isSmall && <SmallStockWidget {...props} onClick={expand} />}
      {isMedium && <MediumStockWidget {...props} onClick={expand} />}
      <PopupWidget {...props} onClick={shrink} open={isBig} />
    </>
  );
}
