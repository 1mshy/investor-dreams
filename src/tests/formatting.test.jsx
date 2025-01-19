import { is_complex_ticker } from "@/app/funcs/stock_api";

test("complex_symbol", () => {
    const symbol = "AAPL";
    const result = is_complex_ticker(symbol);
    expect(result).toBe(false);
});

test("complex_symbol", () => {
    const symbol = "ABR^D";
    const result = is_complex_ticker(symbol);
    expect(result).toBe(true);
});
test("complex_symbol", () => {
    const symbol = "G^FG";
    const result = is_complex_ticker(symbol);
    expect(result).toBe(true);
});
test("complex_symbol", () => {
    const symbol = "ABRD";
    const result = is_complex_ticker(symbol);
    expect(result).toBe(false);
});