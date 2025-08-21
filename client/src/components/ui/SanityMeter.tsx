import React, { useEffect, useState } from 'react';
import { useGameState } from '../../lib/stores/useGameState';
import { useNarrative } from '../../lib/stores/useNarrative';

export const SanityMeter: React.FC = () => {
  const { player } = useGameState();
  const { currentStage } = useNarrative();
  const [isFlashing, setIsFlashing] = useState(false);

  // Sanity decreases as stages progress
  const maxSanity = 100;
  const currentSanity = Math.max(0, maxSanity - (currentStage * 15) - (player.stressEvents || 0) * 5);
  const sanityPercentage = currentSanity / maxSanity;

  // Flash effect when sanity is critically low
  useEffect(() => {
    if (sanityPercentage <= 0.25) {
      const interval = setInterval(() => {
        setIsFlashing(prev => !prev);
      }, 500);
      return () => clearInterval(interval);
    } else {
      setIsFlashing(false);
    }
  }, [sanityPercentage]);

  // Get sanity color based on level
  const getSanityColor = () => {
    if (sanityPercentage > 0.75) return 'bg-green-500';
    if (sanityPercentage > 0.5) return 'bg-yellow-500';
    if (sanityPercentage > 0.25) return 'bg-orange-500';
    return 'bg-red-500';
  };

  // Get sanity status text
  const getSanityStatus = () => {
    if (sanityPercentage > 0.75) return 'Calm';
    if (sanityPercentage > 0.5) return 'Uneasy';
    if (sanityPercentage > 0.25) return 'Disturbed';
    return 'Terrified';
  };

  // Get mind state effects based on sanity
  const getMindEffects = () => {
    const effects = [];
    
    if (sanityPercentage <= 0.5) {
      effects.push('Hearing whispers');
    }
    if (sanityPercentage <= 0.35) {
      effects.push('Seeing shadows');
    }
    if (sanityPercentage <= 0.25) {
      effects.push('Reality distortion');
    }
    if (sanityPercentage <= 0.15) {
      effects.push('Hallucinations');
    }
    
    return effects;
  };

  const mindEffects = getMindEffects();

  return (
    <div className={`mb-2 ${isFlashing ? 'opacity-50' : 'opacity-100'} transition-opacity duration-200`}>
      <div className="flex justify-between items-center mb-1">
        <span className="text-purple-400 text-xs font-medium">Sanity</span>
        <span className="text-white text-xs">{Math.round(currentSanity)}/100</span>
      </div>
      
      {/* Sanity bar */}
      <div className="w-full bg-gray-700 rounded-full h-2 mb-1">
        <div 
          className={`${getSanityColor()} h-2 rounded-full transition-all duration-500 ${isFlashing ? 'animate-pulse' : ''}`}
          style={{ width: `${sanityPercentage * 100}%` }}
        />
      </div>
      
      {/* Sanity status */}
      <div className="flex justify-between items-center">
        <span className={`text-xs font-medium ${
          sanityPercentage > 0.5 ? 'text-green-400' : 
          sanityPercentage > 0.25 ? 'text-yellow-400' : 'text-red-400'
        }`}>
          {getSanityStatus()}
        </span>
        
        {/* Stage indicator for sanity context */}
        {currentStage >= 1 && (
          <span className="text-xs text-gray-400">
            Stage {currentStage}
          </span>
        )}
      </div>
      
      {/* Mind effects indicator */}
      {mindEffects.length > 0 && (
        <div className="mt-1 p-1 bg-red-900/50 rounded text-xs">
          <div className="text-red-300 font-medium mb-1">Mind Effects:</div>
          {mindEffects.map((effect, index) => (
            <div key={index} className="text-red-200 text-xs opacity-80">
              • {effect}
            </div>
          ))}
        </div>
      )}
      
      {/* Dragon proximity warning (Terror/Horror stages) */}
      {currentStage >= 3 && (
        <div className="mt-1 p-1 bg-purple-900/50 rounded text-xs animate-pulse">
          <div className="text-purple-300 text-xs">
            🐉 Ancient presence detected...
          </div>
        </div>
      )}
    </div>
  );
};