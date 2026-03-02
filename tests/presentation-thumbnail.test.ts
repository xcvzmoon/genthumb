import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail presentation', () => {
  it('should generate thumbnail from PPTX document', () => {
    const fixturePath = path.join(testDir, 'test-pptx.pptx');
    const result = generateThumbnail(fixturePath, 240, 160);
    const header = result.subarray(0, 4);

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(header.toString()).toBe('RIFF');
  });

  it('should throw error for malformed PPTX', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'genthumb-pptx-'));
    const malformedPath = path.join(tempDir, 'bad-presentation.pptx');

    try {
      fs.writeFileSync(malformedPath, 'not-a-valid-pptx');
      expect(() => generateThumbnail(malformedPath, 240, 160)).toThrow();
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
