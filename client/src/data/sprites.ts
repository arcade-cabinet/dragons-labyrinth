// Generated game sprite paths
const villagerSprite = '/attached_assets/generated_images/villager_chess_piece_0249ad01.png';
const bossKnightSprite = '/attached_assets/generated_images/boss_knight_piece_f28eef65.png';
const dragonSprite = '/attached_assets/generated_images/dragon_boss_sprite_6ea3ad3e.png';
const grassTile = '/attached_assets/generated_images/grass_hexagon_tile_a528c91a.png';
const forestTile = '/attached_assets/generated_images/forest_hex_tile_1eafc553.png';
const corruptedTile = '/attached_assets/generated_images/corrupted_hex_tile_6c6e11cd.png';

export const sprites = {
  villager: villagerSprite,
  bossKnight: bossKnightSprite,
  dragon: dragonSprite,
  grassTile: grassTile,
};

export const getSpriteForType = (type: 'player' | 'companion' | 'npc' | 'boss' | 'dragon') => {
  switch (type) {
    case 'dragon':
      return sprites.dragon;
    case 'boss':
      return sprites.bossKnight;
    case 'npc':
      return sprites.villager;
    case 'companion':
    case 'player':
    default:
      return sprites.villager; // Using villager as default for now
  }
};