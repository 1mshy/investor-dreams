import {
    format_currency,
    format_number_with_commas
} from "@/app/funcs/formatting";
import { format_number } from "@/app/funcs/formatting";
import { expect } from "vitest";

test("number formatting with comma", () => {
  expect(format_number_with_commas(1000)).toBe("1,000");
  expect(format_number_with_commas(1000001)).toBe("1,000,001");
  expect(format_number_with_commas(1000000000.34)).toBe("1,000,000,000.34");
  expect(format_number_with_commas(1000000000000.66666)).toBe("1,000,000,000,000.67");
  expect(format_number_with_commas(1000000000000000)).toBe(
    "1,000,000,000,000,000"
  );
  expect(format_number_with_commas(999999)).toBe("999,999");
  expect(format_number_with_commas(9999.999999999)).toBe("10,000");
});

test("number formatting with currency", () => {
  expect(format_currency(1000)).toBe("$1,000");
  expect(format_currency(1000000)).toBe("$1,000,000");
  expect(format_currency(1000000000.55)).toBe("$1,000,000,000.55");
  expect(format_currency(1000000000000.5555555555555555555)).toBe("$1,000,000,000,000.56");
  expect(format_currency(1000000000000000)).toBe("$1,000,000,000,000,000");
});

test("number formatting with currency", () => {
  expect(format_number(1000)).toBe("1K");
  expect(format_number(1000000)).toBe("1M");
  expect(format_number(1000000000)).toBe("1B");
  expect(format_number(1000000000000)).toBe("1T");
  expect(format_number(1230000000000)).toBe("1.23T");
  expect(format_number(1234000000000)).toBe("1.23T");
  expect(format_number(1234)).toBe("1.23K");
});
