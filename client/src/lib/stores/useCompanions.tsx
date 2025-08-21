import { create } from "zustand";

export interface Companion {
  id: string;
  name: string;
  isActive: boolean;
  morale: number;
  color?: string;
  archetype?: string;
  backstory?: string;
}

interface CompanionState {
  companions: Companion[];
  
  // Actions
  addCompanion: (companion: Companion) => void;
  removeCompanion: (id: string) => void;
  updateCompanionMorale: (id: string, change: number) => void;
  setCompanionActive: (id: string, active: boolean) => void;
  reset: () => void;
}

export const useCompanions = create<CompanionState>((set, get) => ({
  companions: [],
  
  addCompanion: (companion: Companion) => {
    set((state) => ({
      companions: [...state.companions, companion]
    }));
  },
  
  removeCompanion: (id: string) => {
    set((state) => ({
      companions: state.companions.filter(c => c.id !== id)
    }));
  },
  
  updateCompanionMorale: (id: string, change: number) => {
    set((state) => ({
      companions: state.companions.map(companion =>
        companion.id === id
          ? { ...companion, morale: Math.max(0, Math.min(100, companion.morale + change)) }
          : companion
      )
    }));
  },
  
  setCompanionActive: (id: string, active: boolean) => {
    set((state) => ({
      companions: state.companions.map(companion =>
        companion.id === id
          ? { ...companion, isActive: active }
          : companion
      )
    }));
  },
  
  reset: () => {
    set({ companions: [] });
  },
}));
