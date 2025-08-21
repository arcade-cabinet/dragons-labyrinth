import { useRef, useMemo, useState, useEffect } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
const grassTexture = '/attached_assets/generated_images/grass_hexagon_tile_a528c91a.png';
const forestTexture = '/attached_assets/generated_images/forest_hex_tile_1eafc553.png';
const corruptedTexture = '/attached_assets/generated_images/corrupted_hex_tile_6c6e11cd.png';

interface HexTile {
  q: number;
  r: number;
  s: number;
  type: 'grass' | 'forest' | 'stone' | 'water' | 'corrupted' | 'void';
  elevation: number;
  biomeFeatures: string[];
}

export default function HexagonalWorld() {
  const { playerPosition } = useGameState();
  const { currentStage } = useNarrative();
  const worldRef = useRef<THREE.Group>(null);
  
  // Generate hexagonal world with proper axial coordinates
  const hexTiles = useMemo(() => {
    const tiles: HexTile[] = [];
    const mapRadius = 12; // Larger world for exploration
    
    for (let q = -mapRadius; q <= mapRadius; q++) {
      const r1 = Math.max(-mapRadius, -q - mapRadius);
      const r2 = Math.min(mapRadius, -q + mapRadius);
      
      for (let r = r1; r <= r2; r++) {
        const s = -q - r;
        const distance = (Math.abs(q) + Math.abs(r) + Math.abs(s)) / 2;
        
        // Determine tile type based on position and stage
        let tileType: HexTile['type'] = 'grass';
        let elevation = 0;
        const biomeFeatures: string[] = [];
        
        // Create biome regions
        if (distance < 3) {
          // Starting village area
          tileType = 'grass';
          if (Math.random() < 0.1) biomeFeatures.push('flowers');
        } else if (distance < 6) {
          // Forest ring
          tileType = Math.random() < 0.7 ? 'forest' : 'grass';
          if (tileType === 'forest') {
            elevation = 0.1;
            biomeFeatures.push('trees');
          }
        } else if (distance < 9) {
          // Mixed terrain
          const rand = Math.random();
          if (rand < 0.3) {
            tileType = 'stone';
            elevation = 0.2;
          } else if (rand < 0.5) {
            tileType = 'water';
            elevation = -0.1;
          } else {
            tileType = 'grass';
          }
        } else {
          // Outer regions - progressively more corrupted
          if (currentStage >= 2) {
            tileType = Math.random() < (currentStage * 0.2) ? 'corrupted' : 'stone';
          } else {
            tileType = 'stone';
          }
          elevation = 0.3;
        }
        
        // Stage-based corruption spreading
        if (currentStage >= 3 && distance > 4) {
          const corruptionChance = (currentStage - 2) * 0.15 * (distance / mapRadius);
          if (Math.random() < corruptionChance) {
            tileType = 'corrupted';
            biomeFeatures.push('darkness');
          }
        }
        
        // Dragon's influence in Horror stage
        if (currentStage === 4 && distance < 5) {
          if (Math.random() < 0.3) {
            tileType = 'void';
            elevation = -0.5;
            biomeFeatures.push('whispers');
          }
        }
        
        tiles.push({
          q, r, s,
          type: tileType,
          elevation,
          biomeFeatures
        });
      }
    }
    
    return tiles;
  }, [currentStage]);

  // Convert hex coordinates to world position with proper spacing
  const hexToWorld = (q: number, r: number, elevation: number = 0) => {
    const size = 1;
    const x = size * (Math.sqrt(3) * q + Math.sqrt(3)/2 * r);
    const z = size * (3/2 * r);
    return new THREE.Vector3(x, elevation, z);
  };

  // Get tile appearance based on type and stage
  const getTileAppearance = (tile: HexTile) => {
    const baseColors: Record<HexTile['type'], string> = {
      grass: '#4CAF50',
      forest: '#2E7D32',
      stone: '#757575',
      water: '#1976D2',
      corrupted: '#311B92',
      void: '#000000'
    };
    
    // Darken colors based on stage
    const stageDarkness = currentStage * 0.1;
    const color = new THREE.Color(baseColors[tile.type]);
    color.multiplyScalar(1 - stageDarkness);
    
    return {
      color: `#${color.getHexString()}`,
      emissive: tile.type === 'corrupted' ? '#4A148C' : undefined,
      emissiveIntensity: tile.type === 'corrupted' ? 0.2 : 0,
      opacity: tile.type === 'water' ? 0.8 : 1,
    };
  };

  // Animate world based on stage
  useFrame((state) => {
    if (!worldRef.current) return;
    const time = state.clock.elapsedTime;
    
    // Horror stage warping
    if (currentStage >= 3) {
      worldRef.current.children.forEach((child, i) => {
        if (child instanceof THREE.Mesh) {
          const originalY = child.userData.elevation || 0;
          const warp = Math.sin(time * 0.5 + i * 0.1) * 0.05 * (currentStage - 2);
          child.position.y = originalY + warp;
        }
      });
    }
  });

  // Create hexagon geometry
  // Load textures conditionally
  const [grassTex, setGrassTex] = useState<THREE.Texture | null>(null);
  const [forestTex, setForestTex] = useState<THREE.Texture | null>(null);
  const [corruptedTex, setCorruptedTex] = useState<THREE.Texture | null>(null);
  
  useEffect(() => {
    const loader = new THREE.TextureLoader();
    loader.load(grassTexture, (texture) => {
      texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
      texture.repeat.set(1, 1);
      setGrassTex(texture);
    });
    loader.load(forestTexture, (texture) => {
      texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
      texture.repeat.set(1, 1);
      setForestTex(texture);
    });
    loader.load(corruptedTexture, (texture) => {
      texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
      texture.repeat.set(1, 1);
      setCorruptedTex(texture);
    });
  }, []);
  
  const hexGeometry = useMemo(() => {
    const shape = new THREE.Shape();
    const size = 1;
    
    for (let i = 0; i < 6; i++) {
      const angle = (Math.PI / 3) * i;
      const x = size * Math.cos(angle);
      const y = size * Math.sin(angle);
      
      if (i === 0) {
        shape.moveTo(x, y);
      } else {
        shape.lineTo(x, y);
      }
    }
    shape.closePath();
    
    return new THREE.ShapeGeometry(shape);
  }, []);

  return (
    <group ref={worldRef}>
      {/* Render hex tiles */}
      {hexTiles.map((tile) => {
        const position = hexToWorld(tile.q, tile.r, tile.elevation);
        const appearance = getTileAppearance(tile);
        const isPlayerTile = tile.q === playerPosition.q && tile.r === playerPosition.r;
        
        return (
          <mesh
            key={`${tile.q}_${tile.r}`}
            position={position}
            rotation={[-Math.PI / 2, 0, 0]}
            userData={{ elevation: tile.elevation, q: tile.q, r: tile.r }}
            receiveShadow
            onClick={(e) => {
              e.stopPropagation();
              if ((window as any).handleHexClick) {
                (window as any).handleHexClick(tile.q, tile.r);
              }
            }}
            onPointerOver={(e) => {
              e.stopPropagation();
              document.body.style.cursor = 'pointer';
            }}
            onPointerOut={(e) => {
              e.stopPropagation();
              document.body.style.cursor = 'default';
            }}
          >
            <primitive object={hexGeometry} />
            <meshStandardMaterial
              map={
                tile.type === 'grass' ? grassTex :
                tile.type === 'forest' ? forestTex :
                tile.type === 'corrupted' ? corruptedTex :
                undefined
              }
              color={
                (tile.type === 'grass' || tile.type === 'forest' || tile.type === 'corrupted') && 
                (grassTex || forestTex || corruptedTex) ? '#FFFFFF' : appearance.color
              }
              emissive={appearance.emissive || (isPlayerTile ? '#FFD700' : undefined)}
              emissiveIntensity={appearance.emissiveIntensity || (isPlayerTile ? 0.3 : 0)}
              transparent={tile.type === 'water' || tile.type === 'void'}
              opacity={appearance.opacity}
              roughness={tile.type === 'water' ? 0.1 : 0.8}
              metalness={tile.type === 'water' ? 0.8 : 0.1}
            />
          </mesh>
        );
      })}
      
      {/* Add biome features */}
      {hexTiles.map((tile) => {
        if (tile.biomeFeatures.length === 0) return null;
        const position = hexToWorld(tile.q, tile.r, tile.elevation);
        
        return tile.biomeFeatures.map((feature, idx) => {
          if (feature === 'trees') {
            return (
              <mesh
                key={`${tile.q}_${tile.r}_tree_${idx}`}
                position={[position.x + Math.random() * 0.4 - 0.2, 0.5, position.z + Math.random() * 0.4 - 0.2]}
                castShadow
              >
                <coneGeometry args={[0.2, 1, 6]} />
                <meshStandardMaterial color="#1B5E20" />
              </mesh>
            );
          } else if (feature === 'darkness') {
            return (
              <mesh
                key={`${tile.q}_${tile.r}_dark_${idx}`}
                position={[position.x, 0.3, position.z]}
              >
                <sphereGeometry args={[0.3, 8, 8]} />
                <meshBasicMaterial color="#000000" transparent opacity={0.7} />
              </mesh>
            );
          }
          return null;
        });
      })}
      
      {/* Hex grid overlay for clarity */}
      {hexTiles.map((tile) => {
        const position = hexToWorld(tile.q, tile.r, tile.elevation + 0.01);
        
        return (
          <lineSegments
            key={`grid_${tile.q}_${tile.r}`}
            position={position}
            rotation={[-Math.PI / 2, 0, 0]}
          >
            <edgesGeometry args={[hexGeometry]} />
            <lineBasicMaterial 
              color={0x000000} 
              opacity={0.1} 
              transparent 
            />
          </lineSegments>
        );
      })}
    </group>
  );
}