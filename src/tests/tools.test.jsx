import { format_currency, format_number, format_number_with_commas } from "@/app/funcs/tools";
import { expect } from "vitest";

test('number formatting with comma', () => {
    expect(format_number_with_commas(1000)).toBe("1,000")
    expect(format_number_with_commas(1000000)).toBe("1,000,000")
    expect(format_number_with_commas(1000000000)).toBe("1,000,000,000")
    expect(format_number_with_commas(1000000000000)).toBe("1,000,000,000,000")
    expect(format_number_with_commas(1000000000000000)).toBe("1,000,000,000,000,000")
    expect(format_number_with_commas(999999)).toBe("999,999")
    expect(format_number_with_commas(9999)).toBe("9,999")
});

test('number formatting with currency', () => {
    expect(format_currency(1000)).toBe("$1,000")
    expect(format_currency(1000000)).toBe("$1,000,000")
    expect(format_currency(1000000000)).toBe("$1,000,000,000")
    expect(format_currency(1000000000000)).toBe("$1,000,000,000,000")
    expect(format_currency(1000000000000000)).toBe("$1,000,000,000,000,000")
});

test('number formatting with currency', () => {
    expect(format_number(1000)).toBe("1K")
    expect(format_number(1000000)).toBe("1M")
    expect(format_number(1000000000)).toBe("1B")
    expect(format_number(1000000000000)).toBe("1T")
    expect(format_number(1230000000000)).toBe("1.23T")
    expect(format_number(1234000000000)).toBe("1.23T")
    expect(format_number(1234)).toBe("1.23K")
});