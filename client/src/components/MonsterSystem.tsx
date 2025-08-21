import { useMemo, useRef, useState, useEffect } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { useCompanions } from '../lib/stores/useCompanions';
import { getHexPosition } from '../utils/hexUtils';

interface Monster {
  id: string;
  name: string;
  type: string;
  position: { q: number; r: number };
  health: number;
  isVisible: boolean;
  behavior: 'whisper' | 'stalk' | 'patrol' | 'proximity';
}

export default function MonsterSystem() {
  const { currentStage } = useNarrative();
  const { playerPosition, sanity, adjustSanity } = useGameState();
  const { companions } = useCompanions();
  const [monsters, setMonsters] = useState<Monster[]>([]);
  const whisperTimerRef = useRef<NodeJS.Timeout | null>(null);

  // Generate monsters based on current stage
  const stageMonsters = useMemo(() => {
    const newMonsters: Monster[] = [];
    
    switch (currentStage) {
      case 1: // Unease - Whispering Shades
        for (let i = 0; i < 3; i++) {
          const angle = (i / 3) * Math.PI * 2;
          const radius = 4 + Math.random() * 2;
          const q = Math.round(Math.cos(angle) * radius);
          const r = Math.round(Math.sin(angle) * radius);
          
          newMonsters.push({
            id: `whisper_shade_${i}`,
            name: 'Whispering Shade',
            type: 'whispering_shade',
            position: { q, r },
            health: 100,
            isVisible: false, // Invisible entities
            behavior: 'whisper'
          });
        }
        break;
        
      case 2: // Dread - Hollow NPCs in ruins
        for (let i = 0; i < 2; i++) {
          const q = (i === 0 ? -5 : 5) + Math.round(Math.random() * 2 - 1);
          const r = Math.round(Math.random() * 2 - 1);
          
          newMonsters.push({
            id: `hollow_npc_${i}`,
            name: 'Hollow NPC',
            type: 'hollow_npc',
            position: { q, r },
            health: 50,
            isVisible: true,
            behavior: 'patrol'
          });
        }
        break;
        
      case 3: // Terror - Trauma Echoes
        companions.forEach((companion, index) => {
          if (companion.morale < 50) {
            newMonsters.push({
              id: `trauma_echo_${companion.id}`,
              name: `${companion.name} (Echo)`,
              type: 'trauma_echo',
              position: { 
                q: playerPosition.q + (index * 2 - 1), 
                r: playerPosition.r + (index % 2 === 0 ? 2 : -2) 
              },
              health: 1, // Can't be harmed
              isVisible: sanity < 70, // Only visible at low sanity
              behavior: 'stalk'
            });
          }
        });
        break;
        
      case 4: // Horror - Dragon proximity effects
        newMonsters.push({
          id: 'dragon_presence',
          name: 'Dragon Presence',
          type: 'dragon',
          position: { q: 0, r: 0 }, // Center of labyrinth
          health: 1000,
          isVisible: false, // Never directly visible
          behavior: 'proximity'
        });
        break;
    }
    
    return newMonsters;
  }, [currentStage, companions, playerPosition, sanity]);

  // Update monster list when stage changes
  useEffect(() => {
    setMonsters(stageMonsters);
  }, [stageMonsters]);

  // Handle monster behaviors
  useFrame((state, delta) => {
    const time = state.clock.elapsedTime;
    
    monsters.forEach((monster) => {
      const distance = Math.sqrt(
        (monster.position.q - playerPosition.q) ** 2 + 
        (monster.position.r - playerPosition.r) ** 2
      );
      
      switch (monster.behavior) {
        case 'whisper':
          // Whispering Shades cause sanity loss when near
          if (distance < 3 && Math.random() < 0.01) {
            adjustSanity(-1);
          }
          break;
          
        case 'stalk':
          // Trauma Echoes slowly move toward player
          if (distance > 1) {
            const moveChance = delta * 0.5;
            if (Math.random() < moveChance) {
              const deltaQ = playerPosition.q - monster.position.q;
              const deltaR = playerPosition.r - monster.position.r;
              
              setMonsters(prev => prev.map(m => 
                m.id === monster.id 
                  ? {
                      ...m,
                      position: {
                        q: m.position.q + Math.sign(deltaQ) * 0.1,
                        r: m.position.r + Math.sign(deltaR) * 0.1
                      }
                    }
                  : m
              ));
            }
          }
          break;
          
        case 'proximity':
          // Dragon presence effects
          if (currentStage === 4) {
            const proximity = Math.max(0, 1 - distance / 8);
            if (proximity > 0.5) {
              // Intense dragon proximity effects
              if (Math.random() < proximity * 0.005) {
                adjustSanity(-3);
              }
            }
          }
          break;
          
        case 'patrol':
          // Hollow NPCs patrol in set patterns
          const patrolPhase = Math.sin(time * 0.5 + monster.id.charCodeAt(0)) * 2;
          setMonsters(prev => prev.map(m => 
            m.id === monster.id 
              ? {
                  ...m,
                  position: {
                    q: m.position.q + Math.sin(patrolPhase) * 0.1,
                    r: m.position.r + Math.cos(patrolPhase) * 0.1
                  }
                }
              : m
          ));
          break;
      }
    });
  });

  // Trigger whisper effects for Whispering Shades
  useEffect(() => {
    if (whisperTimerRef.current) {
      clearInterval(whisperTimerRef.current);
    }
    
    const whisperShades = monsters.filter(m => m.type === 'whispering_shade');
    if (whisperShades.length > 0) {
      whisperTimerRef.current = setInterval(() => {
        const nearbyShades = whisperShades.filter(shade => {
          const distance = Math.sqrt(
            (shade.position.q - playerPosition.q) ** 2 + 
            (shade.position.r - playerPosition.r) ** 2
          );
          return distance < 4;
        });
        
        if (nearbyShades.length > 0 && Math.random() < 0.3) {
          // Trigger whisper effect (handled by AudioManager)
          console.log("Whispering Shade nearby...");
        }
      }, 3000);
    }

    return () => {
      if (whisperTimerRef.current) {
        clearInterval(whisperTimerRef.current);
      }
    };
  }, [monsters, playerPosition]);

  return (
    <group>
      {monsters
        .filter(monster => monster.isVisible)
        .map((monster) => {
          const worldPos = getHexPosition(monster.position.q, monster.position.r, 1);
          
          return (
            <MonsterRenderer 
              key={monster.id} 
              monster={monster} 
              worldPosition={worldPos}
              currentStage={currentStage}
            />
          );
        })}
    </group>
  );
}

