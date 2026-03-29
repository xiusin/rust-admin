import { ref, computed } from 'vue';
import type { HistoryState, SlideElement } from '@/components/ppt/editor/types';

export function useHistory(maxHistorySize: number = 50) {
  const history = ref<HistoryState[]>([]);
  const currentIndex = ref(-1);

  const canUndo = computed(() => currentIndex.value > 0);
  const canRedo = computed(() => currentIndex.value < history.value.length - 1);
  const historyLength = computed(() => history.value.length);

  const pushState = (state: Omit<HistoryState, 'timestamp'>) => {
    const newState: HistoryState = {
      ...state,
      timestamp: Date.now(),
    };

    if (currentIndex.value < history.value.length - 1) {
      history.value = history.value.slice(0, currentIndex.value + 1);
    }

    if (history.value.length >= maxHistorySize) {
      history.value.shift();
      currentIndex.value--;
    }

    history.value.push(JSON.parse(JSON.stringify(newState)));
    currentIndex.value = history.value.length - 1;
  };

  const undo = (): HistoryState | null => {
    if (!canUndo.value) return null;
    
    currentIndex.value--;
    return JSON.parse(JSON.stringify(history.value[currentIndex.value]));
  };

  const redo = (): HistoryState | null => {
    if (!canRedo.value) return null;
    
    currentIndex.value++;
    return JSON.parse(JSON.stringify(history.value[currentIndex.value]));
  };

  const clear = () => {
    history.value = [];
    currentIndex.value = -1;
  };

  const getCurrentState = (): HistoryState | null => {
    if (currentIndex.value < 0 || currentIndex.value >= history.value.length) {
      return null;
    }
    return JSON.parse(JSON.stringify(history.value[currentIndex.value]));
  };

  const getStateAt = (index: number): HistoryState | null => {
    if (index < 0 || index >= history.value.length) {
      return null;
    }
    return JSON.parse(JSON.stringify(history.value[index]));
  };

  const jumpTo = (index: number): HistoryState | null => {
    if (index < 0 || index >= history.value.length) {
      return null;
    }
    currentIndex.value = index;
    return getCurrentState();
  };

  const getHistoryList = (): Array<{ index: number; action: string; timestamp: number }> => {
    return history.value.map((state, index) => ({
      index,
      action: state.action,
      timestamp: state.timestamp,
    }));
  };

  return {
    history,
    currentIndex,
    canUndo,
    canRedo,
    historyLength,
    pushState,
    undo,
    redo,
    clear,
    getCurrentState,
    getStateAt,
    jumpTo,
    getHistoryList,
  };
}
