import { useEffect, useRef } from 'react';
import { useKeyboardControls } from '@react-three/drei';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

enum Controls {
  forward = 'forward',
  backward = 'backward',
  leftward = 'leftward',
  rightward = 'rightward',
  interact = 'interact',
}

export default function HexMovement() {
  const { playerPosition, setPlayerPosition, adjustSanity } = useGameState();
  const { currentStage } = useNarrative();
  const lastMoveTime = useRef(0);
  const moveDelay = 200; // ms between moves

  // Get keyboard state
  const [subscribe, getState] = useKeyboardControls<Controls>();

  useEffect(() => {
    const unsubscribe = subscribe(() => {
      const state = getState();
      const now = Date.now();
      
      // Throttle movement
      if (now - lastMoveTime.current < moveDelay) return;
      
      let newQ = playerPosition.q;
      let newR = playerPosition.r;
      let moved = false;

      // Hexagonal movement directions
      // NW (-1, 0), NE (0, -1), E (1, -1), SE (1, 0), SW (0, 1), W (-1, 1)
      if (state.forward) {
        // Move north (up-left)
        newQ -= 1;
        moved = true;
      } else if (state.backward) {
        // Move south (down-right)
        newQ += 1;
        moved = true;
      } else if (state.leftward) {
        // Move west-ish
        newR += 1;
        moved = true;
      } else if (state.rightward) {
        // Move east-ish
        newR -= 1;
        moved = true;
      }

      if (moved) {
        lastMoveTime.current = now;
        
        // Check if movement is within world bounds
        const distance = (Math.abs(newQ) + Math.abs(newR) + Math.abs(-newQ - newR)) / 2;
        if (distance <= 12) {
          setPlayerPosition({ q: newQ, r: newR });
          
          // Movement affects sanity in horror stages
          if (currentStage >= 3) {
            adjustSanity(-0.5);
          }
        }
      }
    });

    return unsubscribe;
  }, [subscribe, getState, playerPosition, setPlayerPosition, currentStage, adjustSanity]);

  // No visual component, just handles input
  return null;
}