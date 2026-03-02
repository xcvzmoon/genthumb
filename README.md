# GENTHUMB

[![CI](https://github.com/xcvzmoon/genthumb/actions/workflows/ci.yaml/badge.svg)](https://github.com/xcvzmoon/genthumb/actions/workflows/ci.yaml)
[![Repository](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/xcvzmoon/genthumb)
[![npm](https://img.shields.io/npm/v/genthumb.svg)](https://www.npmjs.com/package/genthumb)

Generate fast WebP thumbnails from images, PDFs, Office files, spreadsheets, and text.

## Installation

```bash
npm install genthumb
```

## Supported Inputs

- Image: `jpg`, `jpeg`, `png`, `gif`, `bmp`, `webp`, `tiff`
- Document: `pdf`, `docx`, `doc`
- Presentation: `pptx`, `ppt`
- Spreadsheet: `csv`, `tsv`, `xlsx`, `xls`, `xlsm`, `xlsb`, `ods`
- Text: `txt`, `text`, `md`, `markdown`, `log`

Notes:

- `csv` and `tsv` are treated as spreadsheet inputs.
- For unknown formats, genthumb returns an error.

## API

```ts
generateThumbnail(input, width, height, mimeType?) => Buffer
```

- `input`: `string | Buffer`
- `width`: `number`
- `height`: `number`
- `mimeType` (optional): `string`

Returns a WebP-encoded `Buffer`.

## Usage

Path input:

```ts
import { writeFileSync } from 'node:fs';
import { generateThumbnail } from 'genthumb';

const thumb = generateThumbnail('./tests/documents/test-image.jpeg', 320, 240);
writeFileSync('./thumb.webp', thumb);
```

Buffer input:

```ts
import { readFileSync, writeFileSync } from 'node:fs';
import { generateThumbnail } from 'genthumb';

const source = readFileSync('./tests/documents/test-pdf.pdf');
const thumb = generateThumbnail(source, 320, 240, 'application/pdf');
writeFileSync('./thumb.webp', thumb);
```

Buffer input with MIME auto-detection:

```ts
import { readFileSync } from 'node:fs';
import { generateThumbnail } from 'genthumb';

const source = readFileSync('./tests/documents/test-image.jpeg');
const thumb = generateThumbnail(source, 320, 240);
```
