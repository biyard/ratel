import { useEditor, EditorContent } from '@tiptap/react';
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
import Video from './extensions/video';
import { ThemeAwareColor } from './extensions/theme-aware-color';
import { ThemeAwareHighlight } from './extensions/theme-aware-highlight';
import { forwardRef, useEffect, useImperativeHandle, useRef } from 'react';
import { Editor } from '@tiptap/core';
import { cn } from '@/lib/utils';
import { TiptapEditorProps, DEFAULT_ENABLED_FEATURES } from './types';
import { TiptapToolbar } from './tiptap-toolbar';
import { showErrorToast } from '@/lib/toast';
import './theme-aware-colors.css';

export const TiptapEditor = forwardRef<Editor | null, TiptapEditorProps>(
  (
    {
      content = '',
      onUpdate,
      editable = true,
      placeholder = 'Type your script',
      showToolbar = true,
      toolbarPosition = 'top',
      enabledFeatures = DEFAULT_ENABLED_FEATURES,
      className,
      toolbarClassName,
      editorClassName,
      minHeight = '200px',
      maxHeight,
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
            width: 640,
            height: 360,
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
        ],
        content,
        editable,
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
      [editable, uploadAsset, uploadVideo, maxImageSizeMB, maxVideoSizeMB],
    ) as Editor | null;

    useImperativeHandle<Editor | null, Editor | null>(ref, () => editor, [
      editor,
    ]);

    useEffect(() => {
      if (editor && !editor.isDestroyed && content !== editor.getHTML()) {
        editor.commands.setContent(content);
      }
    }, [content, editor]);

    useEffect(() => {
      return () => {
        if (editor) editor.destroy();
      };
    }, [editor]);

    if (!editor) return <></>;

    return (
      <div
        className={cn(
          'flex flex-col w-full rounded-lg border transition-colors p-1',
          'bg-card text-foreground border-transparent',
          'focus-within:border-primary',
          className,
        )}
        data-pw={dataPw}
      >
        {showToolbar && toolbarPosition === 'top' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            className={toolbarClassName}
            openVideoPicker={() => videoInputRef.current?.click()}
            onImageUpload={onImageUpload}
            onUploadPDF={onUploadPDF}
          />
        )}

        <div
          className={cn('flex-1 overflow-y-auto', 'px-5 py-3', editorClassName)}
          style={{
            minHeight: showToolbar ? `calc(${minHeight} - 48px)` : minHeight,
            maxHeight,
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
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:content-[attr(data-placeholder)]',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:text-foreground-muted',
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
            )}
            data-placeholder={placeholder}
          />
          <input
            ref={videoInputRef}
            type="file"
            accept="video/*"
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
