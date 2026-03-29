<template>
  <div
    class="draggable"
    :class="{ dragging: isDragging }"
    @mousedown="handleMouseDown"
    @touchstart="handleTouchStart"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const props = withDefaults(defineProps<{
  initialX?: number;
  initialY?: number;
  bounds?: string | HTMLElement;
  disabled?: boolean;
}>(), {
  initialX: 0,
  initialY: 0,
  disabled: false,
});

const emit = defineEmits<{
  (e: 'dragstart', event: MouseEvent | TouchEvent): void;
  (e: 'drag', x: number, y: number, event: MouseEvent | TouchEvent): void;
  (e: 'dragend', x: number, y: number, event: MouseEvent | TouchEvent): void;
}>();

const isDragging = ref(false);
const position = ref({ x: props.initialX, y: props.initialY });
const offset = ref({ x: 0, y: 0 });

function handleMouseDown(event: MouseEvent) {
  if (props.disabled) return;
  
  event.preventDefault();
  isDragging.value = true;
  
  offset.value = {
    x: event.clientX - position.value.x,
    y: event.clientY - position.value.y,
  };
  
  emit('dragstart', event);
  
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);
}

function handleMouseMove(event: MouseEvent) {
  if (!isDragging.value) return;
  
  let newX = event.clientX - offset.value.x;
  let newY = event.clientY - offset.value.y;
  
  const bounds = getBounds();
  if (bounds) {
    newX = Math.max(bounds.left, Math.min(newX, bounds.right));
    newY = Math.max(bounds.top, Math.min(newY, bounds.bottom));
  }
  
  position.value = { x: newX, y: newY };
  emit('drag', newX, newY, event);
}

function handleMouseUp(event: MouseEvent) {
  if (!isDragging.value) return;
  
  isDragging.value = false;
  emit('dragend', position.value.x, position.value.y, event);
  
  document.removeEventListener('mousemove', handleMouseMove);
  document.removeEventListener('mouseup', handleMouseUp);
}

function handleTouchStart(event: TouchEvent) {
  if (props.disabled) return;
  
  const touch = event.touches[0];
  isDragging.value = true;
  
  offset.value = {
    x: touch.clientX - position.value.x,
    y: touch.clientY - position.value.y,
  };
  
  emit('dragstart', event);
  
  document.addEventListener('touchmove', handleTouchMove);
  document.addEventListener('touchend', handleTouchEnd);
}

function handleTouchMove(event: TouchEvent) {
  if (!isDragging.value) return;
  
  const touch = event.touches[0];
  let newX = touch.clientX - offset.value.x;
  let newY = touch.clientY - offset.value.y;
  
  const bounds = getBounds();
  if (bounds) {
    newX = Math.max(bounds.left, Math.min(newX, bounds.right));
    newY = Math.max(bounds.top, Math.min(newY, bounds.bottom));
  }
  
  position.value = { x: newX, y: newY };
  emit('drag', newX, newY, event);
}

function handleTouchEnd(event: TouchEvent) {
  if (!isDragging.value) return;
  
  isDragging.value = false;
  emit('dragend', position.value.x, position.value.y, event);
  
  document.removeEventListener('touchmove', handleTouchMove);
  document.removeEventListener('touchend', handleTouchEnd);
}

function getBounds(): { left: number; top: number; right: number; bottom: number } | null {
  if (!props.bounds) return null;
  
  let boundsElement: HTMLElement | null = null;
  
  if (typeof props.bounds === 'string') {
    boundsElement = document.querySelector(props.bounds);
  } else {
    boundsElement = props.bounds;
  }
  
  if (!boundsElement) return null;
  
  const rect = boundsElement.getBoundingClientRect();
  return {
    left: rect.left,
    top: rect.top,
    right: rect.right,
    bottom: rect.bottom,
  };
}

function setPosition(x: number, y: number) {
  position.value = { x, y };
}

defineExpose({
  setPosition,
  position,
  isDragging,
});
</script>

<style scoped>
.draggable {
  cursor: move;
  user-select: none;
}

.draggable.dragging {
  cursor: grabbing;
  opacity: 0.8;
}
</style>
