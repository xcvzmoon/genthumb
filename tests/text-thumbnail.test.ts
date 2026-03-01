import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail text', () => {
  it('should generate thumbnail from plain text file', () => {
    const fixturePath = path.join(testDir, 'test-text.txt');

    const result = generateThumbnail(fixturePath, 240, 180);
    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);

    const header = result.subarray(0, 4);
    expect(header.toString()).toBe('RIFF');
  });

  it('should throw error for nonexistent text file', () => {
    expect(() => generateThumbnail('missing-text-file.txt', 240, 180)).toThrow();
  });
});
