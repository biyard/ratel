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
  getCurrentTheme: () => 'light' | 'dark';
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    themeAwareHighlight: {
      setThemeAwareHighlight: (attributes?: { color: string }) => ReturnType;
      toggleThemeAwareHighlight: (attributes?: { color: string }) => ReturnType;
      unsetThemeAwareHighlight: () => ReturnType;
      updateThemeHighlights: () => ReturnType;
    };
  }
}

export const ThemeAwareHighlight = Extension.create<ThemeAwareHighlightOptions>(
  {
    name: 'themeAwareHighlight',

    addOptions() {
      return {
        multicolor: true,
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

                const currentTheme = this.options.getCurrentTheme();
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
        updateThemeHighlights:
          () =>
          ({ tr, state, dispatch }) => {
            if (!dispatch) return false;

            const { doc } = state;
            let modified = false;

            doc.descendants((node, pos) => {
              if (node.marks && node.marks.length > 0) {
                node.marks.forEach((mark) => {
                  if (mark.type.name === 'textStyle' && mark.attrs.highlight) {
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
  },
);
