import { ref, reactive, computed, watch, type Ref } from 'vue';
import { useEditorStore } from '@/store/modules/editor';
import type { CanvasElement, Point, Rect, TransformHandle } from '@/components/ppt/editor/types';

interface CanvasState {
  zoom: number;
  offsetX: number;
  offsetY: number;
  isDragging: boolean;
  isPanning: boolean;
  dragStart: Point;
  selectedHandle: TransformHandle | null;
}

const HANDLE_SIZE = 8;
const ROTATE_HANDLE_OFFSET = 20;

export function useCanvas(
  canvasRef: Ref<HTMLCanvasElement | undefined>,
  containerRef: Ref<HTMLElement | undefined>
) {
  const editorStore = useEditorStore();
  
  const state = reactive<CanvasState>({
    zoom: 1,
    offsetX: 0,
    offsetY: 0,
    isDragging: false,
    isPanning: false,
    dragStart: { x: 0, y: 0 },
    selectedHandle: null,
  });

  const context = ref<CanvasRenderingContext2D | null>(null);
  const animationFrame = ref<number | null>(null);

  const canvasWidth = computed(() => containerRef.value?.clientWidth || 960);
  const canvasHeight = computed(() => containerRef.value?.clientHeight || 540);

  const initCanvas = () => {
    if (!canvasRef.value || !containerRef.value) return;

    const canvas = canvasRef.value;
    const container = containerRef.value;
    const dpr = window.devicePixelRatio || 1;

    canvas.width = container.clientWidth * dpr;
    canvas.height = container.clientHeight * dpr;
    canvas.style.width = `${container.clientWidth}px`;
    canvas.style.height = `${container.clientHeight}px`;

    const ctx = canvas.getContext('2d');
    if (ctx) {
      ctx.scale(dpr, dpr);
      context.value = ctx;
    }

    bindEvents();
    render();
  };

  const destroyCanvas = () => {
    if (animationFrame.value) {
      cancelAnimationFrame(animationFrame.value);
    }
    unbindEvents();
  };

  const bindEvents = () => {
    if (!canvasRef.value) return;
    
    const canvas = canvasRef.value;
    canvas.addEventListener('mousedown', handleMouseDown);
    canvas.addEventListener('mousemove', handleMouseMove);
    canvas.addEventListener('mouseup', handleMouseUp);
    canvas.addEventListener('wheel', handleWheel, { passive: false });
    canvas.addEventListener('dblclick', handleDoubleClick);
    
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('resize', handleResize);
  };

  const unbindEvents = () => {
    if (!canvasRef.value) return;
    
    const canvas = canvasRef.value;
    canvas.removeEventListener('mousedown', handleMouseDown);
    canvas.removeEventListener('mousemove', handleMouseMove);
    canvas.removeEventListener('mouseup', handleMouseUp);
    canvas.removeEventListener('wheel', handleWheel);
    canvas.removeEventListener('dblclick', handleDoubleClick);
    
    window.removeEventListener('keydown', handleKeyDown);
    window.removeEventListener('resize', handleResize);
  };

  const handleMouseDown = (e: MouseEvent) => {
    const point = screenToCanvas(e.clientX, e.clientY);
    state.dragStart = point;
    state.isDragging = true;

    if (editorStore.currentTool === 'pan' || e.button === 1) {
      state.isPanning = true;
      return;
    }

    if (editorStore.currentTool === 'select') {
      const handle = getTransformHandle(point);
      if (handle) {
        state.selectedHandle = handle;
        return;
      }
    }
  };

  const handleMouseMove = (e: MouseEvent) => {
    const point = screenToCanvas(e.clientX, e.clientY);

    if (state.isPanning) {
      const dx = e.movementX;
      const dy = e.movementY;
      pan(dx, dy);
      return;
    }

    if (state.isDragging && editorStore.currentTool === 'select') {
      const dx = point.x - state.dragStart.x;
      const dy = point.y - state.dragStart.y;
      
      if (state.selectedHandle) {
        // TODO: Handle transform
      } else {
        // TODO: Move selected elements
      }
      
      state.dragStart = point;
      render();
    }

    updateCursor(point);
  };

  const handleMouseUp = () => {
    state.isDragging = false;
    state.isPanning = false;
    state.selectedHandle = null;
  };

  const handleWheel = (e: WheelEvent) => {
    e.preventDefault();
    
    if (e.ctrlKey || e.metaKey) {
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      const newZoom = state.zoom * delta;
      zoomAt(e.clientX, e.clientY, newZoom);
    } else {
      pan(-e.deltaX, -e.deltaY);
    }
  };

  const handleDoubleClick = (e: MouseEvent) => {
    const point = screenToCanvas(e.clientX, e.clientY);
    // TODO: Handle double click for text editing
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.ctrlKey || e.metaKey) {
      switch (e.key) {
        case '=':
        case '+':
          e.preventDefault();
          zoomIn();
          break;
        case '-':
          e.preventDefault();
          zoomOut();
          break;
        case '0':
          e.preventDefault();
          resetZoom();
          break;
      }
    }
  };

  const handleResize = () => {
    initCanvas();
  };

  const screenToCanvas = (screenX: number, screenY: number): Point => {
    if (!canvasRef.value) return { x: 0, y: 0 };
    
    const rect = canvasRef.value.getBoundingClientRect();
    return {
      x: (screenX - rect.left - state.offsetX) / state.zoom,
      y: (screenY - rect.top - state.offsetY) / state.zoom,
    };
  };

  const canvasToScreen = (canvasX: number, canvasY: number): Point => {
    if (!canvasRef.value) return { x: 0, y: 0 };
    
    const rect = canvasRef.value.getBoundingClientRect();
    return {
      x: canvasX * state.zoom + state.offsetX + rect.left,
      y: canvasY * state.zoom + state.offsetY + rect.top,
    };
  };

  const getTransformHandle = (point: Point): TransformHandle | null => {
    // TODO: Implement transform handle detection
    return null;
  };

  const updateCursor = (point: Point) => {
    if (!canvasRef.value) return;
    
    if (editorStore.currentTool === 'pan') {
      canvasRef.value.style.cursor = state.isPanning ? 'grabbing' : 'grab';
      return;
    }

    const handle = getTransformHandle(point);
    if (handle) {
      canvasRef.value.style.cursor = handle.cursor;
      return;
    }

    canvasRef.value.style.cursor = 'default';
  };

  const zoomIn = () => {
    state.zoom = Math.min(state.zoom * 1.2, 3);
    editorStore.setZoom(state.zoom);
    render();
  };

  const zoomOut = () => {
    state.zoom = Math.max(state.zoom / 1.2, 0.25);
    editorStore.setZoom(state.zoom);
    render();
  };

  const resetZoom = () => {
    state.zoom = 1;
    editorStore.setZoom(1);
    render();
  };

  const zoomAt = (screenX: number, screenY: number, newZoom: number) => {
    if (!canvasRef.value) return;
    
    const rect = canvasRef.value.getBoundingClientRect();
    const mouseX = screenX - rect.left;
    const mouseY = screenY - rect.top;
    
    const oldZoom = state.zoom;
    state.zoom = Math.max(0.25, Math.min(3, newZoom));
    
    state.offsetX = mouseX - (mouseX - state.offsetX) * (state.zoom / oldZoom);
    state.offsetY = mouseY - (mouseY - state.offsetY) * (state.zoom / oldZoom);
    
    editorStore.setZoom(state.zoom);
    render();
  };

  const pan = (dx: number, dy: number) => {
    state.offsetX += dx;
    state.offsetY += dy;
    editorStore.pan(dx, dy);
    render();
  };

  const render = () => {
    if (animationFrame.value) {
      cancelAnimationFrame(animationFrame.value);
    }
    
    animationFrame.value = requestAnimationFrame(() => {
      if (!context.value) return;
      
      const ctx = context.value;
      const width = canvasWidth.value;
      const height = canvasHeight.value;
      
      ctx.clearRect(0, 0, width, height);
      
      ctx.save();
      ctx.translate(state.offsetX, state.offsetY);
      ctx.scale(state.zoom, state.zoom);
      
      drawGrid(ctx, width, height);
      drawSlide(ctx);
      drawElements(ctx);
      drawSelection(ctx);
      
      ctx.restore();
    });
  };

  const drawGrid = (ctx: CanvasRenderingContext2D, width: number, height: number) => {
    if (!editorStore.showGrid) return;
    
    const gridSize = editorStore.gridSize;
    ctx.strokeStyle = '#e0e0e0';
    ctx.lineWidth = 0.5;
    
    const startX = Math.floor(-state.offsetX / state.zoom / gridSize) * gridSize;
    const startY = Math.floor(-state.offsetY / state.zoom / gridSize) * gridSize;
    const endX = startX + width / state.zoom + gridSize;
    const endY = startY + height / state.zoom + gridSize;
    
    ctx.beginPath();
    for (let x = startX; x <= endX; x += gridSize) {
      ctx.moveTo(x, startY);
      ctx.lineTo(x, endY);
    }
    for (let y = startY; y <= endY; y += gridSize) {
      ctx.moveTo(startX, y);
      ctx.lineTo(endX, y);
    }
    ctx.stroke();
  };

  const drawSlide = (ctx: CanvasRenderingContext2D) => {
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, 960, 540);
    ctx.strokeStyle = '#d0d0d0';
    ctx.lineWidth = 1;
    ctx.strokeRect(0, 0, 960, 540);
  };

  const drawElements = (ctx: CanvasRenderingContext2D) => {
    // TODO: Draw actual elements
  };

  const drawSelection = (ctx: CanvasRenderingContext2D) => {
    if (editorStore.selectedElementIds.length === 0) return;
    
    ctx.strokeStyle = '#165dff';
    ctx.lineWidth = 2 / state.zoom;
    ctx.setLineDash([]);
    
    // TODO: Draw selection boxes for selected elements
  };

  const drawTransformHandles = (ctx: CanvasRenderingContext2D, rect: Rect) => {
    const handleSize = HANDLE_SIZE / state.zoom;
    
    ctx.fillStyle = '#ffffff';
    ctx.strokeStyle = '#165dff';
    ctx.lineWidth = 1 / state.zoom;
    
    const handles: TransformHandle[] = [
      { type: 'nw', x: rect.x, y: rect.y, cursor: 'nwse-resize' },
      { type: 'n', x: rect.x + rect.width / 2, y: rect.y, cursor: 'ns-resize' },
      { type: 'ne', x: rect.x + rect.width, y: rect.y, cursor: 'nesw-resize' },
      { type: 'e', x: rect.x + rect.width, y: rect.y + rect.height / 2, cursor: 'ew-resize' },
      { type: 'se', x: rect.x + rect.width, y: rect.y + rect.height, cursor: 'nwse-resize' },
      { type: 's', x: rect.x + rect.width / 2, y: rect.y + rect.height, cursor: 'ns-resize' },
      { type: 'sw', x: rect.x, y: rect.y + rect.height, cursor: 'nesw-resize' },
      { type: 'w', x: rect.x, y: rect.y + rect.height / 2, cursor: 'ew-resize' },
    ];
    
    handles.forEach(handle => {
      ctx.fillRect(handle.x - handleSize / 2, handle.y - handleSize / 2, handleSize, handleSize);
      ctx.strokeRect(handle.x - handleSize / 2, handle.y - handleSize / 2, handleSize, handleSize);
    });
    
    const rotateHandle: TransformHandle = {
      type: 'rotate',
      x: rect.x + rect.width / 2,
      y: rect.y - ROTATE_HANDLE_OFFSET / state.zoom,
      cursor: 'crosshair',
    };
    
    ctx.beginPath();
    ctx.moveTo(rect.x + rect.width / 2, rect.y);
    ctx.lineTo(rotateHandle.x, rotateHandle.y);
    ctx.stroke();
    
    ctx.beginPath();
    ctx.arc(rotateHandle.x, rotateHandle.y, handleSize / 2, 0, Math.PI * 2);
    ctx.fillStyle = '#165dff';
    ctx.fill();
  };

  watch(() => editorStore.zoom, (newZoom) => {
    state.zoom = newZoom;
    render();
  });

  return {
    state,
    initCanvas,
    destroyCanvas,
    zoomIn,
    zoomOut,
    resetZoom,
    zoomAt,
    pan,
    render,
    screenToCanvas,
    canvasToScreen,
  };
}
