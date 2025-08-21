import { useState, useEffect } from 'react';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
import { useCompanions } from '../lib/stores/useCompanions';
import DialogueSystem from './DialogueSystem';

export default function GameUI() {
  const { playerPosition, sanity } = useGameState();
  const { currentStage, currentQuest } = useNarrative();
  const { companions } = useCompanions();
  
  // UI style changes based on horror stage
  const getUIStyle = (stage: number) => {
    switch (stage) {
      case 0: // Peace
        return {
          background: 'rgba(255, 255, 255, 0.9)',
          color: '#333',
          border: '2px solid #4CAF50',
        };
      case 1: // Unease
        return {
          background: 'rgba(240, 240, 240, 0.8)',
          color: '#444',
          border: '2px solid #FFA726',
        };
      case 2: // Dread
        return {
          background: 'rgba(200, 200, 200, 0.7)',
          color: '#555',
          border: '2px solid #FF7043',
        };
      case 3: // Terror
        return {
          background: 'rgba(100, 100, 100, 0.6)',
          color: '#CCC',
          border: '2px solid #F44336',
        };
      case 4: // Horror
        return {
          background: 'rgba(50, 50, 50, 0.5)',
          color: '#FFF',
          border: '2px solid #B71C1C',
        };
      default:
        return {
          background: 'rgba(255, 255, 255, 0.9)',
          color: '#333',
          border: '2px solid #4CAF50',
        };
    }
  };

  const uiStyle = getUIStyle(currentStage);
  const stageNames = ['Peace', 'Unease', 'Dread', 'Terror', 'Horror'];

  return (
    <>
      {/* Main HUD */}
      <div
        style={{
          position: 'absolute',
          top: '20px',
          left: '20px',
          padding: '15px',
          borderRadius: '8px',
          fontFamily: 'Inter, sans-serif',
          fontSize: '14px',
          minWidth: '200px',
          ...uiStyle,
        }}
      >
        <div style={{ marginBottom: '10px', fontWeight: 'bold', fontSize: '16px' }}>
          Dragon's Labyrinth
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          Stage: {stageNames[currentStage]} ({currentStage}/4)
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          Position: ({playerPosition.q}, {playerPosition.r})
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          Sanity: {Math.round(sanity)}%
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          Companions: {companions.filter(c => c.isActive).length}
        </div>
      </div>

      {/* Quest Display */}
      {currentQuest && (
        <div
          style={{
            position: 'absolute',
            top: '20px',
            right: '20px',
            padding: '15px',
            borderRadius: '8px',
            fontFamily: 'Inter, sans-serif',
            fontSize: '14px',
            maxWidth: '300px',
            ...uiStyle,
          }}
        >
          <div style={{ fontWeight: 'bold', marginBottom: '8px' }}>
            Current Quest
          </div>
          <div style={{ marginBottom: '8px' }}>
            {currentQuest.description}
          </div>
          {currentQuest.tasks && (
            <div>
              <div style={{ fontWeight: 'bold', marginBottom: '4px' }}>Tasks:</div>
              {currentQuest.tasks.map((task, index) => (
                <div key={index} style={{ marginBottom: '2px', fontSize: '12px' }}>
                  • {task}
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Controls Help */}
      <div
        style={{
          position: 'absolute',
          bottom: '20px',
          left: '20px',
          padding: '10px',
          borderRadius: '8px',
          fontFamily: 'Inter, sans-serif',
          fontSize: '12px',
          ...uiStyle,
        }}
      >
        <div>WASD/Arrows: Move</div>
        <div>E/Space: Interact</div>
        <div>Esc: Menu</div>
      </div>

      {/* Companion Status */}
      {companions.length > 0 && (
        <div
          style={{
            position: 'absolute',
            bottom: '20px',
            right: '20px',
            padding: '10px',
            borderRadius: '8px',
            fontFamily: 'Inter, sans-serif',
            fontSize: '12px',
            maxWidth: '200px',
            ...uiStyle,
          }}
        >
          <div style={{ fontWeight: 'bold', marginBottom: '8px' }}>
            Companions
          </div>
          {companions.map((companion) => (
            <div
              key={companion.id}
              style={{
                marginBottom: '4px',
                opacity: companion.isActive ? 1 : 0.5,
              }}
            >
              {companion.name}: {companion.morale.toFixed(0)}% morale
            </div>
          ))}
        </div>
      )}

      <DialogueSystem />
    </>
  );
}
