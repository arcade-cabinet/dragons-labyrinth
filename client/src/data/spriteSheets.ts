export interface SpriteFrame {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface SpriteSheet {
  texture: string;
  frames: Record<string, SpriteFrame>;
}

// Character sprite sheet definitions
export const CHARACTER_SPRITES: SpriteSheet = {
  texture: '/sprites/character_sheet.png',
  frames: {
    'player_knight': { x: 0, y: 0, width: 32, height: 32 },
    'player_mage': { x: 32, y: 0, width: 32, height: 32 },
    'player_archer': { x: 64, y: 0, width: 32, height: 32 },
    'companion_einar': { x: 0, y: 32, width: 32, height: 32 },
    'companion_mira': { x: 32, y: 32, width: 32, height: 32 },
    'companion_sorin': { x: 64, y: 32, width: 32, height: 32 },
    'companion_tamara': { x: 96, y: 32, width: 32, height: 32 },
  }
};

// Monster sprite sheet definitions
export const MONSTER_SPRITES: SpriteSheet = {
  texture: '/sprites/monster_sheet.png',
  frames: {
    'hollow_caretaker': { x: 0, y: 0, width: 32, height: 32 },
    'forsaken_knight': { x: 32, y: 0, width: 32, height: 32 },
    'swamp_creature': { x: 64, y: 0, width: 32, height: 32 },
    'ghost_echo': { x: 96, y: 0, width: 32, height: 32 },
    'dragon': { x: 0, y: 32, width: 64, height: 64 },
  }
};

// 2.5D Hexagonal world tile sprite sheet
export const HEX_WORLD_TILES: SpriteSheet = {
  texture: '/sprites/hex_world_tiles.svg',
  frames: {
    'grass': { x: 0, y: 0, width: 64, height: 64 },
    'forest': { x: 64, y: 0, width: 64, height: 64 },
    'stone': { x: 128, y: 0, width: 64, height: 64 },
    'water': { x: 192, y: 0, width: 64, height: 64 },
    'corrupted': { x: 256, y: 0, width: 64, height: 64 },
    'void': { x: 320, y: 0, width: 64, height: 64 },
    'village': { x: 0, y: 64, width: 64, height: 64 },
    'ruins': { x: 64, y: 64, width: 64, height: 64 },
  }
};

// Environment sprite sheet definitions
export const ENVIRONMENT_SPRITES: SpriteSheet = {
  texture: '/sprites/environment_sheet.png',
  frames: {
    'village_house': { x: 0, y: 0, width: 48, height: 48 },
    'market_stall': { x: 48, y: 0, width: 32, height: 32 },
    'tombstone': { x: 80, y: 0, width: 16, height: 24 },
    'twisted_tree': { x: 96, y: 0, width: 32, height: 48 },
    'ruined_wall': { x: 128, y: 0, width: 32, height: 32 },
    'labyrinth_portal': { x: 0, y: 48, width: 64, height: 64 },
    'flowers': { x: 64, y: 48, width: 16, height: 16 },
    'ruins': { x: 80, y: 48, width: 32, height: 32 },
  }
};

export function getSpriteFrame(sheet: SpriteSheet, frameName: string): SpriteFrame | null {
  return sheet.frames[frameName] || null;
}