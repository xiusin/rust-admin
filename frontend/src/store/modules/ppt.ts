import { defineStore } from "pinia";
import type { PPTProject } from "@/api/modules/ppt/project";
import type { CanvasElement, SlideElement } from "@/components/ppt/editor/types";

interface PPTSlide {
  id: number | string;
  title: string;
  content?: any;
  style?: any;
  elements?: CanvasElement[];
  background?: any;
  notes?: string;
  transition?: any;
}

interface PPTState {
  currentProject: PPTProject | null;
  projectList: PPTProject[];
  slides: PPTSlide[];
  currentSlideIndex: number;
  loading: boolean;
  generating: boolean;
}

export const usePPTStore = defineStore("ppt", {
  state: (): PPTState => ({
    currentProject: null,
    projectList: [],
    slides: [],
    currentSlideIndex: -1,
    loading: false,
    generating: false
  }),

  getters: {
    currentSlide: (state): PPTSlide | null => {
      if (state.currentSlideIndex < 0 || state.currentSlideIndex >= state.slides.length) {
        return null;
      }
      return state.slides[state.currentSlideIndex];
    },

    slideCount: (state): number => {
      return state.slides.length;
    }
  },

  actions: {
    setCurrentProject(project: PPTProject | null) {
      this.currentProject = project;
    },

    setProjectList(list: PPTProject[]) {
      this.projectList = list;
    },

    setLoading(loading: boolean) {
      this.loading = loading;
    },

    setGenerating(generating: boolean) {
      this.generating = generating;
    },

    setSlides(slides: PPTSlide[]) {
      this.slides = slides;
    },

    addSlide(slide: PPTSlide) {
      this.slides.push(slide);
      this.currentSlideIndex = this.slides.length - 1;
    },

    updateSlide(index: number, slide: Partial<PPTSlide>) {
      if (index >= 0 && index < this.slides.length) {
        this.slides[index] = { ...this.slides[index], ...slide };
      }
    },

    deleteSlide(index: number) {
      if (index >= 0 && index < this.slides.length) {
        this.slides.splice(index, 1);
        if (this.currentSlideIndex >= this.slides.length) {
          this.currentSlideIndex = this.slides.length - 1;
        }
      }
    },

    setCurrentSlideIndex(index: number) {
      if (index >= -1 && index < this.slides.length) {
        this.currentSlideIndex = index;
      }
    },

    clearProject() {
      this.currentProject = null;
      this.slides = [];
      this.currentSlideIndex = -1;
    },

    reorderSlides(fromIndex: number, toIndex: number) {
      if (fromIndex < 0 || fromIndex >= this.slides.length) return;
      if (toIndex < 0 || toIndex >= this.slides.length) return;

      const [slide] = this.slides.splice(fromIndex, 1);
      this.slides.splice(toIndex, 0, slide);

      if (this.currentSlideIndex === fromIndex) {
        this.currentSlideIndex = toIndex;
      } else if (this.currentSlideIndex > fromIndex && this.currentSlideIndex <= toIndex) {
        this.currentSlideIndex--;
      } else if (this.currentSlideIndex < fromIndex && this.currentSlideIndex >= toIndex) {
        this.currentSlideIndex++;
      }
    },

    addElement(element: CanvasElement) {
      const slide = this.currentSlide;
      if (!slide) return;
      
      if (!slide.elements) {
        slide.elements = [];
      }
      slide.elements.push(element);
    },

    updateElement(elementId: string, updates: Partial<CanvasElement>) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element) {
        Object.assign(element, updates);
      }
    },

    updateElementPosition(elementId: string, dx: number, dy: number) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element && !element.locked) {
        element.x += dx;
        element.y += dy;
      }
    },

    updateElementSize(elementId: string, width: number, height: number) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element && !element.locked) {
        element.width = width;
        element.height = height;
      }
    },

    updateElementRotation(elementId: string, rotation: number) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element && !element.locked) {
        element.rotation = rotation;
      }
    },

    deleteElement(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const index = slide.elements.findIndex(el => el.id === elementId);
      if (index > -1) {
        slide.elements.splice(index, 1);
      }
    },

    copyElement(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return null;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element) {
        return JSON.parse(JSON.stringify(element));
      }
      return null;
    },

    pasteElement(element: CanvasElement) {
      const slide = this.currentSlide;
      if (!slide) return;
      
      if (!slide.elements) {
        slide.elements = [];
      }
      
      const newElement: CanvasElement = {
        ...element,
        id: `element-${Date.now()}`,
        x: element.x + 20,
        y: element.y + 20,
      };
      
      slide.elements.push(newElement);
      return newElement;
    },

    reorderElements(fromIndex: number, toIndex: number) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      if (fromIndex < 0 || fromIndex >= slide.elements.length) return;
      if (toIndex < 0 || toIndex >= slide.elements.length) return;
      
      const [element] = slide.elements.splice(fromIndex, 1);
      slide.elements.splice(toIndex, 0, element);
    },

    bringElementToFront(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const index = slide.elements.findIndex(el => el.id === elementId);
      if (index > -1 && index < slide.elements.length - 1) {
        const [element] = slide.elements.splice(index, 1);
        slide.elements.push(element);
      }
    },

    sendElementToBack(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const index = slide.elements.findIndex(el => el.id === elementId);
      if (index > 0) {
        const [element] = slide.elements.splice(index, 1);
        slide.elements.unshift(element);
      }
    },

    toggleElementVisibility(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element) {
        element.visible = !element.visible;
      }
    },

    toggleElementLock(elementId: string) {
      const slide = this.currentSlide;
      if (!slide || !slide.elements) return;
      
      const element = slide.elements.find(el => el.id === elementId);
      if (element) {
        element.locked = !element.locked;
      }
    }
  },

  persist: {
    key: "ppt-store",
    paths: ["currentProject"]
  }
});

export type { PPTSlide, PPTState };
