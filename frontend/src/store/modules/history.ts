import { defineStore } from 'pinia';

interface HistoryState {
  undoStack: HistoryAction[];
  redoStack: HistoryAction[];
  maxHistorySize: number;
}

export interface HistoryAction {
  id: string;
  type: string;
  description: string;
  timestamp: number;
  data: any;
  undoData?: any;
}

export const useHistoryStore = defineStore('history', {
  state: (): HistoryState => ({
    undoStack: [],
    redoStack: [],
    maxHistorySize: 50,
  }),

  getters: {
    canUndo: (state): boolean => state.undoStack.length > 0,
    canRedo: (state): boolean => state.redoStack.length > 0,
    
    undoCount: (state): number => state.undoStack.length,
    redoCount: (state): number => state.redoStack.length,
    
    lastAction: (state): HistoryAction | null => {
      return state.undoStack.length > 0 ? state.undoStack[state.undoStack.length - 1] : null;
    },
    
    historyList: (state): HistoryAction[] => {
      return [...state.undoStack].reverse();
    },
  },

  actions: {
    pushAction(action: Omit<HistoryAction, 'id' | 'timestamp'>) {
      const historyAction: HistoryAction = {
        ...action,
        id: `action_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        timestamp: Date.now(),
      };
      
      this.undoStack.push(historyAction);
      
      if (this.undoStack.length > this.maxHistorySize) {
        this.undoStack.shift();
      }
      
      this.redoStack = [];
    },

    undo(): HistoryAction | null {
      if (this.undoStack.length === 0) return null;
      
      const action = this.undoStack.pop()!;
      this.redoStack.push(action);
      
      return action;
    },

    redo(): HistoryAction | null {
      if (this.redoStack.length === 0) return null;
      
      const action = this.redoStack.pop()!;
      this.undoStack.push(action);
      
      return action;
    },

    clear() {
      this.undoStack = [];
      this.redoStack = [];
    },

    removeAction(actionId: string) {
      const undoIndex = this.undoStack.findIndex(a => a.id === actionId);
      if (undoIndex > -1) {
        this.undoStack.splice(undoIndex, 1);
      }
      
      const redoIndex = this.redoStack.findIndex(a => a.id === actionId);
      if (redoIndex > -1) {
        this.redoStack.splice(redoIndex, 1);
      }
    },

    getActionById(actionId: string): HistoryAction | null {
      const undoAction = this.undoStack.find(a => a.id === actionId);
      if (undoAction) return undoAction;
      
      const redoAction = this.redoStack.find(a => a.id === actionId);
      return redoAction || null;
    },

    setMaxHistorySize(size: number) {
      this.maxHistorySize = size;
      
      while (this.undoStack.length > this.maxHistorySize) {
        this.undoStack.shift();
      }
    },
  },
});
