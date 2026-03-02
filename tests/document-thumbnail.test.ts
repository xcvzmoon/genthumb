import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail document', () => {
  it('should generate thumbnail from PDF document', () => {
    const fixturePath = path.join(testDir, 'test-pdf.pdf');
    const result = generateThumbnail(fixturePath, 100, 100);
    const header = result.subarray(0, 4);

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
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

  it('should generate thumbnail from DOCX document', () => {
    const fixturePath = path.join(testDir, 'test-docx.docx');
    const result = generateThumbnail(fixturePath, 220, 220);
    const header = result.subarray(0, 4);

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
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
