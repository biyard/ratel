import { useCallback, useEffect, useRef } from 'react';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { TablePlugin } from '@lexical/react/LexicalTablePlugin';
import {
  type LexicalEditor,
  type EditorState,
  $getRoot,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { logger } from '@/lib/logger';
import { EditorRefPlugin } from './editor-ref-plugin';

export function Editor({
  disabled,
  placeholder,
  content,
  updateContent,
  label,
}: {
  disabled: boolean;
  placeholder: string;
  content: string | null;
  updateContent: (content: string) => void;
  label?: string;
}) {
  const editorRef = useRef<LexicalEditor | null>(null);
  const isLoadingContent = useRef(false);

  const handleLexicalChange = (
    editorState: EditorState,
    editor: LexicalEditor,
  ) => {
    editorRef.current = editor;
    editorState.read(() => {
      const html = $generateHtmlFromNodes(editor, null);
      if (html !== content) {
        updateContent(html);
      }
    });
  };

  const createEditorStateFromHTML = useCallback(
    (editor: LexicalEditor, htmlString: string) => {
      if (!htmlString) {
        const root = $getRoot();
        root.clear();
        root.append($createParagraphNode());
        return;
      }
      try {
        const parser = new DOMParser();
        const dom = parser.parseFromString(htmlString, 'text/html');
        const nodes = $generateNodesFromDOM(editor, dom);
        const root = $getRoot();
        root.clear();
        root.append(...nodes);
      } catch (error) {
        logger.error('Error parsing HTML:', error);
      }
    },
    [],
  );

  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;

    const currentHtml = editor
      .getEditorState()
      .read(() => $generateHtmlFromNodes(editor, null));
    if (!content || content !== currentHtml) {
      isLoadingContent.current = true;

      editor.update(
        () => {
          createEditorStateFromHTML(editor, content ?? '');
        },
        {
          onUpdate: () => {
            setTimeout(() => {
              isLoadingContent.current = false;
            }, 0);
          },
        },
      );
    }
  }, [editorRef, content, createEditorStateFromHTML]);

  return (
    <>
      <RichTextPlugin
        contentEditable={
          <ContentEditable
            aria-label={label || 'editor'}
            disabled={disabled}
            className="outline-none resize-none w-full min-h-[150px] px-5 py-3"
          />
        }
        placeholder={
          <div className="absolute top-3 left-5 text-neutral-500 pointer-events-none select-none">
            {placeholder}
          </div>
        }
        ErrorBoundary={LexicalErrorBoundary}
      />
      <OnChangePlugin onChange={handleLexicalChange} />
      <HistoryPlugin />
      <TablePlugin />
      <EditorRefPlugin
        setEditorRef={(editor) => (editorRef.current = editor)}
      />
    </>
  );
}
