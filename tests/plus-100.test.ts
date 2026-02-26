import { describe, it, expect } from 'vitest';
import { plus100 } from '../index.js';

describe('plus100', () => {
  it('adds 100 to a positive number', () => {
    expect(plus100(5)).toBe(105);
  });

  it('adds 100 to zero', () => {
    expect(plus100(0)).toBe(100);
  });

  it('adds 100 to a large number', () => {
    expect(plus100(1_000_000)).toBe(1_000_100);
  });

  it('handles the max u32 boundary (4294967195 + 100 = 4294967295)', () => {
    expect(plus100(4_294_967_195)).toBe(4_294_967_295);
  });
});
