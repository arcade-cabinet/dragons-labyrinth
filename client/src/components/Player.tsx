import { useRef, useEffect } from 'react';
import { useFrame, useThree } from '@react-three/fiber';
import { useKeyboardControls } from '@react-three/drei';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useCompanions } from '../lib/stores/useCompanions';
import { getHexPosition } from '../utils/hexUtils';

enum Controls {
  forward = 'forward',
  backward = 'backward',
  leftward = 'leftward',
  rightward = 'rightward',
  interact = 'interact',
  menu = 'menu',
}

export default function Player() {
  const meshRef = useRef<THREE.Mesh>(null);
  const { playerPosition, setPlayerPosition, movementSpeed } = useGameState();
  const { companions } = useCompanions();
  const [subscribeKeys, getKeys] = useKeyboardControls<Controls>();

  // Hex movement directions
  const HEX_DIRECTIONS = [
    { q: 1, r: 0 },   // East
    { q: 1, r: -1 },  // Northeast
    { q: 0, r: -1 },  // Northwest
    { q: -1, r: 0 },  // West
    { q: -1, r: 1 },  // Southwest
    { q: 0, r: 1 },   // Southeast
  ];

  const velocity = useRef(new THREE.Vector3());
  const targetPosition = useRef(new THREE.Vector3());

  useEffect(() => {
    if (meshRef.current) {
      const worldPos = getHexPosition(playerPosition.q, playerPosition.r, 1);
      targetPosition.current.set(worldPos.x, 0.5, worldPos.z);
      meshRef.current.position.copy(targetPosition.current);
    }
  }, [playerPosition]);

  useFrame((state, delta) => {
    if (!meshRef.current) return;

    const { forward, backward, leftward, rightward, interact } = getKeys();
    
    let moveDirection = null;
    
    // Determine movement direction based on input
    if (forward && rightward) {
      moveDirection = HEX_DIRECTIONS[1]; // Northeast
    } else if (forward && leftward) {
      moveDirection = HEX_DIRECTIONS[2]; // Northwest
    } else if (backward && rightward) {
      moveDirection = HEX_DIRECTIONS[5]; // Southeast
    } else if (backward && leftward) {
      moveDirection = HEX_DIRECTIONS[4]; // Southwest
    } else if (forward) {
      moveDirection = HEX_DIRECTIONS[2]; // Northwest (toward camera)
    } else if (backward) {
      moveDirection = HEX_DIRECTIONS[5]; // Southeast (away from camera)
    } else if (rightward) {
      moveDirection = HEX_DIRECTIONS[0]; // East
    } else if (leftward) {
      moveDirection = HEX_DIRECTIONS[3]; // West
    }

    // Apply movement
    if (moveDirection) {
      const newQ = playerPosition.q + moveDirection.q;
      const newR = playerPosition.r + moveDirection.r;
      
      // Check bounds (simple circular world)
      const distance = Math.sqrt(newQ * newQ + newR * newR + newQ * newR);
      if (distance <= 8) {
        setPlayerPosition({ q: newQ, r: newR });
      }
    }

    // Smooth position interpolation
    const worldPos = getHexPosition(playerPosition.q, playerPosition.r, 1);
    targetPosition.current.set(worldPos.x, 0.5, worldPos.z);
    
    meshRef.current.position.lerp(targetPosition.current, delta * 8);

    // Handle interaction
    if (interact) {
      console.log("Player interaction at", playerPosition);
      // Check for nearby companions or interactive objects
    }
  });

  return (
    <>
      {/* Player character */}
      <mesh ref={meshRef} castShadow>
        <boxGeometry args={[0.6, 1.2, 0.6]} />
        <meshLambertMaterial color="#4FC3F7" />
      </mesh>
      
      {/* Render companions */}
      {companions.map((companion, index) => {
        const companionPos = getHexPosition(
          playerPosition.q + (index + 1) % 3 - 1,
          playerPosition.r + Math.floor((index + 1) / 3) - 1,
          1
        );
        
        return (
          <mesh
            key={companion.id}
            position={[companionPos.x, 0.5, companionPos.z]}
            castShadow
          >
            <boxGeometry args={[0.5, 1, 0.5]} />
            <meshLambertMaterial color={companion.color || "#FFA726"} />
          </mesh>
        );
      })}
    </>
  );
}
