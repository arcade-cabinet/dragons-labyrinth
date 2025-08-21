import { useState, useEffect, useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

interface HexCoord {
  q: number;
  r: number;
}

export default function PathfindingSystem() {
  const { playerPosition, setPlayerPosition, adjustSanity } = useGameState();
  const { currentStage } = useNarrative();
  const [selectedTile, setSelectedTile] = useState<HexCoord | null>(null);
  const [currentPath, setCurrentPath] = useState<HexCoord[]>([]);
  const [isMoving, setIsMoving] = useState(false);
  const [showConfirmation, setShowConfirmation] = useState(false);
  const moveProgress = useRef(0);
  const pathIndex = useRef(0);

  // Convert hex to world position
  const hexToWorld = (q: number, r: number, y: number = 0.1) => {
    const size = 1;
    const x = size * (Math.sqrt(3) * q + Math.sqrt(3)/2 * r);
    const z = size * (3/2 * r);
    return new THREE.Vector3(x, y, z);
  };

  // A* pathfinding for hexagonal grid
  const findPath = (start: HexCoord, end: HexCoord): HexCoord[] => {
    const openSet: HexCoord[] = [start];
    const closedSet: Set<string> = new Set();
    const cameFrom: Map<string, HexCoord> = new Map();
    const gScore: Map<string, number> = new Map();
    const fScore: Map<string, number> = new Map();
    
    const key = (coord: HexCoord) => `${coord.q},${coord.r}`;
    const hexDistance = (a: HexCoord, b: HexCoord) => 
      (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;
    
    gScore.set(key(start), 0);
    fScore.set(key(start), hexDistance(start, end));
    
    while (openSet.length > 0) {
      // Find node with lowest fScore
      let current = openSet[0];
      let lowestF = fScore.get(key(current)) || Infinity;
      
      for (const node of openSet) {
        const f = fScore.get(key(node)) || Infinity;
        if (f < lowestF) {
          current = node;
          lowestF = f;
        }
      }
      
      // Check if we reached the goal
      if (current.q === end.q && current.r === end.r) {
        // Reconstruct path
        const path: HexCoord[] = [];
        let temp: HexCoord | undefined = current;
        
        while (temp) {
          path.unshift(temp);
          temp = cameFrom.get(key(temp));
        }
        
        return path;
      }
      
      // Move current from open to closed
      openSet.splice(openSet.indexOf(current), 1);
      closedSet.add(key(current));
      
      // Check neighbors
      const neighbors = [
        { q: current.q + 1, r: current.r },
        { q: current.q - 1, r: current.r },
        { q: current.q, r: current.r + 1 },
        { q: current.q, r: current.r - 1 },
        { q: current.q + 1, r: current.r - 1 },
        { q: current.q - 1, r: current.r + 1 },
      ];
      
      for (const neighbor of neighbors) {
        // Check if within bounds (radius 12)
        const distance = (Math.abs(neighbor.q) + Math.abs(neighbor.r) + Math.abs(-neighbor.q - neighbor.r)) / 2;
        if (distance > 12) continue;
        
        if (closedSet.has(key(neighbor))) continue;
        
        const tentativeG = (gScore.get(key(current)) || 0) + 1;
        
        if (!openSet.some(n => n.q === neighbor.q && n.r === neighbor.r)) {
          openSet.push(neighbor);
        } else if (tentativeG >= (gScore.get(key(neighbor)) || Infinity)) {
          continue;
        }
        
        cameFrom.set(key(neighbor), current);
        gScore.set(key(neighbor), tentativeG);
        fScore.set(key(neighbor), tentativeG + hexDistance(neighbor, end));
      }
    }
    
    return []; // No path found
  };

  // Handle tile selection
  const handleTileClick = (q: number, r: number) => {
    if (isMoving) return;
    
    const path = findPath(playerPosition, { q, r });
    if (path.length > 1) {
      setSelectedTile({ q, r });
      setCurrentPath(path);
      setShowConfirmation(true);
    }
  };

  // Confirm movement
  const confirmMovement = () => {
    if (currentPath.length > 1) {
      setIsMoving(true);
      setShowConfirmation(false);
      pathIndex.current = 0;
      moveProgress.current = 0;
    }
  };

  // Cancel movement
  const cancelMovement = () => {
    setSelectedTile(null);
    setCurrentPath([]);
    setShowConfirmation(false);
  };

  // Animate movement along path
  useFrame((state, delta) => {
    if (!isMoving || currentPath.length === 0) return;
    
    moveProgress.current += delta * 2; // Movement speed
    
    if (moveProgress.current >= 1) {
      // Move to next tile in path
      pathIndex.current++;
      moveProgress.current = 0;
      
      if (pathIndex.current < currentPath.length) {
        const nextPos = currentPath[pathIndex.current];
        setPlayerPosition(nextPos);
        
        // Random encounter chance
        if (currentStage >= 1 && Math.random() < 0.1 + (currentStage * 0.05)) {
          // Shake screen and stop movement
          setIsMoving(false);
          setCurrentPath([]);
          triggerRandomEncounter();
        }
        
        // Sanity loss while moving in horror stages
        if (currentStage >= 3) {
          adjustSanity(-0.3);
        }
      } else {
        // Reached destination
        setIsMoving(false);
        setCurrentPath([]);
        setSelectedTile(null);
      }
    }
  });

  const triggerRandomEncounter = () => {
    // Screen shake effect handled by Camera component
    console.log("Random encounter triggered!");
    adjustSanity(-5);
  };

  // Export click handler for HexagonalWorld to use
  useEffect(() => {
    (window as any).handleHexClick = (q: number, r: number) => {
      handleTileClick(q, r);
      // Dispatch event for UI
      if (!isMoving) {
        const path = findPath(playerPosition, { q, r });
        if (path.length > 1) {
          window.dispatchEvent(new CustomEvent('pathSelected', { detail: { length: path.length } }));
        }
      }
    };
    (window as any).confirmPath = confirmMovement;
    (window as any).cancelPath = cancelMovement;
    
    return () => {
      delete (window as any).handleHexClick;
      delete (window as any).confirmPath;
      delete (window as any).cancelPath;
    };
  }, [playerPosition, isMoving, currentPath]);

  // Create path line geometry
  const pathLineGeometry = useMemo(() => {
    if (currentPath.length < 2) return null;
    
    const points = currentPath.map(coord => hexToWorld(coord.q, coord.r, 0.2));
    return new THREE.BufferGeometry().setFromPoints(points);
  }, [currentPath]);

  // Render path visualization
  if (currentPath.length > 1 && pathLineGeometry) {
    return (
      <>
        {/* Path line using basic THREE.js line */}
        <lineSegments geometry={pathLineGeometry}>
          <lineBasicMaterial 
            color={currentStage >= 3 ? "#B71C1C" : "#4CAF50"}
            linewidth={3}
            transparent
            opacity={isMoving ? 1 : 0.7}
          />
        </lineSegments>
        
        {/* Highlight tiles in path */}
        {currentPath.slice(1).map((coord, idx) => {
          const pos = hexToWorld(coord.q, coord.r, 0.05);
          return (
            <mesh
              key={`path_${idx}`}
              position={pos}
              rotation={[-Math.PI / 2, 0, 0]}
            >
              <ringGeometry args={[0.4, 0.8, 6]} />
              <meshBasicMaterial 
                color={currentStage >= 3 ? "#B71C1C" : "#4CAF50"}
                transparent
                opacity={0.5}
              />
            </mesh>
          );
        })}
      </>
    );
  }
  
  return null;
}