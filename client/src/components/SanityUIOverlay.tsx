import { useGameState } from '../lib/stores/useGameState';

export default function SanityUIOverlay() {
  const { sanity } = useGameState();
  const intensity = Math.max(0, (100 - sanity) / 100);

  if (intensity < 0.2) return null;

  return (
    <>
      {/* Screen distortion effects */}
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

      {/* Hallucination text */}
      {intensity > 0.4 && (
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
      )}

      {/* CSS for animations */}
      <style>{`
        @keyframes flicker {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.3; }
        }
      `}</style>
    </>
  );
}