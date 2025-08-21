import { useState, useEffect } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';

export default function PathConfirmationUI() {
  const [showConfirmation, setShowConfirmation] = useState(false);
  const [pathLength, setPathLength] = useState(0);
  const { currentStage } = useNarrative();

  useEffect(() => {
    // Listen for path selection events
    const handlePathSelected = (event: CustomEvent) => {
      setShowConfirmation(true);
      setPathLength(event.detail.length);
    };

    window.addEventListener('pathSelected' as any, handlePathSelected);
    
    return () => {
      window.removeEventListener('pathSelected' as any, handlePathSelected);
    };
  }, []);

  const confirmMovement = () => {
    if ((window as any).confirmPath) {
      (window as any).confirmPath();
    }
    setShowConfirmation(false);
  };

  const cancelMovement = () => {
    if ((window as any).cancelPath) {
      (window as any).cancelPath();
    }
    setShowConfirmation(false);
  };

  if (!showConfirmation) return null;

  const stageColors = {
    0: { bg: '#4CAF50', hover: '#45A049' }, // Peace - Green
    1: { bg: '#FF9800', hover: '#F57C00' }, // Unease - Orange
    2: { bg: '#795548', hover: '#5D4037' }, // Dread - Brown
    3: { bg: '#B71C1C', hover: '#8B0000' }, // Terror - Red
    4: { bg: '#311B92', hover: '#1A237E' }, // Horror - Dark Purple
  };

  const colors = stageColors[currentStage as keyof typeof stageColors] || stageColors[0];

  return (
    <div
      style={{
        position: 'absolute',
        bottom: '120px',
        left: '50%',
        transform: 'translateX(-50%)',
        backgroundColor: 'rgba(0, 0, 0, 0.85)',
        padding: '16px 24px',
        borderRadius: '12px',
        border: `2px solid ${colors.bg}`,
        boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: '12px',
        zIndex: 2000,
        minWidth: '250px',
      }}
    >
      <div
        style={{
          color: '#FFFFFF',
          fontSize: '16px',
          fontFamily: 'Inter, sans-serif',
          textAlign: 'center',
        }}
      >
        Move to selected tile?
        <div style={{ fontSize: '12px', opacity: 0.8, marginTop: '4px' }}>
          {pathLength} steps away
        </div>
      </div>
      
      <div
        style={{
          display: 'flex',
          gap: '12px',
          width: '100%',
        }}
      >
        <button
          onClick={confirmMovement}
          onTouchEnd={(e) => {
            e.preventDefault();
            confirmMovement();
          }}
          style={{
            flex: 1,
            padding: '10px 20px',
            backgroundColor: colors.bg,
            color: '#FFFFFF',
            border: 'none',
            borderRadius: '8px',
            fontSize: '14px',
            fontFamily: 'Inter, sans-serif',
            fontWeight: 'bold',
            cursor: 'pointer',
            transition: 'background-color 0.2s',
            WebkitTapHighlightColor: 'transparent',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = colors.hover;
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = colors.bg;
          }}
        >
          Move
        </button>
        
        <button
          onClick={cancelMovement}
          onTouchEnd={(e) => {
            e.preventDefault();
            cancelMovement();
          }}
          style={{
            flex: 1,
            padding: '10px 20px',
            backgroundColor: '#424242',
            color: '#FFFFFF',
            border: 'none',
            borderRadius: '8px',
            fontSize: '14px',
            fontFamily: 'Inter, sans-serif',
            fontWeight: 'bold',
            cursor: 'pointer',
            transition: 'background-color 0.2s',
            WebkitTapHighlightColor: 'transparent',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = '#616161';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = '#424242';
          }}
        >
          Cancel
        </button>
      </div>
      
      {currentStage >= 1 && (
        <div
          style={{
            fontSize: '11px',
            color: '#FFA726',
            opacity: 0.8,
            textAlign: 'center',
            fontStyle: 'italic',
          }}
        >
          {currentStage >= 3 
            ? "Something might be waiting..." 
            : "Watch for encounters"}
        </div>
      )}
    </div>
  );
}