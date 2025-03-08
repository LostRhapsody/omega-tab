// simple test function
import { describe, it, expect } from 'vitest';

function sum(a, b) {
  return a + b;
}

describe('Sum function', () => {
  it('should add two numbers correctly', () => {
    expect(sum(1, 2)).toBe(3);
    expect(sum(-1, 1)).toBe(0);
    expect(sum(0, 0)).toBe(0);
    expect(sum(5.5, 4.5)).toBe(10);
  });
});
