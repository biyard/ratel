/**
 * Custom TipTap extension for theme-aware text colors
 * Automatically adjusts colors dynamically for any color code when switching themes
 */

import { Extension } from '@tiptap/core';
import '@tiptap/extension-text-style';
import { getThemeAdjustedColor } from '../color-utils';

export interface ThemeAwareColorOptions {
  types: string[];
  getCurrentTheme: () => 'light' | 'dark';
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    themeAwareColor: {
      setThemeAwareColor: (color: string) => ReturnType;
      unsetThemeAwareColor: () => ReturnType;
      updateThemeColors: () => ReturnType;
    };
  }
}

export const ThemeAwareColor = Extension.create<ThemeAwareColorOptions>({
  name: 'themeAwareColor',

  addOptions() {
    return {
      types: ['textStyle'],
      getCurrentTheme: () => {
        if (typeof document === 'undefined') return 'dark';
        const theme = document.documentElement.getAttribute('data-theme');
        return (theme === 'light' ? 'light' : 'dark') as 'light' | 'dark';
      },
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

              const currentTheme = this.options.getCurrentTheme();
              const adjustedColor = getThemeAdjustedColor(
                attributes.color,
                currentTheme,
              );

              if (process.env.NODE_ENV === 'development') {
                console.log('[ThemeAwareColor]', {
                  original: attributes.color,
                  theme: currentTheme,
                  adjusted: adjustedColor,
                });
              }

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
      updateThemeColors:
        () =>
        ({ tr, state, dispatch }) => {
          if (!dispatch) return false;

          const { doc } = state;
          let modified = false;

          doc.descendants((node, pos) => {
            if (node.marks && node.marks.length > 0) {
              node.marks.forEach((mark) => {
                if (mark.type.name === 'textStyle' && mark.attrs.color) {
                  const from = pos;
                  const to = pos + node.nodeSize;
                  
                  tr.removeMark(from, to, mark.type);
                  tr.addMark(from, to, mark.type.create(mark.attrs));
                  modified = true;
                }
              });
            }
          });

          if (modified) {
            dispatch(tr);
          }

          return modified;
        },
    };
  },
});
