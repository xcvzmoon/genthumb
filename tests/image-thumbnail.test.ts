import path from 'node:path';
import { describe, it, expect } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail', () => {
  it('should generate thumbnail from JPEG image', () => {
    const fixturePath = path.join(testDir, 'test-image.jpeg');
    const result = generateThumbnail(fixturePath, 100, 100);
    const header = result.subarray(0, 4);

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(header.toString()).toBe('RIFF');
  });

  it('should throw error for nonexistent file', () => {
    expect(() => generateThumbnail('nonexistent.png', 100, 100)).toThrow();
  });

  it('should throw error for unsupported format', () => {
    const jsonPath = path.join(testDir, 'test-unsupported.json');

    expect(() => generateThumbnail(jsonPath, 100, 100)).toThrow();
  });
});
