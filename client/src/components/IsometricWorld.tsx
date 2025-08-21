import { useRef, useMemo, useState, useEffect } from 'react';
import { useFrame, useLoader } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

interface TileData {
  q: number;
  r: number;
  type: 'grass' | 'dirt' | 'stone' | 'water' | 'corrupted';
  height: number;
}

export default function IsometricWorld() {
  const { playerPosition } = useGameState();
  const { currentStage } = useNarrative();
  const groupRef = useRef<THREE.Group>(null);
  
  // Load tile textures
  const grassTexture = useLoader(THREE.TextureLoader, '/sprites/grass_tile.png');
  
  // Configure texture for pixel-perfect rendering
  useEffect(() => {
    grassTexture.magFilter = THREE.NearestFilter;
    grassTexture.minFilter = THREE.NearestFilter;
    grassTexture.wrapS = THREE.RepeatWrapping;
    grassTexture.wrapT = THREE.RepeatWrapping;
  }, [grassTexture]);

  // Generate world tiles based on current stage
  const tiles = useMemo(() => {
    const worldTiles: TileData[] = [];
    const radius = 10;
    
    for (let q = -radius; q <= radius; q++) {
      for (let r = -radius; r <= radius; r++) {
        const s = -q - r;
        if (Math.abs(s) <= radius) {
          // Determine tile type based on stage and position
          let tileType: TileData['type'] = 'grass';
          
          // Add corruption based on stage
          if (currentStage >= 2) {
            const distance = Math.sqrt(q * q + r * r);
            if (distance > 6 && Math.random() < currentStage * 0.15) {
              tileType = 'corrupted';
            }
          }
          
          // Add water tiles for variety
          if (Math.abs(q) === radius || Math.abs(r) === radius || Math.abs(s) === radius) {
            if (Math.random() < 0.3) {
              tileType = 'water';
            }
          }
          
          worldTiles.push({
            q,
            r,
            type: tileType,
            height: 0,
          });
        }
      }
    }
    
    return worldTiles;
  }, [currentStage]);

  // Convert hex to isometric screen position
  const hexToIsometric = (q: number, r: number) => {
    const tileWidth = 1.732; // sqrt(3) for proper hex ratio
    const tileHeight = 1.5;
    
    const x = (q * tileWidth) + (r * tileWidth / 2);
    const z = r * tileHeight;
    
    return { x, z };
  };

  // Get tile color based on type and stage
  const getTileColor = (type: TileData['type']) => {
    switch (type) {
      case 'grass':
        // Darken grass as stages progress
        const darkness = currentStage * 0.15;
        return new THREE.Color(0.4 - darkness, 0.8 - darkness, 0.3 - darkness);
      case 'dirt':
        return new THREE.Color(0.6, 0.4, 0.2);
      case 'stone':
        return new THREE.Color(0.5, 0.5, 0.5);
      case 'water':
        return new THREE.Color(0.2, 0.4, 0.7);
      case 'corrupted':
        return new THREE.Color(0.2, 0.1, 0.2);
      default:
        return new THREE.Color(0.5, 0.5, 0.5);
    }
  };

  // Animate tiles based on stage
  useFrame((state) => {
    if (!groupRef.current) return;
    
    const time = state.clock.elapsedTime;
    
    // Add subtle animation in horror stages
    if (currentStage >= 3) {
      groupRef.current.children.forEach((child, index) => {
        if (child instanceof THREE.Mesh) {
          const wave = Math.sin(time * 0.5 + index * 0.1) * 0.02;
          child.position.y = wave * currentStage;
        }
      });
    }
  });

  return (
    <group ref={groupRef}>
      {tiles.map((tile, index) => {
        const { x, z } = hexToIsometric(tile.q, tile.r);
        const isPlayerTile = tile.q === playerPosition.q && tile.r === playerPosition.r;
        
        return (
          <mesh
            key={`${tile.q}_${tile.r}`}
            position={[x, tile.height, z]}
            rotation={[-Math.PI / 2, 0, 0]}
          >
            <planeGeometry args={[2, 2]} />
            <meshStandardMaterial
              map={tile.type === 'grass' ? grassTexture : null}
              color={tile.type !== 'grass' ? getTileColor(tile.type) : undefined}
              emissive={isPlayerTile ? new THREE.Color(0.2, 0.2, 0) : undefined}
              emissiveIntensity={isPlayerTile ? 0.3 : 0}
              transparent={tile.type === 'water'}
              opacity={tile.type === 'water' ? 0.8 : 1}
            />
          </mesh>
        );
      })}
      
      {/* Grid lines for debugging (remove in production) */}
      {tiles.map((tile) => {
        const { x, z } = hexToIsometric(tile.q, tile.r);
        return (
          <lineSegments
            key={`grid_${tile.q}_${tile.r}`}
            position={[x, 0.01, z]}
          >
            <edgesGeometry args={[new THREE.PlaneGeometry(2, 2)]} />
            <lineBasicMaterial color={0x333333} opacity={0.2} transparent />
          </lineSegments>
        );
      })}
    </group>
  );
}