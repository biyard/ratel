import { Extension } from '@tiptap/core';
import { Mark } from 'prosemirror-model';
import { TextSelection } from 'prosemirror-state';

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
        ({ state, tr, dispatch }) => {
          const { from, to, empty } = state.selection;

          if (empty) return false;

          const transaction = tr || state.tr;
          const changes: {
            from: number;
            to: number;
            text: string;
            marks: readonly Mark[];
          }[] = [];

          // Collect all text nodes and their ranges within the selection
          state.doc.nodesBetween(from, to, (node, pos) => {
            if (!node.isText) return true;

            // Calculate the actual start and end positions within the node
            const nodeStart = pos;
            const nodeEnd = pos + node.nodeSize;
            const fromInNode = Math.max(from, nodeStart) - nodeStart;
            const toInNode = Math.min(to, nodeEnd) - nodeStart;

            // Skip if the selection doesn't intersect with this node's text
            if (fromInNode >= toInNode) return true;

            // Get the text content and its marks
            const text = node.text?.slice(fromInNode, toInNode) || '';
            if (!text) return true;

            // Toggle case for this text slice
            const isUpperCase = text === text.toUpperCase();
            const newText = isUpperCase
              ? text.toLowerCase()
              : text.toUpperCase();

            changes.push({
              from: nodeStart + fromInNode,
              to: nodeStart + toInNode,
              text: newText,
              marks: node.marks,
            });

            return true;
          });

          // Apply changes from end to start to maintain correct positions
          let hasChanges = false;
          for (let i = changes.length - 1; i >= 0; i--) {
            const { from, to, text, marks } = changes[i];
            if (text) {
              transaction.replaceWith(from, to, state.schema.text(text, marks));
              hasChanges = true;
            }
          }

          if (hasChanges && dispatch) {
            // Update selection to maintain the same range
            const delta = changes.reduce((acc, change) => {
              return acc + (change.text.length - (change.to - change.from));
            }, 0);

            const newTo = to + delta;
            const newSelection = TextSelection.create(
              transaction.doc,
              Math.min(from, newTo),
              Math.max(from, newTo),
            );
            transaction.setSelection(newSelection);

            dispatch(transaction);
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
