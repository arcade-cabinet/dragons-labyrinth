// Main game component for Dragon's Labyrinth
import React, { useState, useEffect, useRef } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Text, Box, Plane } from '@react-three/drei';
import { ContentGenerator } from '../generators/ContentGenerator';
import { AudioGenerator } from '../generators/AudioGenerator';
import * as THREE from 'three';

interface GameState {
  dreadLevel: number;
  sanity: number;
  health: number;
  currentBiome: string;
  companions: string[];
  position: { q: number; r: number };
}

export function DragonsLabyrinth() {
  const [gameState, setGameState] = useState<GameState>({
    dreadLevel: 0,
    sanity: 100,
    health: 100,
    currentBiome: 'meadow',
    companions: ['Einar', 'Mira', 'Sorin', 'Tamara'],
    position: { q: 0, r: 0 }
  });

  const [levelData, setLevelData] = useState<any>(null);
  const [uiScene, setUiScene] = useState<string>('');
  
  const contentGenerator = useRef(new ContentGenerator());
  const audioGenerator = useRef<AudioGenerator | null>(null);

  useEffect(() => {
    // Initialize audio
    audioGenerator.current = new AudioGenerator();
    
    // Generate initial content
    const generator = contentGenerator.current;
    const level = generator.generateYolLevel(gameState.currentBiome, gameState.dreadLevel);
    const ui = generator.generateCobScene(gameState.dreadLevel);
    
    setLevelData(JSON.parse(level));
    setUiScene(ui);
    
    // Start ambient audio
    const ambience = audioGenerator.current.generateAmbience(gameState.dreadLevel);
    ambience.start();
    
    return () => {
      ambience.stop();
      audioGenerator.current?.dispose();
    };
  }, []);

  useEffect(() => {
    // Update content when dread level changes
    const generator = contentGenerator.current;
    const level = generator.generateYolLevel(gameState.currentBiome, gameState.dreadLevel);
    const ui = generator.generateCobScene(gameState.dreadLevel);
    
    setLevelData(JSON.parse(level));
    setUiScene(ui);
    
    // Update audio
    if (audioGenerator.current) {
      const ambience = audioGenerator.current.generateAmbience(gameState.dreadLevel);
      ambience.start();
      
      // Add heartbeat at higher dread levels
      if (gameState.dreadLevel >= 2) {
        audioGenerator.current.generateHeartbeat(gameState.dreadLevel - 1);
      }
      
      // Add whispers at terror level
      if (gameState.dreadLevel >= 3) {
        audioGenerator.current.generateWhisper("turn back", gameState.dreadLevel);
      }
    }
  }, [gameState.dreadLevel, gameState.currentBiome]);

  const handleHexClick = (q: number, r: number) => {
    // Play click sound
    audioGenerator.current?.generateUISound('click', gameState.dreadLevel);
    
    // Move player
    setGameState(prev => ({
      ...prev,
      position: { q, r }
    }));
    
    // Random chance to increase dread
    if (Math.random() < 0.1) {
      increaseDread();
    }
  };

  const increaseDread = () => {
    setGameState(prev => ({
      ...prev,
      dreadLevel: Math.min(4, prev.dreadLevel + 1),
      sanity: Math.max(0, prev.sanity - 20)
    }));
  };

  // Get color based on dread level
  const getDreadColor = () => {
    const colors = ['#4ade80', '#facc15', '#f97316', '#ef4444', '#991b1b'];
    return colors[gameState.dreadLevel];
  };

  return (
    <div style={{ width: '100vw', height: '100vh', background: '#1a1a1a' }}>
      {/* 3D Game View */}
      <Canvas
        camera={{ position: [10, 15, 10], fov: 50 }}
        style={{ background: gameState.dreadLevel >= 4 ? '#000' : '#87CEEB' }}
      >
        <ambientLight intensity={Math.max(0.1, 0.5 - gameState.dreadLevel * 0.1)} />
        <directionalLight position={[10, 10, 5]} intensity={1 - gameState.dreadLevel * 0.15} />
        
        {/* Render hex tiles */}
        {levelData?.entities?.filter((e: any) => e.name === 'HexTile').map((tile: any, index: number) => (
          <HexTile
            key={index}
            q={tile.components.position.q}
            r={tile.components.position.r}
            type={tile.components.tile_type}
            corruption={tile.components.corruption}
            onClick={() => handleHexClick(tile.components.position.q, tile.components.position.r)}
            isPlayer={tile.components.position.q === gameState.position.q && tile.components.position.r === gameState.position.r}
          />
        ))}
        
        {/* Render NPCs */}
        {levelData?.entities?.filter((e: any) => e.name === 'NPC').map((npc: any, index: number) => (
          <NPC
            key={index}
            position={hexToWorld(npc.components.position.q, npc.components.position.r)}
            type={npc.components.npc_type}
            sanity={npc.components.sanity}
          />
        ))}
        
        {/* Render Monsters */}
        {levelData?.entities?.filter((e: any) => e.name === 'Monster').map((monster: any, index: number) => (
          <Monster
            key={index}
            position={hexToWorld(monster.components.position.q, monster.components.position.r)}
            type={monster.components.monster_type}
            health={monster.components.health}
          />
        ))}
        
        <OrbitControls enablePan={false} maxPolarAngle={Math.PI / 3} />
        
        {/* Fog effect for horror atmosphere */}
        <fog attach="fog" color={gameState.dreadLevel >= 3 ? "#ff0000" : "#ffffff"} near={10} far={50 - gameState.dreadLevel * 8} />
      </Canvas>
      
      {/* UI Overlay */}
      <div style={{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        padding: '20px',
        background: `linear-gradient(to bottom, rgba(0,0,0,${0.3 + gameState.dreadLevel * 0.1}), transparent)`,
        color: 'white',
        pointerEvents: 'none'
      }}>
        <h1 style={{ margin: 0, fontSize: '24px', textShadow: '2px 2px 4px rgba(0,0,0,0.8)' }}>
          Dragon's Labyrinth
        </h1>
        <div style={{ marginTop: '10px', display: 'flex', gap: '20px' }}>
          <div>
            Health: <span style={{ color: '#4ade80' }}>{gameState.health}</span>
          </div>
          <div>
            Sanity: <span style={{ color: getDreadColor() }}>{gameState.sanity}</span>
          </div>
          <div>
            Dread: <span style={{ color: getDreadColor() }}>{['Peace', 'Unease', 'Dread', 'Terror', 'Horror'][gameState.dreadLevel]}</span>
          </div>
        </div>
        <div style={{ marginTop: '10px' }}>
          Companions: {gameState.companions.join(', ')}
        </div>
      </div>
      
      {/* Action buttons */}
      <div style={{
        position: 'absolute',
        bottom: '20px',
        left: '50%',
        transform: 'translateX(-50%)',
        display: 'flex',
        gap: '10px'
      }}>
        <button
          onClick={() => {
            audioGenerator.current?.generateUISound('click', gameState.dreadLevel);
            // Generate new dialogue
            const dialogue = contentGenerator.current.generateDialogue(
              gameState.companions[0] || 'Einar',
              gameState.dreadLevel
            );
            console.log('Dialogue:', dialogue);
          }}
          style={{
            padding: '10px 20px',
            background: getDreadColor(),
            color: 'white',
            border: 'none',
            borderRadius: '5px',
            cursor: 'pointer'
          }}
        >
          Talk
        </button>
        <button
          onClick={() => {
            audioGenerator.current?.generateUISound('click', gameState.dreadLevel);
            increaseDread();
          }}
          style={{
            padding: '10px 20px',
            background: '#dc2626',
            color: 'white',
            border: 'none',
            borderRadius: '5px',
            cursor: 'pointer'
          }}
        >
          [DEBUG] Increase Dread
        </button>
      </div>
    </div>
  );
}

