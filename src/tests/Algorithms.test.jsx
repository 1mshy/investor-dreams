import { invoke } from "@tauri-apps/api/core";

test('true', async () => {
    const prices = [44, 44.15, 44.29, 44.21, 44.17, 44.30, 44.29, 44.40, 44.39, 44.51, 44.49, 44.65, 44.73, 44.74, 44.90, 45.01, 45.02].map(price => ({ close: price }));
    const rsiValues = invoke("rsi", { prices, period: 14 });
    expect(rsiValues).toEqual([88.04597701149484, 88.14866760168366]);
});