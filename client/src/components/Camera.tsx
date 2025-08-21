import { useRef, useEffect } from 'react';
import { useFrame, useThree } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
import { getHexPosition } from '../utils/hexUtils';

export default function Camera() {
  const { camera } = useThree();
  const { playerPosition } = useGameState();
  const { currentStage } = useNarrative();
  
  const targetPosition = useRef(new THREE.Vector3());
  const targetLookAt = useRef(new THREE.Vector3());

  useFrame((state, delta) => {
    const worldPos = getHexPosition(playerPosition.q, playerPosition.r, 1);
    
    // Adjust camera based on horror stage
    let cameraOffset = { x: 0, y: 8, z: 12 };
    let fov = 45;
    
    switch (currentStage) {
      case 0: // Peace
        cameraOffset = { x: 0, y: 8, z: 12 };
        fov = 45;
        break;
      case 1: // Unease
        cameraOffset = { x: 0, y: 7, z: 10 };
        fov = 50;
        break;
      case 2: // Dread
        cameraOffset = { x: 0, y: 6, z: 8 };
        fov = 55;
        break;
      case 3: // Terror
        cameraOffset = { x: 0, y: 4, z: 6 };
        fov = 60;
        break;
      case 4: // Horror - First person view in labyrinth
        cameraOffset = { x: 0, y: 1.5, z: 0 };
        fov = 75;
        break;
    }

    // Set camera position
    targetPosition.current.set(
      worldPos.x + cameraOffset.x,
      cameraOffset.y,
      worldPos.z + cameraOffset.z
    );
    
    targetLookAt.current.set(worldPos.x, 0, worldPos.z);
    
    // Smooth camera movement
    camera.position.lerp(targetPosition.current, delta * 2);
    
    // Look at player position
    const lookDirection = new THREE.Vector3();
    lookDirection.subVectors(targetLookAt.current, camera.position);
    lookDirection.normalize();
    
    camera.lookAt(targetLookAt.current);
    
    // Adjust FOV for horror progression
    if (camera instanceof THREE.PerspectiveCamera) {
      camera.fov = THREE.MathUtils.lerp(camera.fov, fov, delta * 2);
      camera.updateProjectionMatrix();
    }
  });

  return null;
}
