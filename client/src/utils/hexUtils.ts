export interface HexCoordinate {
  q: number;
  r: number;
}

export interface WorldPosition {
  x: number;
  y: number;
  z: number;
}

/**
 * Convert hex coordinates (q, r) to world position (x, y, z)
 */
export function getHexPosition(q: number, r: number, y: number = 0): WorldPosition {
  const size = 1; // Hex tile size
  const x = size * (3/2 * q);
  const z = size * (Math.sqrt(3)/2 * q + Math.sqrt(3) * r);
  
  return { x, y, z };
}

/**
 * Convert world position to hex coordinates
 */
export function worldToHex(x: number, z: number): HexCoordinate {
  const size = 1;
  const q = (2/3 * x) / size;
  const r = (-1/3 * x + Math.sqrt(3)/3 * z) / size;
  
  return { q: Math.round(q), r: Math.round(r) };
}

/**
 * Calculate distance between two hex coordinates
 */
export function hexDistance(a: HexCoordinate, b: HexCoordinate): number {
  return (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;
}

/**
 * Get neighbors of a hex coordinate
 */
export function getHexNeighbors(coord: HexCoordinate): HexCoordinate[] {
  const directions = [
    { q: 1, r: 0 },   // Right
    { q: 1, r: -1 },  // Top-right
    { q: 0, r: -1 },  // Top-left
    { q: -1, r: 0 },  // Left
    { q: -1, r: 1 },  // Bottom-left
    { q: 0, r: 1 },   // Bottom-right
  ];
  
  return directions.map(dir => ({
    q: coord.q + dir.q,
    r: coord.r + dir.r,
  }));
}

/**
 * Check if a hex coordinate is within a certain radius of center
 */
export function isWithinRadius(coord: HexCoordinate, center: HexCoordinate, radius: number): boolean {
  return hexDistance(coord, center) <= radius;
}

/**
 * Generate hex coordinates in a spiral pattern around center
 */
export function generateHexSpiral(center: HexCoordinate, radius: number): HexCoordinate[] {
  const results: HexCoordinate[] = [];
  
  for (let q = -radius; q <= radius; q++) {
    const r1 = Math.max(-radius, -q - radius);
    const r2 = Math.min(radius, -q + radius);
    
    for (let r = r1; r <= r2; r++) {
      results.push({ q: center.q + q, r: center.r + r });
    }
  }
  
  return results;
}