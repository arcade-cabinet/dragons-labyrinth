export interface HexCoordinate {
  q: number;
  r: number;
}

export interface Point3D {
  x: number;
  y: number;
  z: number;
}

export function getHexPosition(q: number, r: number, size: number = 1): Point3D {
  const x = size * (3/2 * q);
  const z = size * (Math.sqrt(3)/2 * q + Math.sqrt(3) * r);
  return { x, y: 0, z };
}

export function getHexDistance(a: HexCoordinate, b: HexCoordinate): number {
  return (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;
}

export function getHexNeighbors(coord: HexCoordinate): HexCoordinate[] {
  const directions = [
    { q: 1, r: 0 },   // East
    { q: 1, r: -1 },  // Northeast
    { q: 0, r: -1 },  // Northwest
    { q: -1, r: 0 },  // West
    { q: -1, r: 1 },  // Southwest
    { q: 0, r: 1 },   // Southeast
  ];
  
  return directions.map(dir => ({
    q: coord.q + dir.q,
    r: coord.r + dir.r
  }));
}

export function hexToPixel(coord: HexCoordinate, size: number): { x: number, y: number } {
  const x = size * (3/2 * coord.q);
  const y = size * (Math.sqrt(3)/2 * coord.q + Math.sqrt(3) * coord.r);
  return { x, y };
}

export function pixelToHex(point: { x: number, y: number }, size: number): HexCoordinate {
  const q = (2/3 * point.x) / size;
  const r = (-1/3 * point.x + Math.sqrt(3)/3 * point.y) / size;
  return hexRound({ q, r });
}

export function hexRound(coord: { q: number, r: number }): HexCoordinate {
  const s = -coord.q - coord.r;
  
  let rq = Math.round(coord.q);
  let rr = Math.round(coord.r);
  let rs = Math.round(s);
  
  const qDiff = Math.abs(rq - coord.q);
  const rDiff = Math.abs(rr - coord.r);
  const sDiff = Math.abs(rs - s);
  
  if (qDiff > rDiff && qDiff > sDiff) {
    rq = -rr - rs;
  } else if (rDiff > sDiff) {
    rr = -rq - rs;
  }
  
  return { q: rq, r: rr };
}

export function getHexRing(center: HexCoordinate, radius: number): HexCoordinate[] {
  if (radius === 0) return [center];
  
  const results: HexCoordinate[] = [];
  let cube = { q: center.q - radius, r: center.r, s: center.r + radius };
  
  const directions = [
    { q: 1, r: -1 },  // Northeast
    { q: 1, r: 0 },   // East
    { q: 0, r: 1 },   // Southeast
    { q: -1, r: 1 },  // Southwest
    { q: -1, r: 0 },  // West
    { q: 0, r: -1 },  // Northwest
  ];
  
  for (let i = 0; i < 6; i++) {
    for (let j = 0; j < radius; j++) {
      results.push({ q: cube.q, r: cube.r });
      cube.q += directions[i].q;
      cube.r += directions[i].r;
    }
  }
  
  return results;
}
