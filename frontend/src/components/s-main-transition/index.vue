<template>
  <Transition
    :name="transitionName"
    :mode="transitionMode"
    :appear="appear"
    @before-enter="onBeforeEnter"
    @after-enter="onAfterEnter"
    @before-leave="onBeforeLeave"
    @after-leave="onAfterLeave"
  >
    <slot></slot>
  </Transition>
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useThemeConfig } from "@/store/modules/theme-config";

interface Props {
  mode?: "in-out" | "out-in" | "default";
  appear?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  mode: "out-in",
  appear: true
});

const emit = defineEmits<{
  beforeEnter: [el: Element];
  afterEnter: [el: Element];
  beforeLeave: [el: Element];
  afterLeave: [el: Element];
}>();

const themeStore = useThemeConfig();
const { transitionPage } = storeToRefs(themeStore);

const transitionName = computed(() => {
  const validTransitions = [
    "fadeInOut",
    "slideLeftRight",
    "slideUpDown",
    "zoomInOut",
    "rotateInOut",
    "cardInOut",
    "fadeOut",
    "flipInOut",
    "bounceInOut"
  ];
  return validTransitions.includes(transitionPage.value)
    ? transitionPage.value
    : "fadeInOut";
});

const transitionMode = computed(() =>
  props.mode === "default" ? undefined : props.mode
);

const onBeforeEnter = (el: Element) => {
  emit("beforeEnter", el);
};

const onAfterEnter = (el: Element) => {
  emit("afterEnter", el);
};

const onBeforeLeave = (el: Element) => {
  emit("beforeLeave", el);
};

const onAfterLeave = (el: Element) => {
  emit("afterLeave", el);
};
</script>

<style lang="scss" scoped></style>
