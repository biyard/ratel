'use client';

import {
  useState,
  useCallback,
  useEffect,
  useRef,
  useImperativeHandle,
  forwardRef,
} from 'react';
import {
  ImagePlus,
  Bold,
  Italic,
  Underline,
  Strikethrough,
  X,
} from 'lucide-react';

import { cn } from '@/lib/utils';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import {
  $getSelection,
  $isRangeSelection,
  FORMAT_TEXT_COMMAND,
  LexicalEditor,
  EditorState,
  $getRoot,
  TextFormatType,
  COMMAND_PRIORITY_LOW,
  SELECTION_CHANGE_COMMAND,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import FileUploader from '@/components/file-uploader';
import { logger } from '@/lib/logger';

import Image from 'next/image';

export const editorTheme = {
  ltr: 'text-left',
  rtl: 'text-right',
  text: {
    bold: 'font-bold',
    italic: 'italic',
    underline: 'underline',
    strikethrough: 'line-through',
    underlineStrikethrough: 'underline line-through',
  },
};

export const createEditorStateFromHTML = (
  editor: LexicalEditor,
  htmlString?: string | null,
) => {
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
};

function ToolbarPlugin({
  onImageUpload,
}: {
  onImageUpload: (url: string) => void;
}) {
  const [editor] = useLexicalComposerContext();
  const [activeEditor, setActiveEditor] = useState(editor);
  const [isBold, setIsBold] = useState(false);
  const [isItalic, setIsItalic] = useState(false);
  const [isUnderline, setIsUnderline] = useState(false);
  const [isStrikethrough, setIsStrikethrough] = useState(false);

  const updateToolbar = useCallback(() => {
    const selection = $getSelection();
    if ($isRangeSelection(selection)) {
      setIsBold(selection.hasFormat('bold'));
      setIsItalic(selection.hasFormat('italic'));
      setIsUnderline(selection.hasFormat('underline'));
      setIsStrikethrough(selection.hasFormat('strikethrough'));
    }
  }, []);

  useEffect(() => {
    return editor.registerCommand(
      SELECTION_CHANGE_COMMAND,
      (_payload, newEditor) => {
        updateToolbar();
        setActiveEditor(newEditor);
        return false;
      },
      COMMAND_PRIORITY_LOW,
    );
  }, [editor, updateToolbar]);

  useEffect(() => {
    return activeEditor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        updateToolbar();
      });
    });
  }, [activeEditor, updateToolbar]);

  const formatText = (format: TextFormatType) => {
    activeEditor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  };

  return (
    <div className="flex shrink-0 items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700">
      <button
        onClick={() => formatText('bold')}
        className={cn(isBold && 'bg-neutral-600 text-white')}
        aria-label="Format text as bold"
      >
        <Bold />
      </button>
      <button
        onClick={() => formatText('italic')}
        className={cn(isItalic && 'bg-neutral-600 text-white')}
        aria-label="Format text as italics"
      >
        <Italic />
      </button>
      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 text-white')}
        aria-label="Format text to underlined"
      >
        <Underline />
      </button>
      <button
        onClick={() => formatText('strikethrough')}
        className={cn(isStrikethrough && 'bg-neutral-600 text-white')}
        aria-label="Format text with a strikethrough"
      >
        <Strikethrough />
      </button>
      <FileUploader onUploadSuccess={onImageUpload}>
        <ImagePlus />
      </FileUploader>
    </div>
  );
}

export function EditorRefPlugin({
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
export interface LexicalHtmlEditorRef {
  getContent: () => string;
  clear: () => void;
}

export const LexicalHtmlEditor = forwardRef<
  LexicalHtmlEditorRef,
  {
    id?: string;
    onChange?: (content: string) => void;
    initialContent?: string;
    placeholder?: string;
    className?: string;
  }
>(function LexicalHtmlEditor(
  {
    id = 'default',
    onChange,
    initialContent,
    placeholder = 'Type here, Use Markdown, BB code, or HTML to format.',
    className = '',
  },
  ref,
) {
  const editorConfig = {
    namespace: `lexical-html-editor-${id}`,
    theme: editorTheme,
    onError(error: Error) {
      console.error(error);
    },
  };

  const [image, setImage] = useState<string | null>(null);
  const editorRef = useRef<LexicalEditor | null>(null);

  useImperativeHandle(
    ref,
    () => ({
      getContent: () => {
        const editor = editorRef.current;
        if (editor) {
          let html = '';
          editor.getEditorState().read(() => {
            html = $generateHtmlFromNodes(editor, null);
          });
          return html;
        }
        return '';
      },
      clear: () => {
        const editor = editorRef.current;
        if (editor) {
          editor.update(() => {
            const root = $getRoot();
            root.clear();
            root.append($createParagraphNode());
          });
        }
      },
    }),
    [],
  );

  const handleLexicalChange = (
    editorState: EditorState,
    editor: LexicalEditor,
  ) => {
    editorRef.current = editor;
    editorState.read(() => {
      const html = $generateHtmlFromNodes(editor, null);
      if (html !== initialContent && onChange) {
        onChange(html);
      }
    });
  };

  const removeImage = () => {
    setImage(null);
  };

  useEffect(() => {
    const editor = editorRef.current;
    if (editor && initialContent !== undefined) {
      editor.update(() => {
        createEditorStateFromHTML(editor, initialContent);
      });
    }
  }, [initialContent]);

  return (
    <div className={cn('flex flex-col min-h-50 p-4', className)}>
      <LexicalComposer initialConfig={editorConfig}>
        {/* Lexical Content Area */}
        <div className="relative flex flex-1 text-neutral-300 ">
          <RichTextPlugin
            contentEditable={
              <ContentEditable className="outline-none resize-none w-full flex-1" />
            }
            placeholder={
              <div className="absolute text-neutral-500 pointer-events-none select-none">
                {placeholder}
              </div>
            }
            ErrorBoundary={LexicalErrorBoundary}
          />
          <OnChangePlugin onChange={handleLexicalChange} />

          <HistoryPlugin />
          <EditorRefPlugin
            setEditorRef={(editor) => (editorRef.current = editor)}
          />
        </div>

        {image && (
          <div className="px-4 pt-2">
            <div className="flex flex-wrap gap-2">
              <div className="relative size-16">
                <Image
                  width={64}
                  height={64}
                  src={image}
                  alt={`Uploaded image`}
                  className="object-cover rounded-lg border border-neutral-600"
                />
                <button
                  onClick={removeImage}
                  className="absolute -top-1.5 -right-1.5 w-5 h-5 bg-red-600 rounded-full flex items-center justify-center text-white text-xs hover:bg-red-700 border-2 border-neutral-900"
                  aria-label={`Remove uploaded image`}
                >
                  <X size={12} />
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Bottom toolbar */}
        <div className="flex items-center justify-between text-neutral-400">
          <ToolbarPlugin onImageUpload={(url) => setImage(url)} />
        </div>
      </LexicalComposer>
    </div>
  );
});
