import { useMemo } from 'react';
import { useTexture } from '@react-three/drei';
import * as THREE from 'three';
import { getHexPosition } from '../utils/hexUtils';

interface BiomeRendererProps {
  stage: number;
}

export default function BiomeRenderer({ stage }: BiomeRendererProps) {
  const woodTexture = useTexture("/textures/wood.jpg");
  
  // Generate stage-specific environmental objects
  const environmentObjects = useMemo(() => {
    const objects = [];
    const objectCount = Math.max(1, 15 - stage * 3); // Fewer objects as horror increases
    
    for (let i = 0; i < objectCount; i++) {
      const angle = (i / objectCount) * Math.PI * 2;
      const radius = 3 + Math.random() * 4;
      const x = Math.cos(angle) * radius;
      const z = Math.sin(angle) * radius;
      
      let objectType = 'tree';
      let color = '#4CAF50';
      let scale = [0.5, 2, 0.5];
      
      switch (stage) {
        case 0: // Peace - bright trees and flowers
          color = Math.random() > 0.5 ? '#4CAF50' : '#FFC107';
          break;
        case 1: // Unease - darker trees
          color = '#2E7D32';
          break;
        case 2: // Dread - dead trees and ruins
          color = '#424242';
          if (Math.random() > 0.7) {
            objectType = 'ruins';
            scale = [1, 0.5, 1];
            color = '#616161';
          }
          break;
        case 3: // Terror - twisted structures
          color = '#212121';
          scale = [0.3 + Math.random() * 0.4, 1 + Math.random(), 0.3 + Math.random() * 0.4];
          break;
        case 4: // Horror - labyrinth walls
          objectType = 'wall';
          color = '#0D0D0D';
          scale = [0.5, 3, 0.5];
          break;
      }
      
      objects.push({
        id: `env-${i}`,
        position: [x, scale[1] / 2, z],
        scale,
        color,
        objectType,
      });
    }
    
    return objects;
  }, [stage]);

  return (
    <group>
      {environmentObjects.map((obj) => (
        <mesh
          key={obj.id}
          position={obj.position as [number, number, number]}
          scale={obj.scale as [number, number, number]}
          castShadow
          receiveShadow
        >
          {obj.objectType === 'wall' ? (
            <boxGeometry args={[1, 1, 1]} />
          ) : (
            <cylinderGeometry args={[0.3, 0.5, 1, 8]} />
          )}
          <meshLambertMaterial 
            color={obj.color}
            map={obj.objectType === 'ruins' ? woodTexture : undefined}
          />
        </mesh>
      ))}
      
      {/* Stage-specific atmospheric effects */}
      {stage >= 2 && (
        <fog attach="fog" args={['#666666', 5, 25]} />
      )}
      
      {stage >= 4 && (
        <fog attach="fog" args={['#000000', 2, 10]} />
      )}
    </group>
  );
}
