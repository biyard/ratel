'use client';

import { useState, useCallback, useEffect, useRef } from 'react';
import { X, Loader2 } from 'lucide-react';

import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import UserCircleIcon from '@/assets/icons/user-circle.svg';
import Certified from '@/assets/icons/certified.svg';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import {
  LexicalEditor,
  EditorState,
  $getRoot,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { logger } from '@/lib/logger';
import Image from 'next/image';
// import { showErrorToast } from '@/lib/toast';
import ToolbarPlugin from '@/components/toolbar/toolbar';
import { useTranslations } from 'next-intl';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { HexColorPicker } from 'react-colorful';
import { useUserInfo } from '../../_hooks/user';

import { PostType, Status, usePostEditorContext } from './provider';
import { ArtworkTrait, ArtworkTraitDisplayType } from '@/lib/api/models/feeds';

export {
  PostType,
  Status,
  usePostEditorContext,
  PostEditorProvider,
} from './provider';

export const editorTheme = {
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
};

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

function Editor({
  disabled,
  placeholder,
  content,
  updateContent,
}: {
  disabled: boolean;
  placeholder: string;
  content: string | null;
  updateContent: (content: string) => void;
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
            disabled={disabled}
            className="outline-none resize-none w-full min-h-[60px]"
          />
        }
        placeholder={
          <div className="absolute top-2 left-5 text-neutral-500 pointer-events-none select-none">
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
    </>
  );
}
const editorConfig = {
  namespace: 'CreatePostEditor',
  theme: editorTheme,
  onError(error: Error) {
    console.error(error);
  },
};

export function CreatePost() {
  const t = useTranslations('Home');

  const {
    expand,
    toggleExpand,
    content,
    updateContent,
    status,
    isSubmitDisabled,
    postType,
    updatePostType,
    title,
    updateTitle,
    image,
    updateImage,

    handleUpdate,
  } = usePostEditorContext();

  const { data: userInfo, isLoading } = useUserInfo();

  if (isLoading || !expand) {
    return null;
  }
  return (
    <div className={`flex flex-col w-full`}>
      <div className="w-full bg-card-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden">
        {/* Header */}
        <div className="flex items-center p-4 justify-between">
          <div className="flex items-center gap-3">
            <div className="flex flex-row items-center gap-2">
              {userInfo?.profile_url && userInfo?.profile_url !== '' ? (
                <Image
                  width={24}
                  height={24}
                  src={userInfo?.profile_url}
                  alt="Profile"
                  className="w-6 h-6 object-cover rounded-full"
                />
              ) : (
                <div className="w-6 h-6 rounded-full bg-profile-bg" />
              )}
              <div className="flex items-center gap-2">
                <span className="text-text-primary font-medium text-lg">
                  {userInfo?.nickname || 'Anonymous'}
                </span>
              </div>
              <Certified className="size-5" />
            </div>
            <SelectPostType postType={postType} setPostType={updatePostType} />
          </div>
          <div className={cn('cursor-pointer')} onClick={toggleExpand}>
            <DoubleArrowDown className="[&>path]:stroke-text-primary" />
          </div>
        </div>
        {postType === PostType.General ? (
          <LexicalComposer initialConfig={editorConfig}>
            {/* Title input */}
            <div className="px-4 pt-4">
              <input
                type="text"
                placeholder={t('write_title')}
                value={title}
                onChange={(e) => {
                  updateTitle(e.target.value);
                }}
                className="w-full bg-transparent text-text-primary text-xl font-semibold placeholder-neutral-500 outline-none border-none"
              />
            </div>

            {/* Lexical Content Area */}
            <div className="px-4 pt-2 min-h-[80px] relative text-text-primary text-[15px] leading-relaxed">
              <Editor
                disabled={false}
                content={content}
                updateContent={updateContent}
                placeholder={t('write_content')}
              />
            </div>

            {/* Image previews */}
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
                      onClick={() => updateImage(null)}
                      className="absolute -top-1.5 -right-1.5 w-5 h-5 bg-red-600 rounded-full flex items-center justify-center text-white text-xs hover:bg-red-700 border-2 border-component-bg"
                      aria-label={`Remove uploaded image`}
                    >
                      <X size={12} />
                    </button>
                  </div>
                </div>
              </div>
            )}

            {/* Bottom toolbar */}
            <div className="flex items-center justify-between p-4 text-neutral-400">
              <ToolbarPlugin onImageUpload={(url) => updateImage(url)} />

              <div className="flex items-center gap-4">
                {/* Status indicator */}
                {status === Status.Saving && (
                  <div className="flex items-center gap-2 text-sm text-neutral-400">
                    <Loader2 className="animate-spin" size={16} />
                    <span>Saving...</span>
                  </div>
                )}
                {/* {status === 'error' && (
                    <span className="text-sm text-red-500">Save failed</span>
                  )} */}

                <Button
                  variant="rounded_primary"
                  size="default"
                  onClick={async () => {
                    await handleUpdate();
                  }}
                  disabled={isSubmitDisabled || status !== Status.Idle}
                  className="gap-2"
                >
                  {status !== Status.Idle ? (
                    <Loader2 className="animate-spin" />
                  ) : (
                    <UserCircleIcon />
                  )}
                </Button>
              </div>
            </div>
          </LexicalComposer>
        ) : (
          <>
            <EditableArtworkPost />
            <div className="flex items-center justify-end p-4 text-neutral-400">
              <div className="flex items-center gap-4">
                {/* Status indicator */}
                {status === Status.Saving && (
                  <div className="flex items-center gap-2 text-sm text-neutral-400">
                    <Loader2 className="animate-spin" size={16} />
                    <span>Saving...</span>
                  </div>
                )}
                {/* {status === 'error' && (
                    <span className="text-sm text-red-500">Save failed</span>
                  )} */}

                <Button
                  variant="rounded_primary"
                  size="default"
                  onClick={async () => {
                    console.log('Publish button clicked');
                    await handleUpdate();
                  }}
                  disabled={isSubmitDisabled || status !== Status.Idle}
                  className="gap-2"
                >
                  {status !== Status.Idle ? (
                    <Loader2 className="animate-spin" />
                  ) : (
                    <UserCircleIcon />
                  )}
                </Button>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

function SelectPostType({
  postType,
  setPostType,
}: {
  postType: PostType;
  setPostType: (type: PostType) => void;
}) {
  return (
    <Select
      value={postType}
      onValueChange={(val) => setPostType(val as PostType)}
    >
      <SelectTrigger>
        <SelectValue placeholder="Select Post Type" />
      </SelectTrigger>
      <SelectContent className="bg-neutral-600 text-white border-0">
        <SelectItem value={PostType.General}>General</SelectItem>
        <SelectItem value={PostType.Artwork}>Artwork</SelectItem>
      </SelectContent>
    </Select>
  );
}

function EditableArtworkPost() {
  const {
    title,
    updateTitle,
    content,
    updateContent,
    image,
    updateImage,

    traits,
    updateTrait,
  } = usePostEditorContext();

  return (
    <ArtworkPost
      editMode={true}
      title={title}
      updateTitle={updateTitle}
      content={content}
      updateContent={updateContent}
      traits={traits}
      updateTrait={updateTrait}
      image={image}
      updateImage={updateImage}
    />
  );
}

export function ArtworkPost({
  editMode = true,
  title,
  updateTitle = () => {},
  content,
  updateContent = () => {},
  image,
  updateImage = () => {},

  traits,
  updateTrait = () => {},
}: {
  editMode?: boolean;

  title: string | null;
  updateTitle?: (title: string) => void;
  content: string | null;
  updateContent?: (content: string) => void;
  image: string | null;
  updateImage?: (image: string | null) => void;
  traits: ArtworkTrait[];
  updateTrait?: (
    trait_type: string,
    value: string,
    display_type?: ArtworkTraitDisplayType,
  ) => void;
}) {
  const t = useTranslations('EditArtworkPost');

  const [showColorPicker, setShowColorPicker] = useState(false);

  const handleImageUpload = () => {
    if (!editMode) return;
    const handleFileChange = (event: Event) => {
      const target = event.target as HTMLInputElement;
      const file = target.files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onloadend = () => {
          updateImage(reader.result as string);
        };
        reader.readAsDataURL(file);
      }
    };
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.id = 'imageUpload';
    input.onchange = handleFileChange;
    input.click();
  };
  const backgroundColor =
    String(
      traits.find((trait) => trait.trait_type === 'background_color')?.value,
    ) || '#ffffff';
  return (
    <div className="flex flex-row p-5 gap-5">
      <div className="flex-1 flex flex-col gap-4 p-4 [&>label]:text-neutral-50 [&>label]:font-sm">
        <label htmlFor="artwork">{t('artwork_name')}</label>
        <Input
          id="artwork"
          placeholder={t('placeholder', { fieldName: t('artwork_name') })}
          value={title || ''}
          disabled={!editMode}
          onChange={(e) => updateTitle(e.target.value)}
        />
        <label htmlFor="description">{t('description')}</label>
        <div
          id="description"
          className="min-h-[80px] relative rounded-md bg-input/30 border border-input px-3 py-1 "
        >
          <LexicalComposer initialConfig={editorConfig}>
            <Editor
              disabled={!editMode}
              content={content}
              updateContent={updateContent}
              placeholder={t('placeholder', { fieldName: t('description') })}
            />
          </LexicalComposer>
        </div>
        {traits.map((trait, index) => {
          let name = formatTraitType(trait.trait_type);
          try {
            name = t(trait.trait_type) || formatTraitType(trait.trait_type);
          } catch (error) {
            console.error('Error formatting trait name:', error);
          }

          switch (trait.display_type) {
            case null:
            case undefined:
            case ArtworkTraitDisplayType.String:
            case ArtworkTraitDisplayType.Number:
              return (
                <div key={index} className="flex flex-col gap-1">
                  <label htmlFor={`trait-${index}`}>{name}</label>
                  <Input
                    id={`trait-${index}`}
                    placeholder={t('placeholder', { fieldName: name })}
                    value={String(trait.value)}
                    disabled={!editMode}
                    onChange={(e) =>
                      updateTrait(trait.trait_type, e.target.value)
                    }
                  />
                </div>
              );
            case ArtworkTraitDisplayType.Color:
              return (
                <div key={index} className="flex flex-col gap-1">
                  <label htmlFor={`trait-${index}`}>{name} </label>
                  <div className="relative">
                    <Button
                      className="disabled:opacity-100"
                      style={{
                        backgroundColor: !editMode
                          ? backgroundColor
                          : undefined,
                      }}
                      disabled={!editMode}
                      onClick={() => setShowColorPicker(!showColorPicker)}
                      size="sm"
                    >
                      <span>
                        {!editMode ? backgroundColor : 'Select ' + name}
                      </span>
                    </Button>
                    {showColorPicker && (
                      <div className="absolute z-10 p-4 bg-neutral-600 bottom-0 left-0 flex flex-col gap-2 justify-center items-center">
                        <HexColorPicker
                          color={backgroundColor}
                          onChange={(value) =>
                            updateTrait(
                              trait.trait_type,
                              value,
                              ArtworkTraitDisplayType.Color,
                            )
                          }
                        />
                        <Button
                          className="w-full"
                          onClick={() => setShowColorPicker(false)}
                        >
                          Close
                        </Button>
                      </div>
                    )}
                  </div>
                </div>
              );
            default:
              return null;
          }
        })}
      </div>

      <div className="flex-1" style={{ backgroundColor }}>
        <button
          disabled={!editMode}
          onClick={handleImageUpload}
          className="relative w-full h-full p-4 flex items-center justify-center"
        >
          {image ? (
            <Image
              layout="fill"
              objectFit="contain"
              src={image}
              alt="Artwork"
              className="max-h-full max-w-full"
            />
          ) : (
            <div className="flex flex-col items-center">
              <div className="w-full px-2 py-1 rounded-full bg-neutral-600">
                No Image
              </div>

              <div className="text-neutral-400 text-sm mt-2">
                Click to upload artwork image
              </div>
            </div>
          )}
        </button>
      </div>
    </div>
  );
}

export function formatTraitType(traitType: string) {
  return traitType
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}
