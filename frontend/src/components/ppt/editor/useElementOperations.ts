import { usePPTStore } from '@/store/modules/ppt';
import { useEditorStore } from '@/store/modules/editor';
import type { CanvasElement, Point } from '@/components/ppt/editor/types';

export function useElementOperations() {
  const pptStore = usePPTStore();
  const editorStore = useEditorStore();

  const selectElement = (elementId: string, multi: boolean = false) => {
    if (multi) {
      editorStore.toggleSelection(elementId);
    } else {
      editorStore.setSelection([elementId]);
    }
  };

  const selectAll = () => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;
    
    const allElementIds = currentSlide.elements.map(el => el.id);
    editorStore.setSelection(allElementIds);
  };

  const deselectAll = () => {
    editorStore.clearSelection();
  };

  const moveElement = (elementId: string, dx: number, dy: number) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element && !element.locked) {
      element.x += dx;
      element.y += dy;
      
      if (editorStore.snapToGrid) {
        const gridSize = editorStore.gridSize;
        element.x = Math.round(element.x / gridSize) * gridSize;
        element.y = Math.round(element.y / gridSize) * gridSize;
      }
    }
  };

  const moveElements = (elementIds: string[], dx: number, dy: number) => {
    elementIds.forEach(id => moveElement(id, dx, dy));
  };

  const resizeElement = (
    elementId: string,
    width: number,
    height: number,
    maintainAspect: boolean = false
  ) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element && !element.locked) {
      if (maintainAspect) {
        const aspectRatio = element.width / element.height;
        if (width / height > aspectRatio) {
          width = height * aspectRatio;
        } else {
          height = width / aspectRatio;
        }
      }
      
      element.width = Math.max(10, width);
      element.height = Math.max(10, height);
    }
  };

  const rotateElement = (elementId: string, angle: number) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element && !element.locked) {
      element.rotation = angle % 360;
    }
  };

  const deleteElement = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const index = currentSlide.elements.findIndex(el => el.id === elementId);
    if (index > -1) {
      currentSlide.elements.splice(index, 1);
      editorStore.removeSelection(elementId);
    }
  };

  const deleteSelectedElements = () => {
    const selectedIds = [...editorStore.selectedElementIds];
    selectedIds.forEach(id => deleteElement(id));
  };

  const copyElement = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element) {
      editorStore.setClipboard(element);
    }
  };

  const copySelectedElements = () => {
    const selectedIds = editorStore.selectedElementIds;
    if (selectedIds.length === 1) {
      copyElement(selectedIds[0]);
    }
  };

  const pasteElement = () => {
    const clipboard = editorStore.clipboard;
    const currentSlide = pptStore.currentSlide;
    if (!clipboard || !currentSlide) return;

    const newElement: CanvasElement = {
      ...JSON.parse(JSON.stringify(clipboard)),
      id: `element-${Date.now()}`,
      x: clipboard.x + 20,
      y: clipboard.y + 20,
    };

    currentSlide.elements.push(newElement);
    editorStore.setSelection([newElement.id]);
  };

  const duplicateElement = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element) {
      const newElement: CanvasElement = {
        ...JSON.parse(JSON.stringify(element)),
        id: `element-${Date.now()}`,
        x: element.x + 20,
        y: element.y + 20,
      };

      currentSlide.elements.push(newElement);
      editorStore.setSelection([newElement.id]);
    }
  };

  const duplicateSelectedElements = () => {
    const selectedIds = [...editorStore.selectedElementIds];
    selectedIds.forEach(id => duplicateElement(id));
  };

  const bringToFront = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const index = currentSlide.elements.findIndex(el => el.id === elementId);
    if (index > -1 && index < currentSlide.elements.length - 1) {
      const [element] = currentSlide.elements.splice(index, 1);
      currentSlide.elements.push(element);
    }
  };

  const sendToBack = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const index = currentSlide.elements.findIndex(el => el.id === elementId);
    if (index > 0) {
      const [element] = currentSlide.elements.splice(index, 1);
      currentSlide.elements.unshift(element);
    }
  };

  const bringForward = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const index = currentSlide.elements.findIndex(el => el.id === elementId);
    if (index > -1 && index < currentSlide.elements.length - 1) {
      const [element] = currentSlide.elements.splice(index, 1);
      currentSlide.elements.splice(index + 1, 0, element);
    }
  };

  const sendBackward = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const index = currentSlide.elements.findIndex(el => el.id === elementId);
    if (index > 0) {
      const [element] = currentSlide.elements.splice(index, 1);
      currentSlide.elements.splice(index - 1, 0, element);
    }
  };

  const alignElements = (
    alignment: 'left' | 'center' | 'right' | 'top' | 'middle' | 'bottom'
  ) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const selectedIds = editorStore.selectedElementIds;
    if (selectedIds.length < 2) return;

    const elements = currentSlide.elements.filter(el => selectedIds.includes(el.id));
    if (elements.length < 2) return;

    const bounds = getElementsBounds(elements);

    elements.forEach(element => {
      if (element.locked) return;

      switch (alignment) {
        case 'left':
          element.x = bounds.x;
          break;
        case 'center':
          element.x = bounds.x + (bounds.width - element.width) / 2;
          break;
        case 'right':
          element.x = bounds.x + bounds.width - element.width;
          break;
        case 'top':
          element.y = bounds.y;
          break;
        case 'middle':
          element.y = bounds.y + (bounds.height - element.height) / 2;
          break;
        case 'bottom':
          element.y = bounds.y + bounds.height - element.height;
          break;
      }
    });
  };

  const distributeElements = (direction: 'horizontal' | 'vertical') => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const selectedIds = editorStore.selectedElementIds;
    if (selectedIds.length < 3) return;

    const elements = currentSlide.elements.filter(el => selectedIds.includes(el.id));
    if (elements.length < 3) return;

    if (direction === 'horizontal') {
      elements.sort((a, b) => a.x - b.x);
      const totalWidth = elements.reduce((sum, el) => sum + el.width, 0);
      const bounds = getElementsBounds(elements);
      const spacing = (bounds.width - totalWidth) / (elements.length - 1);

      let currentX = bounds.x;
      elements.forEach((element, index) => {
        if (index > 0 && !element.locked) {
          element.x = currentX;
        }
        currentX += element.width + spacing;
      });
    } else {
      elements.sort((a, b) => a.y - b.y);
      const totalHeight = elements.reduce((sum, el) => sum + el.height, 0);
      const bounds = getElementsBounds(elements);
      const spacing = (bounds.height - totalHeight) / (elements.length - 1);

      let currentY = bounds.y;
      elements.forEach((element, index) => {
        if (index > 0 && !element.locked) {
          element.y = currentY;
        }
        currentY += element.height + spacing;
      });
    }
  };

  const getElementsBounds = (elements: CanvasElement[]) => {
    if (elements.length === 0) {
      return { x: 0, y: 0, width: 0, height: 0 };
    }

    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;

    elements.forEach(element => {
      minX = Math.min(minX, element.x);
      minY = Math.min(minY, element.y);
      maxX = Math.max(maxX, element.x + element.width);
      maxY = Math.max(maxY, element.y + element.height);
    });

    return {
      x: minX,
      y: minY,
      width: maxX - minX,
      height: maxY - minY,
    };
  };

  const getElementAtPoint = (point: Point): CanvasElement | null => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return null;

    for (let i = currentSlide.elements.length - 1; i >= 0; i--) {
      const element = currentSlide.elements[i];
      if (
        !element.locked &&
        element.visible &&
        point.x >= element.x &&
        point.x <= element.x + element.width &&
        point.y >= element.y &&
        point.y <= element.y + element.height
      ) {
        return element;
      }
    }

    return null;
  };

  const toggleElementVisibility = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element) {
      element.visible = !element.visible;
    }
  };

  const toggleElementLock = (elementId: string) => {
    const currentSlide = pptStore.currentSlide;
    if (!currentSlide) return;

    const element = currentSlide.elements.find(el => el.id === elementId);
    if (element) {
      element.locked = !element.locked;
    }
  };

  return {
    selectElement,
    selectAll,
    deselectAll,
    moveElement,
    moveElements,
    resizeElement,
    rotateElement,
    deleteElement,
    deleteSelectedElements,
    copyElement,
    copySelectedElements,
    pasteElement,
    duplicateElement,
    duplicateSelectedElements,
    bringToFront,
    sendToBack,
    bringForward,
    sendBackward,
    alignElements,
    distributeElements,
    getElementsBounds,
    getElementAtPoint,
    toggleElementVisibility,
    toggleElementLock,
  };
}
