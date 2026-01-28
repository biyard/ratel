/**
 * Custom TipTap extension for theme-aware text highlighting
 * Automatically adjusts highlight colors dynamically for any color when switching themes
 */

import { Extension } from '@tiptap/core';
import '@tiptap/extension-text-style';
import { getThemeAdjustedHighlight } from '../color-utils';

export interface ThemeAwareHighlightOptions {
  multicolor: boolean;
  types: string[];
  getCurrentTheme?: () => 'light' | 'dark';
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    themeAwareHighlight: {
      /**
       * Set highlight with theme awareness
       */
      setThemeAwareHighlight: (attributes?: { color: string }) => ReturnType;
      /**
       * Toggle highlight
       */
      toggleThemeAwareHighlight: (attributes?: { color: string }) => ReturnType;
      /**
       * Unset highlight
       */
      unsetThemeAwareHighlight: () => ReturnType;
    };
  }
}

/**
 * ThemeAwareHighlight extension
 * Stores the original highlight color as data attribute and uses CSS to handle theme-based inversion
 */
export const ThemeAwareHighlight = Extension.create<ThemeAwareHighlightOptions>(
  {
    name: 'themeAwareHighlight',

    addOptions() {
      return {
        multicolor: true,
        types: ['textStyle'],
      };
    },

    addGlobalAttributes() {
      return [
        {
          types: this.options.types,
          attributes: {
            highlight: {
              default: null,
              parseHTML: (element) => {
                const dataHighlight = element.getAttribute('data-highlight');
                if (dataHighlight) return dataHighlight;

                const styleColor = element.style.backgroundColor?.replace(/['"]+/g, '');
                return styleColor || null;
              },
              renderHTML: (attributes) => {
                if (!attributes.highlight) {
                  return {};
                }

                const currentTheme = this.options.getCurrentTheme?.() ||
                  (typeof document !== 'undefined' && document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark');
                const adjustedColor = getThemeAdjustedHighlight(
                  attributes.highlight,
                  currentTheme,
                );

                return {
                  'data-highlight': attributes.highlight,
                  style: `background-color: ${adjustedColor}`,
                };
              },
            },
          },
        },
      ];
    },

    addCommands() {
      return {
        setThemeAwareHighlight:
          (attributes) =>
            ({ chain }) => {
              return chain()
                .setMark('textStyle', { highlight: attributes?.color })
                .run();
            },
        toggleThemeAwareHighlight:
          (attributes) =>
            ({ chain }) => {
              return chain()
                .toggleMark('textStyle', { highlight: attributes?.color })
                .run();
            },
        unsetThemeAwareHighlight:
          () =>
            ({ chain }) => {
              return chain()
                .setMark('textStyle', { highlight: null })
                .removeEmptyTextStyle()
                .run();
            },
      };
    },
  },
);
