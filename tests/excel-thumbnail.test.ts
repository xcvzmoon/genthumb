import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail spreadsheet', () => {
  it('should generate thumbnail from CSV spreadsheet', () => {
    const fixturePath = path.join(testDir, 'test-csv.csv');

    const result = generateThumbnail(fixturePath, 240, 160);
    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);

    const header = result.subarray(0, 4);
    expect(header.toString()).toBe('RIFF');
  });

  it('should generate thumbnail from XLSX spreadsheet', () => {
    const fixturePath = path.join(testDir, 'test-xlsx.xlsx');

    const result = generateThumbnail(fixturePath, 240, 160);
    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);

    const header = result.subarray(0, 4);
    expect(header.toString()).toBe('RIFF');
  });
});
