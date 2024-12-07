import { calculateRSI } from "@/app/funcs/algorithms";

test('true', () => {
    const prices = [44, 44.15, 44.29, 44.21, 44.17, 44.30, 44.29, 44.40, 44.39, 44.51, 44.49, 44.65, 44.73, 44.74, 44.90, 45.01, 45.02].map(price => ({close: price}));
    const rsiValues = calculateRSI(prices, 14);
    expect(rsiValues.map(data => data.rsi)).toEqual([88.04597701149484, 88.14866760168366]);
});