/**
 * Custom TipTap extension for theme-aware text colors
 * Automatically inverts colors when switching between light and dark themes
 * to maintain readability
 */

import { Extension } from '@tiptap/core';
import '@tiptap/extension-text-style';

export interface ThemeAwareColorOptions {
  types: string[];
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    themeAwareColor: {
      /**
       * Set the text color with theme awareness
       */
      setThemeAwareColor: (color: string) => ReturnType;
      /**
       * Unset the text color
       */
      unsetThemeAwareColor: () => ReturnType;
    };
  }
}

/**
 * ThemeAwareColor extension
 * Stores the original color as data attribute and uses CSS to handle theme-based inversion
 */
export const ThemeAwareColor = Extension.create<ThemeAwareColorOptions>({
  name: 'themeAwareColor',

  addOptions() {
    return {
      types: ['textStyle'],
    };
  },

  addGlobalAttributes() {
    return [
      {
        types: this.options.types,
        attributes: {
          color: {
            default: null,
            parseHTML: (element) =>
              element.getAttribute('data-color') ||
              element.style.color?.replace(/['"]+/g, ''),
            renderHTML: (attributes) => {
              if (!attributes.color) {
                return {};
              }

              return {
                'data-color': attributes.color,
                style: `color: var(--theme-text-color, ${attributes.color})`,
              };
            },
          },
        },
      },
    ];
  },

  addCommands() {
    return {
      setThemeAwareColor:
        (color: string) =>
        ({ chain }) => {
          return chain().setMark('textStyle', { color }).run();
        },
      unsetThemeAwareColor:
        () =>
        ({ chain }) => {
          return chain()
            .setMark('textStyle', { color: null })
            .removeEmptyTextStyle()
            .run();
        },
    };
  },
});
