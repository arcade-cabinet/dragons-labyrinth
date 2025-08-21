import { useRef, useMemo, useState, useEffect } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

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
        
        // Biome regions based on narrative document
        if (distance < 3) {
          // Peace: Village, Market, Farmland
          tileType = 'grass';
          if (Math.random() < 0.1) biomeFeatures.push('flowers');
          if (Math.random() < 0.05) biomeFeatures.push('house');
        } else if (distance < 6 && currentStage >= 1) {
          // Unease: Forest, Town, Crypt Outskirts
          const rand = Math.random();
          if (rand < 0.6) {
            tileType = 'forest';
            elevation = 0.1;
            biomeFeatures.push('trees');
          } else if (rand < 0.8) {
            tileType = 'stone'; // Town areas
            elevation = 0.15;
          } else {
            tileType = 'grass';
            if (currentStage >= 1) biomeFeatures.push('tombstone');
          }
        } else if (distance < 9 && currentStage >= 2) {
          // Dread: Swamp, Ruins, Abandoned Fort, Cavern
          const rand = Math.random();
          if (rand < 0.3) {
            tileType = 'water'; // Swamp
            elevation = -0.1;
            biomeFeatures.push('murk');
          } else if (rand < 0.6) {
            tileType = 'stone'; // Ruins and fort
            elevation = 0.2;
            if (Math.random() < 0.3) biomeFeatures.push('ruins');
          } else {
            tileType = 'forest'; // Dark forest
            elevation = 0.1;
            biomeFeatures.push('trees');
            biomeFeatures.push('darkness');
          }
        } else if (distance >= 9 && currentStage >= 3) {
          // Terror: Ghost Town, Warped City, Mirror Lake, Labyrinth Outskirts
          const rand = Math.random();
          if (rand < 0.3) {
            tileType = 'corrupted'; // Warped areas
            biomeFeatures.push('warped');
          } else if (rand < 0.5) {
            tileType = 'water'; // Mirror lake
            elevation = -0.05;
            biomeFeatures.push('mirror');
          } else {
            tileType = 'stone'; // Ghost town
            elevation = 0.25;
            biomeFeatures.push('abandoned');
          }
          
          // Labyrinth entrance (specific location in Terror stage)
          if (q === 10 && r === -5 && currentStage >= 3) {
            biomeFeatures.push('labyrinth_entrance');
          }
        } else {
          // Default outer areas
          tileType = distance < 6 ? 'grass' : 'stone';
          elevation = distance < 6 ? 0 : 0.2;
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

  // Use simple geometries instead of heavy 3D models for mobile performance
  const getTileGeometry = (tileType: string) => {
    // Return simple colored geometries instead of loading heavy models
    const baseHeight = 0.2;
    const radius = 1;
    
    switch (tileType) {
      case 'water':
        return {
          geometry: <cylinderGeometry args={[radius, radius, baseHeight * 0.5, 6]} />,
          material: (
            <meshStandardMaterial
              color="#1976D2"
              transparent
              opacity={0.7}
              roughness={0.1}
              metalness={0.8}
            />
          )
        };
      case 'corrupted':
        return {
          geometry: <cylinderGeometry args={[radius, radius, baseHeight, 6]} />,
          material: (
            <meshStandardMaterial
              color="#7C4DFF"
              emissive="#4A148C"
              emissiveIntensity={0.3}
            />
          )
        };
      case 'void':
        return {
          geometry: <cylinderGeometry args={[radius, radius, baseHeight * 0.3, 6]} />,
          material: (
            <meshStandardMaterial
              color="#000000"
              transparent
              opacity={0.3}
            />
          )
        };
      default:
        return {
          geometry: <cylinderGeometry args={[radius, radius, baseHeight, 6]} />,
          material: (
            <meshStandardMaterial
              color={
                tileType === 'grass' || tileType === 'village' ? '#4CAF50' :
                tileType === 'forest' ? '#2E7D32' :
                tileType === 'stone' || tileType === 'ruins' ? '#757575' :
                '#616161'
              }
            />
          )
        };
    }
  };
  
  // Add ground plane for visual reference
  const groundPlane = useMemo(() => (
    <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -0.5, 0]} receiveShadow>
      <planeGeometry args={[100, 100]} />
      <meshStandardMaterial color="#1a1a1a" />
    </mesh>
  ), []);

  return (
    <group ref={worldRef}>
      {/* Ground plane for visual reference */}
      {groundPlane}
      
      {/* Render hex tiles */}
      {hexTiles.map((tile) => {
        const position = hexToWorld(tile.q, tile.r, tile.elevation);
        const appearance = getTileAppearance(tile);
        const isPlayerTile = tile.q === playerPosition.q && tile.r === playerPosition.r;
        
        // Check for special tiles
        const hasLabyrinthEntrance = tile.biomeFeatures?.includes('labyrinth_entrance');
        const tileGeom = getTileGeometry(tile.type);
        
        return (
          <group
            key={`${tile.q}_${tile.r}`}
            position={position}
            userData={{ elevation: tile.elevation, q: tile.q, r: tile.r }}
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
            {hasLabyrinthEntrance ? (
              // Special indicator for labyrinth entrance - simple geometry
              <group>
                <mesh receiveShadow position={[0, 0, 0]}>
                  <cylinderGeometry args={[1, 1, 0.3, 6]} />
                  <meshStandardMaterial
                    color="#1A237E"
                    emissive="#7C4DFF"
                    emissiveIntensity={0.5}
                  />
                </mesh>
                <mesh position={[0, 0.5, 0]}>
                  <boxGeometry args={[0.5, 1, 0.1]} />
                  <meshStandardMaterial
                    color="#000000"
                    emissive="#9C27B0"
                    emissiveIntensity={0.8}
                  />
                </mesh>
              </group>
            ) : (
              // Use lightweight geometry instead of heavy models
              <mesh receiveShadow position={[0, 0, 0]}>
                {tileGeom.geometry}
                {tileGeom.material}
              </mesh>
            )}
            
            {/* Player highlight effect */}
            {isPlayerTile && (
              <mesh position={[0, 0.1, 0]} rotation={[-Math.PI / 2, 0, 0]}>
                <ringGeometry args={[0.8, 1.2, 6]} />
                <meshBasicMaterial color="#FFD700" transparent opacity={0.5} />
              </mesh>
            )}
          </group>
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
                <meshStandardMaterial color={currentStage >= 2 ? "#0D2818" : "#1B5E20"} />
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
          } else if (feature === 'house') {
            return (
              <mesh
                key={`${tile.q}_${tile.r}_house_${idx}`}
                position={[position.x, 0.4, position.z]}
                castShadow
              >
                <boxGeometry args={[0.6, 0.8, 0.6]} />
                <meshStandardMaterial color={currentStage === 0 ? "#8D6E63" : "#424242"} />
              </mesh>
            );
          } else if (feature === 'tombstone') {
            return (
              <mesh
                key={`${tile.q}_${tile.r}_tomb_${idx}`}
                position={[position.x, 0.3, position.z]}
                castShadow
              >
                <boxGeometry args={[0.3, 0.6, 0.1]} />
                <meshStandardMaterial color="#616161" />
              </mesh>
            );
          } else if (feature === 'ruins') {
            return (
              <mesh
                key={`${tile.q}_${tile.r}_ruin_${idx}`}
                position={[position.x, 0.4, position.z]}
                castShadow
              >
                <boxGeometry args={[0.7, 0.4, 0.7]} />
                <meshStandardMaterial color="#424242" roughness={1} />
              </mesh>
            );
          } else if (feature === 'labyrinth_entrance') {
            // This will be handled by a separate component
            return null;
          }
          return null;
        });
      })}
      
      {/* Remove grid overlay - tiles should be clear enough on their own */}
    </group>
  );
}