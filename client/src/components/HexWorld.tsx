import { useMemo } from 'react';
import { useTexture } from '@react-three/drei';
import * as THREE from 'three';
import { useNarrative } from '../lib/stores/useNarrative';
import { getHexPosition } from '../utils/hexUtils';

const HEX_SIZE = 1;
const WORLD_RADIUS = 8;

export default function HexWorld() {
  const { currentStage } = useNarrative();
  
  // Get textures based on current stage
  const grassTexture = useTexture("/textures/grass.png");
  const sandTexture = useTexture("/textures/sand.jpg");
  const asphaltTexture = useTexture("/textures/asphalt.png");
  
  // Generate hex tiles based on current stage
  const hexTiles = useMemo(() => {
    const tiles = [];
    
    for (let q = -WORLD_RADIUS; q <= WORLD_RADIUS; q++) {
      const r1 = Math.max(-WORLD_RADIUS, -q - WORLD_RADIUS);
      const r2 = Math.min(WORLD_RADIUS, -q + WORLD_RADIUS);
      
      for (let r = r1; r <= r2; r++) {
        const position = getHexPosition(q, r, HEX_SIZE);
        const distance = Math.sqrt(q * q + r * r + q * r);
        
        // Determine tile type based on stage and distance from center
        let tileType = 'grass';
        let color = '#4CAF50';
        let texture = grassTexture;
        
        if (currentStage >= 1) { // Unease
          if (distance > 4) {
            tileType = 'dark_grass';
            color = '#2E7D32';
          }
        }
        
        if (currentStage >= 2) { // Dread
          if (distance > 3) {
            tileType = 'swamp';
            color = '#4A4A4A';
            texture = sandTexture;
          }
        }
        
        if (currentStage >= 3) { // Terror
          if (distance > 2) {
            tileType = 'ruins';
            color = '#303030';
            texture = asphaltTexture;
          }
        }
        
        if (currentStage >= 4) { // Horror
          if (distance > 1) {
            tileType = 'labyrinth';
            color = '#1A1A1A';
            texture = asphaltTexture;
          }
        }
        
        tiles.push({
          id: `${q}-${r}`,
          position: [position.x, 0, position.z],
          color,
          texture,
          tileType,
          q,
          r
        });
      }
    }
    
    return tiles;
  }, [currentStage, grassTexture, sandTexture, asphaltTexture]);

  return (
    <group>
      {hexTiles.map((tile) => (
        <mesh
          key={tile.id}
          position={tile.position as [number, number, number]}
          rotation={[-Math.PI / 2, 0, 0]}
          receiveShadow
        >
          <cylinderGeometry args={[HEX_SIZE * 0.9, HEX_SIZE * 0.9, 0.1, 6]} />
          <meshLambertMaterial 
            map={tile.texture}
            color={tile.color}
          />
        </mesh>
      ))}
    </group>
  );
}
