'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { $getRoot, EditorState, LexicalEditor } from 'lexical';
import React, { useEffect, useRef } from 'react';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import ToolbarPlugin from '@/components/toolbar/toolbar';

const editorTheme = {
  ltr: 'text-left',
  rtl: 'text-right',
  paragraph: 'relative mb-1',
  text: {
    bold: 'font-bold',
    italic: 'italic',
    underline: 'underline',
    strikethrough: 'line-through',
    underlineStrikethrough: 'underline line-through',
  },
  placeholder:
    'absolute top-0 left-0 text-neutral-500 pointer-events-none select-none inline-block',
};

export interface SpaceContentsProps {
  isEdit?: boolean;
  htmlContents: string;
  setContents?: (htmlContents: string) => void;
}

export default function SpaceContents({
  isEdit = false,
  htmlContents,
  setContents = () => {},
}: SpaceContentsProps) {
  const editorRef = useRef<LexicalEditor | null>(null);

  const html = (
    <BlackBox>
      <div
        className="rich-content"
        dangerouslySetInnerHTML={{ __html: htmlContents }}
      />
      <style jsx global>{`
        .rich-content h1 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content h2 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content h3 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content h4 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content h5 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content h6 {
          font-size: 15px;
          font-weight: 700;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
        .rich-content p {
          font-size: 15px;
          font-weight: 400;
          line-height: 24px;
          margin-bottom: 20px;
          color: #d4d4d4;
        }
      `}</style>
    </BlackBox>
  );

  const editorConfig = {
    namespace: 'CreatePostEditor',
    theme: editorTheme,
    onError(error: Error) {
      console.error(error);
    },
    editorState: (editor: LexicalEditor) => {
      const parser = new DOMParser();
      const dom = parser.parseFromString(htmlContents, 'text/html');
      editor.update(() => {
        const nodes = $generateNodesFromDOM(editor, dom);
        const root = $getRoot();
        root.clear();
        root.append(...nodes);
      });
    },
  };

  const handleLexicalChange = (
    editorState: EditorState,
    editor: LexicalEditor,
  ) => {
    editorRef.current = editor;
    editorState.read(() => {
      const html = $generateHtmlFromNodes(editor, null);
      if (html !== htmlContents) {
        setContents(html);
      }
    });
  };
  return isEdit ? (
    <LexicalComposer initialConfig={editorConfig}>
      <div className="px-4 py-2 min-h-[80px] relative text-neutral-300 text-[15px] leading-relaxed border border-neutral-500 rounded-[8px]">
        <RichTextPlugin
          contentEditable={
            <ContentEditable className="outline-none resize-none w-full min-h-[100px]" />
          }
          placeholder={
            <div className="absolute top-0 text-neutral-500 pointer-events-none select-none">
              Type here, Use Markdown, BB code, or HTML to format.
            </div>
          }
          ErrorBoundary={LexicalErrorBoundary}
        />
        <OnChangePlugin onChange={handleLexicalChange} />

        <HistoryPlugin />
        <EditorRefPlugin
          setEditorRef={(editor) => (editorRef.current = editor)}
        />

        <ToolbarPlugin enableImage={false} onImageUpload={() => {}} />
      </div>
    </LexicalComposer>
  ) : (
    html
  );
}

function EditorRefPlugin({
  setEditorRef,
}: {
  setEditorRef: (editor: LexicalEditor) => void;
}) {
  const [editor] = useLexicalComposerContext();
  useEffect(() => {
    setEditorRef(editor);
  }, [editor, setEditorRef]);
  return null;
}
