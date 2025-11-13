/**
 * Custom TipTap extension for theme-aware text highlighting
 * Automatically inverts highlight colors when switching between light and dark themes
 * to maintain readability
 */

import { Extension } from '@tiptap/core';
import '@tiptap/extension-text-style';

export interface ThemeAwareHighlightOptions {
  multicolor: boolean;
  types: string[];
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
              parseHTML: (element) =>
                element.getAttribute('data-highlight') ||
                element.style.backgroundColor?.replace(/['"]+/g, ''),
              renderHTML: (attributes) => {
                if (!attributes.highlight) {
                  return {};
                }

                return {
                  'data-highlight': attributes.highlight,
                  style: `background-color: var(--theme-highlight-color, ${attributes.highlight})`,
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
