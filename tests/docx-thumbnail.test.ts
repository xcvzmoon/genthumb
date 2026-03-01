import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail docx', () => {
  it('should generate thumbnail from DOCX document', () => {
    const fixturePath = path.join(testDir, 'test-docx.docx');

    const result = generateThumbnail(fixturePath, 220, 220);
    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);

    const header = result.subarray(0, 4);
    expect(header.toString()).toBe('RIFF');
  });

  it('should throw error for malformed DOCX', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'sipat-docx-'));
    const badDocxPath = path.join(tempDir, 'bad-document.docx');

    try {
      fs.writeFileSync(badDocxPath, 'this is not a valid docx zip file');
      expect(() => generateThumbnail(badDocxPath, 220, 220)).toThrow();
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
