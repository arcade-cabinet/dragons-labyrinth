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
    
    // Isometric camera angle for 2.5D view
    // 45 degrees rotation, 30 degrees elevation
    const distance = currentStage === 4 ? 2 : 12 - currentStage; // Closer as horror progresses
    const angleH = Math.PI / 4; // 45 degrees horizontal for isometric
    const angleV = Math.PI / 6; // 30 degrees vertical for 2.5D tilt
    
    let cameraOffset = { 
      x: distance * Math.cos(angleH) * Math.cos(angleV),
      y: distance * Math.sin(angleV), 
      z: distance * Math.sin(angleH) * Math.cos(angleV)
    };
    let fov = 45 + (currentStage * 5); // Wider FOV as horror progresses
    
    // Special case for Horror stage - much closer
    if (currentStage === 4) {
      cameraOffset = { x: 0, y: 1.5, z: 0 };
      fov = 75;
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
