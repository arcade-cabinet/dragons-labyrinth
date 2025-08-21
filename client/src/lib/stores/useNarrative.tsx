import { create } from "zustand";

export interface Quest {
  id: string;
  description: string;
  tasks?: string[];
  stage: number;
  isCompleted?: boolean;
}

interface NarrativeState {
  currentStage: number; // 0=Peace, 1=Unease, 2=Dread, 3=Terror, 4=Horror
  currentQuest: Quest | null;
  completedQuests: string[];
  stageTransitionTimer: number;
  
  // Actions
  triggerStageTransition: () => void;
  startQuest: (quest: Quest) => void;
  completeQuest: () => void;
  reset: () => void;
}

export const useNarrative = create<NarrativeState>((set, get) => ({
  currentStage: 0,
  currentQuest: null,
  completedQuests: [],
  stageTransitionTimer: 0,
  
  triggerStageTransition: () => {
    set((state) => {
      const newStage = Math.min(4, state.currentStage + 1);
      return {
        currentStage: newStage,
        currentQuest: null, // Clear current quest when transitioning
        stageTransitionTimer: 0,
      };
    });
  },
  
  startQuest: (quest: Quest) => {
    set({ currentQuest: quest });
  },
  
  completeQuest: () => {
    const { currentQuest } = get();
    if (currentQuest) {
      set((state) => ({
        currentQuest: null,
        completedQuests: [...state.completedQuests, currentQuest.id],
      }));
    }
  },
  
  reset: () => {
    set({
      currentStage: 0,
      currentQuest: null,
      completedQuests: [],
      stageTransitionTimer: 0,
    });
  },
}));
