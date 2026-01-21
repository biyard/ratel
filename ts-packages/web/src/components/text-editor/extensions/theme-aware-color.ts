/**
 * Custom TipTap extension for theme-aware text colors
 * Automatically adjusts colors dynamically for any color code when switching themes
 */

import { Extension } from '@tiptap/core';
import '@tiptap/extension-text-style';
import { getThemeAdjustedColor } from '../color-utils';

export interface ThemeAwareColorOptions {
  types: string[];
  getCurrentTheme?: () => 'light' | 'dark';
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
            parseHTML: (element) => {
              const dataColor = element.getAttribute('data-color');
              if (dataColor) return dataColor;

              const styleColor = element.style.color?.replace(/['"]+/g, '');
              return styleColor || null;
            },
            renderHTML: (attributes) => {
              if (!attributes.color) {
                return {};
              }

              const currentTheme = this.options.getCurrentTheme?.() ||
                (typeof document !== 'undefined' && document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark');
              const adjustedColor = getThemeAdjustedColor(
                attributes.color,
                currentTheme,
              );

              return {
                'data-color': attributes.color,
                style: `color: ${adjustedColor}`,
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
