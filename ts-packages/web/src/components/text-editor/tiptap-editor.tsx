import { useEditor, EditorContent, BubbleMenu } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Image from '@tiptap/extension-image';
import Link from '@tiptap/extension-link';
import Table from '@tiptap/extension-table';
import TableRow from '@tiptap/extension-table-row';
import TableCell from '@tiptap/extension-table-cell';
import TableHeader from '@tiptap/extension-table-header';
import Youtube from '@tiptap/extension-youtube';
import Placeholder from '@tiptap/extension-placeholder';
import Video from './extensions/video';
import { ThemeAwareColor } from './extensions/theme-aware-color';
import { ThemeAwareHighlight } from './extensions/theme-aware-highlight';
import {
  forwardRef,
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
} from 'react';
import { Editor } from '@tiptap/core';
import { cn } from '@/lib/utils';
import { TiptapEditorProps, DEFAULT_ENABLED_FEATURES } from './types';
import { TiptapToolbar } from './tiptap-toolbar';
import { showErrorToast } from '@/lib/toast';
import './theme-aware-colors.css';
import { AnalyzeLdaBlock } from './extensions/analyze/lda-block';
import { AnalyzeNetworkBlock } from './extensions/analyze/network-block';
import { AnalyzeTfidfBlock } from './extensions/analyze/tfidf-blck';

const FOLD_HEIGHT = 240;