interface MonsterRendererProps {
  monster: Monster;
  worldPosition: { x: number; y: number; z: number };
  currentStage: number;
}

function MonsterRenderer({ monster, worldPosition, currentStage }: MonsterRendererProps) {
  const meshRef = useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (!meshRef.current) return;
    
    const time = state.clock.elapsedTime;
    
    // Monster-specific animations
    switch (monster.type) {
      case 'hollow_npc':
        // Jerky, unnatural movement
        meshRef.current.rotation.y = Math.sin(time * 3) * 0.5;
        meshRef.current.position.y = 0.5 + Math.sin(time * 8) * 0.05;
        break;
        
      case 'trauma_echo':
        // Flickering, unstable appearance
        const opacity = 0.3 + Math.sin(time * 10) * 0.2;
        if (meshRef.current.material instanceof THREE.MeshLambertMaterial) {
          meshRef.current.material.opacity = opacity;
        }
        break;
    }
  });

  const getMonsterAppearance = () => {
    switch (monster.type) {
      case 'hollow_npc':
        return {
          color: '#616161',
          scale: [0.5, 1.2, 0.5] as [number, number, number],
          geometry: 'box'
        };
      case 'trauma_echo':
        return {
          color: '#424242',
          scale: [0.4, 0.8, 0.4] as [number, number, number],
          geometry: 'sphere'
        };
      default:
        return {
          color: '#212121',
          scale: [0.3, 0.6, 0.3] as [number, number, number],
          geometry: 'box'
        };
    }
  };

  const appearance = getMonsterAppearance();

  return (
    <mesh
      ref={meshRef}
      position={[worldPosition.x, appearance.scale[1] / 2, worldPosition.z]}
      scale={appearance.scale}
      castShadow
    >
      {appearance.geometry === 'sphere' ? (
        <sphereGeometry args={[0.5, 8, 8]} />
      ) : (
        <boxGeometry args={[1, 1, 1]} />
      )}
      <meshLambertMaterial 
        color={appearance.color}
        transparent={monster.type === 'trauma_echo'}
        opacity={monster.type === 'trauma_echo' ? 0.6 : 1}
      />
    </mesh>
  );
}