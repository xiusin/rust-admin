import { defineStore } from 'pinia';
import type { CanvasElement, EditorState, EditorTool, SlideElement } from '@/components/ppt/editor/types';

export const useEditorStore = defineStore('editor', {
  state: (): EditorState => ({
    selectedElementIds: [],
    currentTool: 'select',
    clipboard: null,
    zoom: 1,
    offset: { x: 0, y: 0 },
    gridSize: 10,
    showGrid: true,
    snapToGrid: true,
    snapToElement: true,
  }),

  getters: {
    hasSelection: (state): boolean => state.selectedElementIds.length > 0,
    
    isMultiSelect: (state): boolean => state.selectedElementIds.length > 1,
    
    selectedCount: (state): number => state.selectedElementIds.length,
  },

  actions: {
    setTool(tool: EditorTool) {
      this.currentTool = tool;
    },

    setSelection(elementIds: string[]) {
      this.selectedElementIds = elementIds;
    },

    addSelection(elementId: string) {
      if (!this.selectedElementIds.includes(elementId)) {
        this.selectedElementIds.push(elementId);
      }
    },

    removeSelection(elementId: string) {
      const index = this.selectedElementIds.indexOf(elementId);
      if (index > -1) {
        this.selectedElementIds.splice(index, 1);
      }
    },

    toggleSelection(elementId: string) {
      if (this.selectedElementIds.includes(elementId)) {
        this.removeSelection(elementId);
      } else {
        this.addSelection(elementId);
      }
    },

    clearSelection() {
      this.selectedElementIds = [];
    },

    setClipboard(element: CanvasElement | null) {
      this.clipboard = element ? JSON.parse(JSON.stringify(element)) : null;
    },

    setZoom(zoom: number) {
      this.zoom = Math.max(0.25, Math.min(3, zoom));
    },

    zoomIn() {
      this.zoom = Math.min(this.zoom * 1.2, 3);
    },

    zoomOut() {
      this.zoom = Math.max(this.zoom / 1.2, 0.25);
    },

    resetZoom() {
      this.zoom = 1;
    },

    setOffset(x: number, y: number) {
      this.offset = { x, y };
    },

    pan(dx: number, dy: number) {
      this.offset.x += dx;
      this.offset.y += dy;
    },

    toggleGrid() {
      this.showGrid = !this.showGrid;
    },

    toggleSnapToGrid() {
      this.snapToGrid = !this.snapToGrid;
    },

    toggleSnapToElement() {
      this.snapToElement = !this.snapToElement;
    },

    setGridSize(size: number) {
      this.gridSize = Math.max(5, Math.min(50, size));
    },
  },

  persist: {
    key: 'editor-store',
    paths: ['showGrid', 'snapToGrid', 'snapToElement', 'gridSize'],
  },
});
