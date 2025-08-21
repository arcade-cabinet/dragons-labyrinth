import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

export interface HexPosition {
  q: number;
  r: number;
}

interface GameState {
  // Player state
  playerPosition: HexPosition;
  sanity: number;
  movementSpeed: number;
  
  // Game progression
  isInitialized: boolean;
  
  // Actions
  initializeGame: () => void;
  setPlayerPosition: (position: HexPosition) => void;
  adjustSanity: (amount: number) => void;
  reset: () => void;
}

export const useGameState = create<GameState>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    playerPosition: { q: 0, r: 0 },
    sanity: 100,
    movementSpeed: 2,
    isInitialized: false,
    
    initializeGame: () => {
      set({
        playerPosition: { q: 0, r: 0 },
        sanity: 100,
        movementSpeed: 2,
        isInitialized: true,
      });
    },
    
    setPlayerPosition: (position: HexPosition) => {
      set({ playerPosition: position });
    },
    
    adjustSanity: (amount: number) => {
      set((state) => ({
        sanity: Math.max(0, Math.min(100, state.sanity + amount))
      }));
    },
    
    reset: () => {
      set({
        playerPosition: { q: 0, r: 0 },
        sanity: 100,
        movementSpeed: 2,
        isInitialized: false,
      });
    },
  }))
);