export const TiptapEditor = forwardRef<Editor | null, TiptapEditorProps>(
  (
    {
      isMe = false,
      content = '',
      onUpdate,
      editable = true,
      placeholder = 'Type your script',
      variant = 'default',
      showToolbar = true,
      toolbarPosition = 'top',
      enabledFeatures = DEFAULT_ENABLED_FEATURES,
      showBubbleToolbar = false,
      bubbleEnabledFeatures,
      bubbleToolbarClassName,
      toolbarFooter,
      className,
      toolbarClassName,
      editorClassName,
      minHeight = '200px',
      isFoldable = false,
      onFocus,
      onBlur,
      uploadAsset,
      uploadVideo,
      maxImageSizeMB = 50,
      maxVideoSizeMB = 50,
      onImageUpload,
      onUploadPDF,
      'data-pw': dataPw,
    },
    ref,
  ) => {
    const videoInputRef = useRef<HTMLInputElement | null>(null);
    const [isFolded, setIsFolded] = useState<boolean>(false);
    const containerRef = useRef<HTMLDivElement | null>(null);
    const [showFoldToggle, setShowFoldToggle] = useState(false);
    const bubbleHostRef = useRef<HTMLDivElement | null>(null);
    const bubbleEnabledRef = useRef(showBubbleToolbar);
    const bubbleKeepAliveRef = useRef(false);
    const bubbleSelectionRef = useRef<Editor['state']['selection'] | null>(
      null,
    );

    useEffect(() => {
      bubbleEnabledRef.current = showBubbleToolbar;
    }, [showBubbleToolbar]);

    const shouldShowBubble = useCallback(
      ({
        editor,
        state,
      }: {
        editor: Editor;
        state: { selection: { empty: boolean } };
      }) => {
        if (!bubbleEnabledRef.current) return false;
        if (!editor.isEditable) return false;
        if (!editor.view?.dom?.isConnected) return false;
        if (bubbleKeepAliveRef.current) return true;
        return !state.selection.empty;
      },
      [],
    );

    const resolvedBubbleFeatures: typeof DEFAULT_ENABLED_FEATURES = {
      ...DEFAULT_ENABLED_FEATURES,
      image: false,
      table: false,
      pdf: false,
      ...bubbleEnabledFeatures,
    };

    const canFold = isFoldable;

    const insertImage = (ed: Editor, src: string, alt?: string) =>
      ed.chain().focus().setImage({ src, alt }).run();
    const insertVideo = (ed: Editor, src: string) =>
      ed.chain().focus().setVideo({ src, controls: true }).run();

    const handleFiles = async (ed: Editor, files: FileList | null) => {
      if (!files?.length) return true;
      const file = files[0];

      if (file.type.startsWith('image/')) {
        if (file.size > maxImageSizeMB * 1024 * 1024) return false;
        if (uploadAsset) {
          const { url } = await uploadAsset(file);
          insertImage(ed, url, file.name);
          return false;
        }
        return true;
      }

      if (file.type.startsWith('video/')) {
        if (file.size > maxVideoSizeMB * 1024 * 1024) return false;
        if (uploadVideo) {
          const { url } = await uploadVideo(file);
          insertVideo(ed, url);
          return false;
        }
        return false;
      }

      return true;
    };

    const editor = useEditor(
      {
        extensions: [
          StarterKit.configure({
            heading: { levels: [1, 2, 3] },
            bulletList: { HTMLAttributes: { class: 'list-disc pl-4' } },
            orderedList: { HTMLAttributes: { class: 'list-decimal pl-4' } },
          }),
          Placeholder.configure({ placeholder }),
          TextStyle,
          Color,
          Highlight.configure({ multicolor: true }),
          ThemeAwareColor,
          ThemeAwareHighlight.configure({ multicolor: true }),
          TextAlign.configure({
            types: ['heading', 'paragraph'],
            alignments: ['left', 'center', 'right'],
          }),
          Underline,
          Image.configure({
            inline: true,
            allowBase64: true,
            HTMLAttributes: {
              class: 'rounded-lg max-w-full h-auto my-4 mx-auto block',
            },
          }),
          Link.configure({
            autolink: true,
            linkOnPaste: true,
            openOnClick: false,
            HTMLAttributes: {
              rel: 'noopener noreferrer nofollow',
              class:
                'text-blue-500 underline underline-offset-2 decoration-blue-500 hover:text-blue-600',
            },
          }),
          Youtube.configure({
            controls: true,
            nocookie: true,
            allowFullscreen: true,
            HTMLAttributes: {
              class: 'w-full max-w-[640px] aspect-video mx-auto',
            },
          }),
          Video,
          Table.configure({
            resizable: true,
            HTMLAttributes: { class: 'border-collapse table-auto w-full my-4' },
          }),
          TableRow,
          TableHeader.configure({
            HTMLAttributes: { class: 'bg-muted font-semibold' },
          }),
          TableCell.configure({
            HTMLAttributes: { class: 'border border-border p-2 min-w-[100px]' },
          }),
          AnalyzeLdaBlock,
          AnalyzeNetworkBlock,
          AnalyzeTfidfBlock,
        ],
        content,
        editable: editable,
        onUpdate: ({ editor }) => onUpdate?.(editor.getHTML()),
        onFocus: () => onFocus?.(),
        onBlur: () => onBlur?.(),
        editorProps: {
          handleDrop(view, ev) {
            const dt = (ev as DragEvent).dataTransfer;
            if (!dt?.files?.length) return false;
            ev.preventDefault();
            ev.stopPropagation();
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            handleFiles((view as any).editor, dt.files);
            return true;
          },
          handlePaste(view, ev) {
            const cb = (ev as ClipboardEvent).clipboardData;
            const files = cb?.files ?? null;
            if (files?.length) {
              ev.preventDefault();
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              handleFiles((view as any).editor, files);
              return true;
            }
            return false;
          },
        },
      },
      [uploadAsset, uploadVideo, maxImageSizeMB, maxVideoSizeMB],
    ) as Editor | null;

    const restoreBubbleSelection = useCallback(() => {
      if (!editor?.view || !bubbleSelectionRef.current) return;
      try {
        editor.view.dispatch(
          editor.state.tr.setSelection(bubbleSelectionRef.current),
        );
        // eslint-disable-next-line unused-imports/no-unused-vars
      } catch (_) {
        // Ignore selection restore errors for non-text selections.
      }
    }, [editor]);

    const handleHeadingDropdownOpenChange = useCallback(
      (open: boolean) => {
        bubbleKeepAliveRef.current = open;
        if (open && editor?.state?.selection) {
          bubbleSelectionRef.current = editor.state.selection;
        }
      },
      [editor],
    );

    const handleHeadingDropdownTriggerPointerDown = useCallback(() => {
      if (editor?.state?.selection) {
        bubbleSelectionRef.current = editor.state.selection;
      }
      bubbleKeepAliveRef.current = true;
    }, [editor]);

    useEffect(() => {
      if (!canFold) {
        setShowFoldToggle(false);
        setIsFolded(false);
        return;
      }

      const id = window.requestAnimationFrame(() => {
        const el = containerRef.current;
        if (!el) return;

        const hasOverflow = el.scrollHeight >= FOLD_HEIGHT;
        setShowFoldToggle(hasOverflow);
        setIsFolded(hasOverflow);
      });

      return () => cancelAnimationFrame(id);
    }, [canFold, content, minHeight]);

    useImperativeHandle<Editor | null, Editor | null>(ref, () => editor, [
      editor,
    ]);

    useEffect(() => {
      if (editor && !editor.isDestroyed && content !== editor.getHTML()) {
        editor.commands.setContent(content);
      }
    }, [content, editor]);

    useEffect(() => {
      if (!editor || editor.isDestroyed) return;
      editor.setEditable(editable);
    }, [editor, editable]);

    useEffect(() => {
      return () => {
        if (editor) editor.destroy();
      };
    }, [editor]);

    if (!editor) return <></>;

    const variantClasses = {
      default: 'bg-card border-transparent',
      post: 'bg-[var(--color-post-input-bg)] border-[var(--color-post-input-border)]',
    };

    return (
      <div
        className={cn(
          'flex flex-col w-full rounded-lg border transition-colors p-1',
          'text-foreground',
          isMe && 'bg-neutral-700 light:bg-neutral-200',
          !isMe && variantClasses[variant],
          'focus-within:border-primary',
          className,
        )}
        data-pw={dataPw}
      >
        {showToolbar && toolbarPosition === 'top' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            variant={variant}
            mode="default"
            className={toolbarClassName}
            openVideoPicker={() => videoInputRef.current?.click()}
            onImageUpload={onImageUpload}
            onUploadPDF={onUploadPDF}
          />
        )}
        {showToolbar && toolbarPosition === 'top' && toolbarFooter && (
          <div className="px-2 py-2">{toolbarFooter}</div>
        )}

        {(showBubbleToolbar || bubbleEnabledFeatures) && editor && (
          <div ref={bubbleHostRef}>
            <BubbleMenu
              editor={editor}
              shouldShow={shouldShowBubble}
              tippyOptions={{
                duration: 120,
                placement: 'top',
                maxWidth: 'none',
                appendTo: () => bubbleHostRef.current ?? document.body,
              }}
            >
              <TiptapToolbar
                editor={editor}
                enabledFeatures={resolvedBubbleFeatures}
                variant={variant}
                mode="bubble"
                dropdownPortalContainer={
                  typeof document !== 'undefined' ? document.body : null
                }
                onHeadingDropdownOpenChange={handleHeadingDropdownOpenChange}
                onHeadingDropdownTriggerPointerDown={
                  handleHeadingDropdownTriggerPointerDown
                }
                onColorPickerOpenChange={handleHeadingDropdownOpenChange}
                onColorPickerTriggerPointerDown={
                  handleHeadingDropdownTriggerPointerDown
                }
                headingDropdownContentProps={{
                  side: 'bottom',
                  align: 'start',
                  sideOffset: 6,
                  alignOffset: 0,
                  avoidCollisions: false,
                  sticky: 'always',
                  onOpenAutoFocus: (event) => {
                    event.preventDefault();
                    restoreBubbleSelection();
                  },
                  onCloseAutoFocus: (event) => {
                    event.preventDefault();
                    restoreBubbleSelection();
                  },
                }}
                className={cn(
                  'rounded-md border bg-card shadow-lg px-2 py-1',
                  bubbleToolbarClassName,
                )}
              />
            </BubbleMenu>
          </div>
        )}

        <div
          className={cn('flex-1 px-5 py-3', 'overflow-y-auto', editorClassName)}
          style={{
            minHeight,
          }}
        >
          <div
            ref={containerRef}
            style={{
              maxHeight: canFold && isFolded ? `${FOLD_HEIGHT}px` : 'none',
              overflowY: canFold && isFolded ? 'hidden' : 'visible',
            }}
          >
            <EditorContent
              editor={editor}
              className={cn(
                'tiptap-editor',
                'text-foreground text-[15px]',
                '[&_.ProseMirror]:outline-none',
                '[&_.ProseMirror]:min-h-full',
                '[&_.ProseMirror]:h-full',
                '[&_.ProseMirror]:wrap-break-word',
                '[&_.ProseMirror]:max-w-full',
                '[&_.ProseMirror_p.is-editor-empty:first-child::before]:content-[attr(data-placeholder)]',
                '[&_.ProseMirror_p.is-editor-empty:first-child::before]:text-[var(--color-post-input-placeholder)]',
                '[&_.ProseMirror_p.is-editor-empty:first-child::before]:float-left',
                '[&_.ProseMirror_p.is-editor-empty:first-child::before]:pointer-events-none',
                '[&_.ProseMirror_p.is-editor-empty:first-child::before]:h-0',
                '[&_.ProseMirror_h1]:text-2xl [&_.ProseMirror_h1]:font-bold [&_.ProseMirror_h1]:mt-6 [&_.ProseMirror_h1]:mb-4',
                '[&_.ProseMirror_h2]:text-xl [&_.ProseMirror_h2]:font-bold [&_.ProseMirror_h2]:mt-5 [&_.ProseMirror_h2]:mb-3',
                '[&_.ProseMirror_h3]:text-lg [&_.ProseMirror_h3]:font-semibold [&_.ProseMirror_h3]:mt-4 [&_.ProseMirror_h3]:mb-2',
                '[&_.ProseMirror_ul]:list-disc [&_.ProseMirror_ul]:pl-6 [&_.ProseMirror_ul]:my-2',
                '[&_.ProseMirror_ol]:list-decimal [&_.ProseMirror_ol]:pl-6 [&_.ProseMirror_ol]:my-2',
                '[&_.ProseMirror_li]:my-1',
                '[&_.ProseMirror_p]:my-2',
                '[&_.ProseMirror_mark]:bg-yellow-200 [&_.ProseMirror_mark]:px-0.5',
                '[&_.ProseMirror_table]:border-collapse [&_.ProseMirror_table]:table-auto [&_.ProseMirror_table]:w-full [&_.ProseMirror_table]:my-4',
                '[&_.ProseMirror_td]:border [&_.ProseMirror_td]:border-border [&_.ProseMirror_td]:p-2 [&_.ProseMirror_td]:min-w-[100px] [&_.ProseMirror_td]:relative',
                '[&_.ProseMirror_th]:border [&_.ProseMirror_th]:border-border [&_.ProseMirror_th]:p-2 [&_.ProseMirror_th]:min-w-[100px] [&_.ProseMirror_th]:bg-muted [&_.ProseMirror_th]:font-semibold [&_.ProseMirror_th]:relative',
                '[&_.ProseMirror_.selectedCell]:bg-primary/20',
                '[&_.ProseMirror_.selectedCell]:border-primary',
                '[&_.ProseMirror_.selectedCell]:border-2',
                '[&_.ProseMirror_.selectedCell]:outline',
                '[&_.ProseMirror_.selectedCell]:outline-2',
                '[&_.ProseMirror_.youtube]:pt-[56.25%]',
                '[&_.ProseMirror_.selectedCell]:outline-primary/40',
                '[&_.ProseMirror_.selectedCell]:outline-offset-[-1px]',
                '[&_.ProseMirror_.column-resize-handle]:absolute [&_.ProseMirror_.column-resize-handle]:right-[-2px] [&_.ProseMirror_.column-resize-handle]:top-0 [&_.ProseMirror_.column-resize-handle]:bottom-0 [&_.ProseMirror_.column-resize-handle]:w-[4px] [&_.ProseMirror_.column-resize-handle]:bg-primary [&_.ProseMirror_.column-resize-handle]:pointer-events-none',
                '[&_.ProseMirror_iframe]:w-full [&_.ProseMirror_iframe]:max-w-full',
              )}
              data-testid="tiptap-editor-content"
              data-placeholder={placeholder}
            />
          </div>

          {canFold && showFoldToggle && (
            <div className="flex justify-center mt-2">
              <button
                type="button"
                className="text-xs hover:underline text-primary"
                onClick={() => setIsFolded((prev) => !prev)}
              >
                {isFolded ? 'More' : 'Close'}
              </button>
            </div>
          )}

          <input
            ref={videoInputRef}
            type="file"
            accept="video/*,.mkv"
            className="hidden"
            onChange={async (e) => {
              const file = e.target.files?.[0];
              if (!file) return;
              if (file.size > maxVideoSizeMB * 1024 * 1024) {
                showErrorToast(`File size exceeds ${maxVideoSizeMB}MB limit.`);
                e.currentTarget.value = '';
                return;
              }
              if (uploadVideo && editor) {
                const { url } = await uploadVideo(file);
                insertVideo(editor, url);
              }
              e.currentTarget.value = '';
            }}
          />
        </div>

        {showToolbar && toolbarPosition === 'bottom' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            variant={variant}
            mode="default"
            className={toolbarClassName}
            openVideoPicker={() => videoInputRef.current?.click()}
            onImageUpload={onImageUpload}
            onUploadPDF={onUploadPDF}
          />
        )}
      </div>
    );
  },
);

TiptapEditor.displayName = 'TiptapEditor';
