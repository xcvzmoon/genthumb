import fs from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { generateThumbnail } from '../index.js';

const testDir = path.join(process.cwd(), 'tests', 'documents');

describe('generate_thumbnail buffer input', () => {
  it('should generate thumbnail from image buffer', () => {
    const inputPath = path.join(testDir, 'test-image.jpeg');
    const input = fs.readFileSync(inputPath);
    const result = generateThumbnail(input, 200, 120, 'image/jpeg');

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(result.subarray(0, 4).toString()).toBe('RIFF');
  });

  it('should generate thumbnail from text buffer', () => {
    const inputPath = path.join(testDir, 'test-text.txt');
    const input = fs.readFileSync(inputPath);
    const result = generateThumbnail(input, 200, 120, 'text/plain');

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(result.subarray(0, 4).toString()).toBe('RIFF');
  });

  it('should generate thumbnail from the failing DOCX buffer labeled as zip', () => {
    const inputPath = path.join(testDir, 'test-docx.docx');
    const input = fs.readFileSync(inputPath);
    const result = generateThumbnail(input, 220, 220, 'application/zip');

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(result.subarray(0, 4).toString()).toBe('RIFF');
  });

  it('should generate thumbnail from PPTX buffer labeled as zip', () => {
    const inputPath = path.join(testDir, 'test-pptx.pptx');
    const input = fs.readFileSync(inputPath);
    const result = generateThumbnail(input, 240, 160, 'application/zip');

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(result.subarray(0, 4).toString()).toBe('RIFF');
  });

  it('should generate thumbnail from XLSX buffer labeled as zip', () => {
    const inputPath = path.join(testDir, 'test-xlsx.xlsx');
    const input = fs.readFileSync(inputPath);
    const result = generateThumbnail(input, 240, 160, 'application/zip');

    expect(result).toBeInstanceOf(Buffer);
    expect(result.length).toBeGreaterThan(0);
    expect(result.subarray(0, 4).toString()).toBe('RIFF');
  });
});
