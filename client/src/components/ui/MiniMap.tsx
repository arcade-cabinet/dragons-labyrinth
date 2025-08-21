import React, { useMemo } from 'react';
import { useGameState } from '../../lib/stores/useGameState';
import { useNarrative } from '../../lib/stores/useNarrative';

export const MiniMap: React.FC = () => {
  const { hexTiles, playerPosition, companions } = useGameState();
  const { currentStage } = useNarrative();

  // Calculate minimap bounds and scale
  const mapData = useMemo(() => {
    if (hexTiles.length === 0) return null;

    const minQ = Math.min(...hexTiles.map(t => t.q));
    const maxQ = Math.max(...hexTiles.map(t => t.q));
    const minR = Math.min(...hexTiles.map(t => t.r));
    const maxR = Math.max(...hexTiles.map(t => t.r));

    const width = maxQ - minQ + 1;
    const height = maxR - minR + 1;
    const scale = Math.min(120 / width, 120 / height);

    return { minQ, minR, scale, width, height };
  }, [hexTiles]);

  if (!mapData) return null;

  // Convert hex coordinates to minimap pixel coordinates
  const hexToPixel = (q: number, r: number) => {
    const x = (q - mapData.minQ) * mapData.scale;
    const y = (r - mapData.minR) * mapData.scale;
    return { x: x + 4, y: y + 4 };
  };

  // Get tile color based on type and stage
  const getTileColor = (tileType: string, stage: number) => {
    const baseColors: Record<string, string> = {
      grass: '#4CAF50',
      forest: '#2E7D32',
      stone: '#757575',
      water: '#1976D2',
      corrupted: '#7C4DFF',
      void: '#000000',
      village: '#8BC34A',
      ruins: '#616161'
    };

    let color = baseColors[tileType] || '#424242';
    
    // Darken based on stage progression
    if (stage >= 2) {
      color = darkenColor(color, 0.3);
    }
    if (stage >= 4) {
      color = darkenColor(color, 0.5);
    }

    return color;
  };

  const darkenColor = (hex: string, factor: number) => {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    
    const newR = Math.round(r * (1 - factor));
    const newG = Math.round(g * (1 - factor));
    const newB = Math.round(b * (1 - factor));
    
    return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`;
  };

  return (
    <div className="relative">
      <h4 className="text-white text-xs font-semibold mb-2 text-center">Map</h4>
      <div 
        className="relative border border-gray-500 bg-gray-900"
        style={{ 
          width: mapData.width * mapData.scale + 8, 
          height: mapData.height * mapData.scale + 8 
        }}
      >
        {/* Render tiles */}
        {hexTiles.map((tile) => {
          const pos = hexToPixel(tile.q, tile.r);
          const isPlayerTile = tile.q === playerPosition.q && tile.r === playerPosition.r;
          
          return (
            <div
              key={`${tile.q}_${tile.r}`}
              className={`absolute rounded-sm ${isPlayerTile ? 'ring-2 ring-yellow-400 ring-opacity-80' : ''}`}
              style={{
                left: pos.x,
                top: pos.y,
                width: mapData.scale - 1,
                height: mapData.scale - 1,
                backgroundColor: getTileColor(tile.type, currentStage),
                opacity: tile.type === 'void' ? 0.3 : 1
              }}
            />
          );
        })}

        {/* Player indicator */}
        <div
          className="absolute w-2 h-2 bg-yellow-400 rounded-full border border-yellow-600 animate-pulse"
          style={{
            left: hexToPixel(playerPosition.q, playerPosition.r).x + mapData.scale / 2 - 4,
            top: hexToPixel(playerPosition.q, playerPosition.r).y + mapData.scale / 2 - 4,
          }}
        />

        {/* Companion indicators */}
        {companions.map((companion, index) => {
          if (!companion.position) return null;
          const pos = hexToPixel(companion.position.q, companion.position.r);
          
          return (
            <div
              key={companion.id}
              className="absolute w-1.5 h-1.5 bg-green-400 rounded-full border border-green-600"
              style={{
                left: pos.x + mapData.scale / 2 - 3,
                top: pos.y + mapData.scale / 2 - 3,
              }}
            />
          );
        })}

        {/* Fog of war overlay for unexplored areas */}
        <div className="absolute inset-0 pointer-events-none">
          {hexTiles.map((tile) => {
            if (tile.isExplored) return null;
            
            const pos = hexToPixel(tile.q, tile.r);
            return (
              <div
                key={`fog_${tile.q}_${tile.r}`}
                className="absolute bg-black opacity-70"
                style={{
                  left: pos.x,
                  top: pos.y,
                  width: mapData.scale - 1,
                  height: mapData.scale - 1,
                }}
              />
            );
          })}
        </div>
      </div>

      {/* Stage indicator on minimap */}
      <div className="absolute top-0 right-0 bg-red-900 text-red-200 px-1 py-0.5 text-xs rounded-bl">
        Stage {currentStage}
      </div>
    </div>
  );
};