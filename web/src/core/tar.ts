// Minimal reader for ustar archives (regular files only). Used for the sysroot bundle.

export interface TarEntry {
  path: string;
  data: Uint8Array;
}

const BLOCK = 512;
const decoder = new TextDecoder();

function readString(block: Uint8Array, offset: number, length: number): string {
  const bytes = block.subarray(offset, offset + length);
  const end = bytes.indexOf(0);
  return decoder.decode(end === -1 ? bytes : bytes.subarray(0, end));
}

function readOctal(block: Uint8Array, offset: number, length: number): number {
  const s = readString(block, offset, length).trim();
  const n = s === "" ? 0 : parseInt(s, 8);
  if (Number.isNaN(n)) throw new Error(`invalid tar header number: "${s}"`);
  return n;
}

export function parseTar(archive: Uint8Array): TarEntry[] {
  const entries: TarEntry[] = [];
  let offset = 0;
  while (offset + BLOCK <= archive.length) {
    const header = archive.subarray(offset, offset + BLOCK);
    if (header.every((b) => b === 0)) break; // end-of-archive marker
    const name = readString(header, 0, 100);
    const size = readOctal(header, 124, 12);
    const type = String.fromCharCode(header[156]);
    const prefix = readString(header, 345, 155);
    const dataStart = offset + BLOCK;
    if (dataStart + size > archive.length) throw new Error(`truncated tar entry: ${name}`);
    if (type === "0" || type === "\0") {
      const path = (prefix === "" ? name : `${prefix}/${name}`).replace(/^\.\//, "");
      entries.push({ path, data: archive.subarray(dataStart, dataStart + size) });
    }
    offset = dataStart + Math.ceil(size / BLOCK) * BLOCK;
  }
  return entries;
}
