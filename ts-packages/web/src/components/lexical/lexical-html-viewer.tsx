'use client';
import { useEffect } from 'react';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';

import { $generateNodesFromDOM } from '@lexical/html';
import { $getRoot, $insertNodes } from 'lexical';

import { ParagraphNode, TextNode } from 'lexical';
import { logger } from '@/lib/logger';
import { editorTheme } from './lexical-html-editor';

function HtmlContentLoader({ htmlString }: { htmlString: string }) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    if (!htmlString) {
      editor.update(() => {
        $getRoot().clear();
      });
      return;
    }

    editor.update(() => {
      const parser = new DOMParser();
      const dom = parser.parseFromString(htmlString, 'text/html');
      const nodes = $generateNodesFromDOM(editor, dom);
      const root = $getRoot();
      root.clear();
      root.selectEnd();
      $insertNodes(nodes);
    });
  }, [editor, htmlString]);

  return null;
}

export default function LexicalHtmlViewer({
  id = 'default',
  htmlString,
}: {
  id?: string;
  htmlString: string;
}) {
  const editorConfig = {
    namespace: `lexical-html-viewer-${id}`,
    editable: false,
    nodes: [ParagraphNode, TextNode],
    theme: editorTheme,
    onError(error: unknown) {
      logger.error('Lexical Viewer Error:', error);
    },
  };

  return (
    <LexicalComposer initialConfig={editorConfig}>
      <RichTextPlugin
        contentEditable={
          <ContentEditable className="lexical-content-editable outline-none cursor-default select-text whitespace-pre-wrap break-words" />
        }
        placeholder={
          <div className="absolute top-4 left-4 text-gray-400 select-none pointer-events-none">
            {'No Content Available'}
          </div>
        }
        ErrorBoundary={LexicalErrorBoundary}
      />
      <HtmlContentLoader htmlString={htmlString} />
    </LexicalComposer>
  );
}
