export interface CanvasElement {
  id: string;
  type: 'text' | 'image' | 'shape' | 'chart' | 'table' | 'line';
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  opacity: number;
  locked: boolean;
  visible: boolean;
  zIndex: number;
  content: ElementContent;
  style: ElementStyle;
  animation?: ElementAnimation;
}

export interface ElementContent {
  text?: string;
  imageUrl?: string;
  shapeType?: 'rectangle' | 'circle' | 'triangle' | 'line' | 'arrow';
  chartData?: any;
  tableData?: any;
}

export interface ElementStyle {
  backgroundColor?: string;
  borderColor?: string;
  borderWidth?: number;
  borderRadius?: number;
  fontFamily?: string;
  fontSize?: number;
  fontWeight?: string;
  fontStyle?: string;
  textDecoration?: string;
  color?: string;
  textAlign?: 'left' | 'center' | 'right';
  verticalAlign?: 'top' | 'middle' | 'bottom';
  lineHeight?: number;
  letterSpacing?: number;
  filter?: string;
  shadow?: string;
}

export interface ElementAnimation {
  type: 'fade' | 'slide' | 'zoom' | 'bounce' | 'rotate';
  duration: number;
  delay: number;
  direction?: 'left' | 'right' | 'top' | 'bottom';
  easing?: string;
}

export interface SlideElement {
  id: string;
  title: string;
  elements: CanvasElement[];
  background: SlideBackground;
  notes?: string;
  duration?: number;
  transition?: SlideTransition;
}

export interface SlideBackground {
  type: 'color' | 'gradient' | 'image';
  color?: string;
  gradient?: {
    type: 'linear' | 'radial';
    colors: string[];
    angle?: number;
  };
  imageUrl?: string;
  opacity?: number;
}

export interface SlideTransition {
  type: 'fade' | 'slide' | 'zoom' | 'flip' | 'cube';
  duration: number;
  direction?: 'left' | 'right' | 'top' | 'bottom';
}

export interface EditorState {
  selectedElementIds: string[];
  currentTool: EditorTool;
  clipboard: CanvasElement | null;
  zoom: number;
  offset: { x: number; y: number };
  gridSize: number;
  showGrid: boolean;
  snapToGrid: boolean;
  snapToElement: boolean;
}

export type EditorTool = 'select' | 'text' | 'image' | 'shape' | 'line' | 'pan' | 'crop';

export interface HistoryState {
  slides: SlideElement[];
  currentSlideIndex: number;
  timestamp: number;
  action: string;
}

export interface Point {
  x: number;
  y: number;
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TransformHandle {
  type: 'nw' | 'n' | 'ne' | 'e' | 'se' | 's' | 'sw' | 'w' | 'rotate';
  x: number;
  y: number;
  cursor: string;
}
