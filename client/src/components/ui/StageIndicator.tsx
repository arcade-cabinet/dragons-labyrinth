import React from 'react';

interface StageIndicatorProps {
  stage: number;
}

export const StageIndicator: React.FC<StageIndicatorProps> = ({ stage }) => {
  const stageNames = ['Peace', 'Unease', 'Dread', 'Terror', 'Horror'];
  const stageColors = ['text-green-400', 'text-yellow-400', 'text-orange-400', 'text-red-400', 'text-purple-400'];
  
  return (
    <div className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600">
      <div className="text-center">
        <div className="text-xs text-gray-400 mb-1">Current Stage</div>
        <div className={`font-bold text-sm ${stageColors[stage] || 'text-gray-400'}`}>
          {stageNames[stage] || 'Unknown'}
        </div>
        <div className="text-xs text-gray-500">Stage {stage}</div>
      </div>
    </div>
  );
};