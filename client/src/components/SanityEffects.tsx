import { useMemo, useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

export default function SanityEffects() {
  const { sanity } = useGameState();
  const { currentStage } = useNarrative();
  const hallucinationRefs = useRef<THREE.Mesh[]>([]);

  // Generate hallucinations based on sanity level
  const hallucinations = useMemo(() => {
    const effects = [];
    const intensity = Math.max(0, (100 - sanity) / 100);
    
    if (intensity > 0.3) {
      // Add floating dark objects
      for (let i = 0; i < Math.floor(intensity * 8); i++) {
        const angle = (i / 8) * Math.PI * 2 + Date.now() * 0.001;
        const radius = 6 + Math.sin(Date.now() * 0.002 + i) * 2;
        const x = Math.cos(angle) * radius;
        const z = Math.sin(angle) * radius;
        const y = 2 + Math.sin(Date.now() * 0.003 + i) * 1;
        
        effects.push({
          id: `hallucination_${i}`,
          position: [x, y, z] as [number, number, number],
          opacity: intensity * 0.4,
          scale: 0.3 + Math.random() * 0.4,
        });
      }
    }
    
    if (intensity > 0.6) {
      // Add shadow figures (companion echoes)
      for (let i = 0; i < 3; i++) {
        const x = (i - 1) * 4 + Math.sin(Date.now() * 0.002 + i) * 2;
        const z = 8 + Math.cos(Date.now() * 0.002 + i) * 2;
        
        effects.push({
          id: `shadow_${i}`,
          position: [x, 0.5, z] as [number, number, number],
          opacity: intensity * 0.6,
          scale: 0.8,
        });
      }
    }
    
    return effects;
  }, [sanity]);

  // Animate hallucinations
  useFrame((state) => {
    hallucinationRefs.current.forEach((mesh, index) => {
      if (mesh) {
        const time = state.clock.elapsedTime;
        
        // Floating motion
        mesh.position.y += Math.sin(time * 2 + index) * 0.01;
        
        // Flickering opacity
        if (mesh.material instanceof THREE.MeshBasicMaterial) {
          const baseOpacity = hallucinations[index]?.opacity || 0.3;
          mesh.material.opacity = baseOpacity * (0.5 + 0.5 * Math.sin(time * 5 + index));
        }
        
        // Subtle rotation
        mesh.rotation.y = time * 0.5 + index;
      }
    });
  });

  // Screen distortion effects based on sanity
  const DistortionOverlay = () => {
    const intensity = Math.max(0, (100 - sanity) / 100);
    
    if (intensity < 0.2) return null;
    
    return (
      <div
        style={{
          position: 'absolute',
          top: '0',
          left: '0',
          width: '100%',
          height: '100%',
          pointerEvents: 'none',
          zIndex: 1000,
          background: `radial-gradient(circle at center, 
            transparent ${60 + intensity * 30}%, 
            rgba(0, 0, 0, ${intensity * 0.4}) 100%)`,
          mixBlendMode: 'multiply',
        }}
      />
    );
  };

  // Audio-visual hallucination indicators
  const HallucinationUI = () => {
    const intensity = Math.max(0, (100 - sanity) / 100);
    
    if (intensity < 0.4) return null;
    
    return (
      <div
        style={{
          position: 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          color: '#F44336',
          fontSize: '18px',
          fontFamily: 'Inter, sans-serif',
          textAlign: 'center',
          opacity: intensity * 0.6,
          textShadow: '2px 2px 4px rgba(0, 0, 0, 0.8)',
          animation: intensity > 0.7 ? 'flicker 0.5s infinite' : 'none',
        }}
      >
        {intensity > 0.8 && "Do you hear that?"}
        {intensity > 0.6 && intensity <= 0.8 && "Something is watching..."}
        {intensity > 0.4 && intensity <= 0.6 && "The shadows move..."}
      </div>
    );
  };

  return (
    <group>
      {/* 3D Hallucinations */}
      {hallucinations.map((hallucination, index) => (
        <mesh
          key={hallucination.id}
          ref={(el) => {
            if (el) hallucinationRefs.current[index] = el;
          }}
          position={hallucination.position}
          scale={hallucination.scale}
        >
          <sphereGeometry args={[0.5, 8, 8]} />
          <meshBasicMaterial 
            color="#212121"
            transparent
            opacity={hallucination.opacity}
          />
        </mesh>
      ))}
    </group>
  );
}