'use client';

import { useRef, useState, useEffect } from 'react';
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';
import TextAlign from '@tiptap/extension-text-align';
import BulletList from '@tiptap/extension-bullet-list';
import OrderedList from '@tiptap/extension-ordered-list';
import ListItem from '@tiptap/extension-list-item';
import {
  Bold1,
  Italic1,
  Underline1,
  StrikeThrough1,
  Image1,
  Bullet1,
  Ordered1,
} from '../icons';

export default function TextEditor({
  isImage = false,
  content,
  onChange,
  onKeyDown,
}: {
  isImage?: boolean;
  content: string;
  onChange?: (newContent: string) => void;
  onKeyDown?: (e: React.KeyboardEvent) => void;
}) {
  const editor = useEditor({
    extensions: [
      StarterKit,
      BulletList,
      OrderedList,
      ListItem,
      Underline,
      Highlight,
      TextStyle,
      Color,
      Link,
      Image,
      TextAlign.configure({
        types: ['heading', 'paragraph'],
      }),
    ],
    content,
    onUpdate: ({ editor }) => {
      onChange?.(editor.getHTML());
    },
  });

  const [dropdownOpen, setDropdownOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(e.target as Node)
      ) {
        setDropdownOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const buttonClass = (active: boolean) =>
    `p-1 rounded-full transition ${active ? 'bg-neutral-400' : 'bg-transparent'}`;

  const headingLabel = () => {
    if (!editor) return 'Normal';
    if (editor.isActive('heading', { level: 1 })) return 'Heading 1';
    if (editor.isActive('heading', { level: 2 })) return 'Heading 2';
    if (editor.isActive('heading', { level: 3 })) return 'Heading 3';
    return 'Normal';
  };

  const applyHeading = (value: string) => {
    const chain = editor?.chain().focus();
    if (!chain) return;

    if (value === 'paragraph') chain.setParagraph().run();
    else chain.setHeading({ level: parseInt(value) as 1 | 2 | 3 }).run();

    setDropdownOpen(false);
  };

  if (!editor) return null;

  return (
    <div className="gap-5 py-5 px-4 w-full text-white rounded-lg border bg-card-bg-secondary border-card-border-secondary">
      <div className="flex flex-wrap gap-2 items-center mb-4 text-sm">
        <div className="relative" ref={dropdownRef}>
          <button
            onClick={() => setDropdownOpen((prev) => !prev)}
            className="py-1 px-3 text-sm rounded bg-card-bg-secondary text-text-primary"
          >
            {headingLabel()}
          </button>
          {dropdownOpen && (
            <div className="absolute left-0 z-10 mt-2 w-40 rounded-md border shadow-lg bg-neutral-800 border-neutral-700 light:bg-white">
              <div className="py-1">
                <button
                  onClick={() => applyHeading('1')}
                  className="block py-2 px-4 w-full text-xl text-left light:bg-white text-text-primary hover:bg-neutral-600"
                >
                  Heading 1
                </button>
                <button
                  onClick={() => applyHeading('2')}
                  className="block py-2 px-4 w-full text-lg text-left light:bg-white text-text-primary hover:bg-neutral-600"
                >
                  Heading 2
                </button>
                <button
                  onClick={() => applyHeading('3')}
                  className="block py-2 px-4 w-full text-base text-left light:bg-white text-text-primary hover:bg-neutral-600"
                >
                  Heading 3
                </button>
                <button
                  onClick={() => applyHeading('paragraph')}
                  className="block py-2 px-4 w-full text-sm text-left light:bg-white text-text-primary hover:bg-neutral-600"
                >
                  Normal
                </button>
              </div>
            </div>
          )}
        </div>

        <button
          onClick={() => editor.chain().focus().toggleBold().run()}
          className={buttonClass(editor.isActive('bold'))}
        >
          <Bold1 />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleItalic().run()}
          className={buttonClass(editor.isActive('italic'))}
        >
          <Italic1 />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleUnderline().run()}
          className={buttonClass(editor.isActive('underline'))}
        >
          <Underline1 />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleStrike().run()}
          className={buttonClass(editor.isActive('strike'))}
        >
          <StrikeThrough1 />
        </button>

        <button
          onClick={() => {
            editor.chain().focus().toggleBulletList().run();
          }}
          className={buttonClass(editor.isActive('bulletList'))}
        >
          <Bullet1 />
        </button>
        <button
          onClick={() => {
            editor.chain().focus().toggleOrderedList().run();
          }}
          className={buttonClass(editor.isActive('orderedList'))}
        >
          <Ordered1 />
        </button>

        {isImage ?? (
          <button onClick={() => {}} className={buttonClass(false)}>
            <Image1 />
          </button>
        )}
      </div>

      <EditorContent
        editor={editor}
        onKeyDown={onKeyDown}
        className="tiptap prose prose-invert h-[300px] overflow-y-auto bg-neutral-800 light:bg-card-bg light:border-neutral-300 text-text-primary border border-neutral-700 px-3 py-4 rounded-lg
    focus:outline-none focus:ring-0 focus-visible:ring-0 focus:border-transparent focus-visible:border-transparent
    list-disc list-inside
    [&_ul]:list-disc [&_ol]:list-decimal [&_li]:ml-4
    [&_h1]:text-2xl [&_h2]:text-xl [&_h3]:text-lg"
      />
      <div>Shift + Enter for new line</div>
      <style>{`
        .ProseMirror:focus {
          outline: none;
        }
      `}</style>
    </div>
  );
}
