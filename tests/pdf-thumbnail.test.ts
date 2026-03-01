import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { describe, it, expect } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail pdf', () => {
  it('should generate thumbnail from PDF document', () => {
    const fixturePath = path.join(testDir, 'test-pdf.pdf');

    const result = generateThumbnail(fixturePath, 100, 100);
    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);

    const header = result.subarray(0, 4);
    expect(header.toString()).toBe('RIFF');
  });

  it('should throw error for nonexistent PDF file', () => {
    expect(() => generateThumbnail('nonexistent.pdf', 100, 100)).toThrow();
  });

  it('should throw error for malformed PDF', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'sipat-pdf-'));
    const badPdfPath = path.join(tempDir, 'bad-document.pdf');

    try {
      fs.writeFileSync(badPdfPath, 'this is not a real pdf');

      expect(() => generateThumbnail(badPdfPath, 100, 100)).toThrow();
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
