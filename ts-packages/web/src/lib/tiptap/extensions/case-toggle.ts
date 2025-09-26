import { Extension } from '@tiptap/core';

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    caseToggle: {
      toggleCase: () => ReturnType;
    };
  }
}

export const CaseToggle = Extension.create({
  name: 'caseToggle',

  addCommands() {
    return {
      toggleCase:
        () =>
        ({ commands, state, tr }) => {
          const { from, to, empty } = state.selection;

          if (empty) return false;

          const text = state.doc.textBetween(from, to, ' ');
          let newText = '';
          const isUpperCase = text === text.toUpperCase();

          newText = isUpperCase ? text.toLowerCase() : text.toUpperCase();

          // text with the new cased text
          tr.replaceWith(from, to, state.schema.text(newText));

          // transaction application
          if (tr.docChanged) {
            commands.setTextSelection({ from, to: from + newText.length });
            return true;
          }

          return false;
        },
    };
  },

  addProseMirrorPlugins() {
    return [];
  },
});