// Hex tile component
function HexTile({ q, r, type, corruption, onClick, isPlayer }: any) {
  const position = hexToWorld(q, r);
  
  const getColor = () => {
    if (isPlayer) return '#3b82f6';
    if (corruption > 0.5) return '#991b1b';
    
    const colors: Record<string, string> = {
      grass: '#4ade80',
      withered_grass: '#a3a3a3',
      forest_floor: '#059669',
      dead_leaves: '#78716c',
      murky_water: '#164e63',
      cracked_stone: '#57534e',
      ancient_stone: '#1f2937',
      corrupted: '#7c2d12'
    };
    return colors[type] || '#525252';
  };
  
  return (
    <group position={position} onClick={onClick}>
      <mesh>
        <cylinderGeometry args={[1, 1, 0.2, 6]} />
        <meshStandardMaterial color={getColor()} />
      </mesh>
      {isPlayer && (
        <mesh position={[0, 0.5, 0]}>
          <coneGeometry args={[0.3, 1, 4]} />
          <meshStandardMaterial color="#fbbf24" />
        </mesh>
      )}
    </group>
  );
}

// NPC component
function NPC({ position, type, sanity }: any) {
  const color = sanity > 50 ? '#10b981' : sanity > 20 ? '#f59e0b' : '#ef4444';
  
  return (
    <group position={position}>
      <mesh position={[0, 0.5, 0]}>
        <capsuleGeometry args={[0.3, 0.8, 4, 8]} />
        <meshStandardMaterial color={color} />
      </mesh>
      <Text
        position={[0, 1.5, 0]}
        fontSize={0.3}
        color="white"
        anchorX="center"
        anchorY="middle"
      >
        {type}
      </Text>
    </group>
  );
}

// Monster component
function Monster({ position, type, health }: any) {
  const color = health > 100 ? '#dc2626' : health > 50 ? '#ef4444' : '#f87171';
  
  return (
    <group position={position}>
      <mesh position={[0, 0.5, 0]}>
        <boxGeometry args={[0.6, 1, 0.6]} />
        <meshStandardMaterial color={color} />
      </mesh>
      <Text
        position={[0, 1.5, 0]}
        fontSize={0.25}
        color="#ff0000"
        anchorX="center"
        anchorY="middle"
      >
        {type}
      </Text>
    </group>
  );
}

// Convert hex coordinates to world position
function hexToWorld(q: number, r: number): [number, number, number] {
  const size = 1.5;
  const x = size * (Math.sqrt(3) * q + Math.sqrt(3) / 2 * r);
  const z = size * (3 / 2 * r);
  return [x, 0, z];
}