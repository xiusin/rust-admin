import { defineStore } from 'pinia';
import type { ColorScheme, FontScheme, LayoutRules, StyleParams } from '@/api/modules/ppt/generate';

interface StyleState {
  currentStyle: StyleParams | null;
  colorScheme: ColorScheme | null;
  fontScheme: FontScheme | null;
  layoutRules: LayoutRules | null;
  presetStyles: StyleParams[];
  recentStyles: StyleParams[];
  favoriteStyles: string[];
}

export const useStyleStore = defineStore('style', {
  state: (): StyleState => ({
    currentStyle: null,
    colorScheme: null,
    fontScheme: null,
    layoutRules: null,
    presetStyles: [],
    recentStyles: [],
    favoriteStyles: [],
  }),

  getters: {
    hasStyle: (state): boolean => state.currentStyle !== null,
    
    primaryColor: (state): string => {
      return state.colorScheme?.primary_color || '#0066FF';
    },
    
    secondaryColor: (state): string => {
      return state.colorScheme?.secondary_color || '#FF6B00';
    },
    
    titleFont: (state): string => {
      return state.fontScheme?.title_font?.family || 'Microsoft YaHei';
    },
    
    bodyFont: (state): string => {
      return state.fontScheme?.body_font?.family || 'Microsoft YaHei';
    },
  },

  actions: {
    setCurrentStyle(style: StyleParams) {
      this.currentStyle = style;
      this.colorScheme = style.color_scheme || null;
      this.fontScheme = style.font_scheme || null;
      this.layoutRules = style.layout_rules || null;
      
      this.addToRecent(style);
    },

    updateColorScheme(scheme: Partial<ColorScheme>) {
      if (this.colorScheme) {
        this.colorScheme = { ...this.colorScheme, ...scheme };
      } else {
        this.colorScheme = scheme as ColorScheme;
      }
    },

    updateFontScheme(scheme: Partial<FontScheme>) {
      if (this.fontScheme) {
        this.fontScheme = { ...this.fontScheme, ...scheme };
      } else {
        this.fontScheme = scheme as FontScheme;
      }
    },

    updateLayoutRules(rules: Partial<LayoutRules>) {
      if (this.layoutRules) {
        this.layoutRules = { ...this.layoutRules, ...rules };
      } else {
        this.layoutRules = rules as LayoutRules;
      }
    },

    setPrimaryColor(color: string) {
      if (this.colorScheme) {
        this.colorScheme.primary_color = color;
      }
    },

    setSecondaryColor(color: string) {
      if (this.colorScheme) {
        this.colorScheme.secondary_color = color;
      }
    },

    setTitleFont(font: string) {
      if (this.fontScheme?.title_font) {
        this.fontScheme.title_font.family = font;
      }
    },

    setBodyFont(font: string) {
      if (this.fontScheme?.body_font) {
        this.fontScheme.body_font.family = font;
      }
    },

    addToRecent(style: StyleParams) {
      const exists = this.recentStyles.findIndex(
        s => JSON.stringify(s) === JSON.stringify(style)
      );
      if (exists > -1) {
        this.recentStyles.splice(exists, 1);
      }
      this.recentStyles.unshift(style);
      if (this.recentStyles.length > 10) {
        this.recentStyles.pop();
      }
    },

    toggleFavorite(styleId: string) {
      const index = this.favoriteStyles.indexOf(styleId);
      if (index > -1) {
        this.favoriteStyles.splice(index, 1);
      } else {
        this.favoriteStyles.push(styleId);
      }
    },

    isFavorite(styleId: string): boolean {
      return this.favoriteStyles.includes(styleId);
    },

    reset() {
      this.currentStyle = null;
      this.colorScheme = null;
      this.fontScheme = null;
      this.layoutRules = null;
    },
  },

  persist: {
    key: 'ppt-style-store',
    storage: localStorage,
    paths: ['recentStyles', 'favoriteStyles'],
  },
});
