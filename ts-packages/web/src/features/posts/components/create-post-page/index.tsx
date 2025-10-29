'use client';

import { X, Loader2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Checkbox } from '@/components/checkbox/checkbox';
import { cn } from '@/lib/utils';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import ToolbarPlugin from '@/components/toolbar/toolbar';

import {
  useCreatePostPageController,
  EditorStatus,
} from './use-create-post-page-controller';
import { editorConfig } from './editor-config';
import { Editor } from './editor';
import { SpaceTypeCarousel } from './space-type-carousel';
import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';

export { editorTheme } from './editor-theme';

export default function CreatePostPage() {
  const ctrl = useCreatePostPageController();

  const renderedForms = ctrl.spaceDefinitions.map((form, i) => (
    <SpaceTypeItem
      key={form.labelKey}
      spaceDefinition={form}
      selected={i === ctrl.selected.get()}
      onClick={() => ctrl.handleSelectSpace(i)}
    />
  ));

  return (
    <Col className="gap-5 py-5 px-4 mx-auto w-full max-w-[906px]">
      {/* Header */}
      <h1 className="text-2xl font-bold text-text-primary">
        {ctrl.t.page_title}
      </h1>

      <Col>
        {/* Title Input */}
        <div className="relative">
          <Input
            type="text"
            placeholder={ctrl.t.title_placeholder}
            value={ctrl.title.get()}
            onChange={ctrl.handleTitleChange}
            className="w-full text-text-primary bg-input-box-bg border-input-box-border"
          />
          <div className="absolute right-3 top-1/2 text-sm -translate-y-1/2 text-neutral-400">
            {ctrl.title.get().length}/{ctrl.TITLE_MAX_LENGTH}
          </div>
        </div>

        {/* Rich Text Editor */}
        <div className="relative rounded-md border border-input-box-border bg-input-box-bg">
          <LexicalComposer initialConfig={editorConfig}>
            <div className="relative text-base min-h-[200px] text-text-primary">
              <Editor
                label="post-content-editor"
                disabled={false}
                content={ctrl.content.get()}
                updateContent={ctrl.handleContentChange}
                placeholder={ctrl.t.content_placeholder}
              />
            </div>

            {/* Image Preview */}
            {ctrl.image.get() && (
              <div className="px-5 pb-3">
                <div className="inline-block relative">
                  <img
                    src={ctrl.image.get()}
                    alt="Uploaded"
                    className="object-cover w-16 h-16 rounded-lg border border-neutral-600"
                  />
                  <button
                    onClick={ctrl.handleRemoveImage}
                    className="flex absolute -top-1.5 -right-1.5 justify-center items-center w-5 h-5 text-white bg-red-600 rounded-full border-2 hover:bg-red-700 border-component-bg"
                    aria-label={ctrl.t.remove_image}
                  >
                    <X size={12} />
                  </button>
                </div>
              </div>
            )}

            {/* Toolbar and Auto-save indicator */}
            <div className="flex justify-between items-center py-3 px-5 border-t border-input-box-border">
              <ToolbarPlugin
                onImageUpload={ctrl.handleImageUpload}
                enableImage={true}
              />
            </div>
          </LexicalComposer>
        </div>

        {ctrl.lastSavedAt.get() && (
          <div className="text-xs text-neutral-400">
            {ctrl.formatLastSaved(ctrl.lastSavedAt.get())}
          </div>
        )}
      </Col>

      {/* Space Creation Section */}
      {!ctrl.skipCreatingSpace.get() && (
        <SpaceTypeCarousel>{renderedForms}</SpaceTypeCarousel>
      )}

      {/* Bottom Actions */}
      <div className="flex gap-4 justify-end items-center">
        <Checkbox
          id="skip-space"
          value={ctrl.skipCreatingSpace.get()}
          onChange={(checked) => ctrl.skipCreatingSpace.set(checked)}
        >
          <span className="text-sm text-text-primary">
            {ctrl.t.skip_creating_space}
          </span>
        </Checkbox>

        <Button
          variant="rounded_primary"
          size="default"
          onClick={ctrl.handlePublish}
          disabled={ctrl.isPublishDisabled}
          className={cn(
            'min-w-[150px]',
            ctrl.isPublishDisabled && 'opacity-50',
          )}
        >
          {ctrl.status.get() === EditorStatus.Publishing ? (
            <>
              <Loader2 className="animate-spin" size={16} />
              <span>{ctrl.t.publishing}</span>
            </>
          ) : (
            <span>{ctrl.actionButtonText}</span>
          )}
        </Button>
      </div>

      {/* Saving Indicator */}
      {ctrl.status.get() === EditorStatus.Saving && (
        <div className="flex gap-2 justify-center items-center mt-4 text-sm text-neutral-400">
          <Loader2 className="animate-spin" size={16} />
          <span>{ctrl.t.saving}</span>
        </div>
      )}
    </Col>
  );
}
