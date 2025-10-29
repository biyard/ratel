import { Loader2, Check, Clock } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Checkbox } from '@/components/checkbox/checkbox';
import { cn } from '@/lib/utils';
import { TiptapEditor } from '@/components/text-editor';

import {
  useCreatePostPageController,
  EditorStatus,
} from './use-create-post-page-controller';
import { SpaceTypeCarousel } from './space-type-carousel';
import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';

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

        {/* Rich Text Editor - TipTap */}
        <div className="relative">
          <TiptapEditor
            ref={ctrl.editorRef}
            content={ctrl.content.get() || ''}
            onUpdate={ctrl.handleContentUpdate}
            placeholder={ctrl.t.content_placeholder}
            data-pw="post-content-editor"
            minHeight="300px"
          />

          {/* Saving Status Indicator - positioned at bottom right of editor */}
          <div className="flex absolute right-3 bottom-3 gap-2 items-center py-1 px-2 text-xs rounded text-neutral-400 bg-card">
            {ctrl.status.get() === EditorStatus.Saving ? (
              <>
                <Loader2 className="animate-spin" size={14} />
                <span>{ctrl.t.saving}</span>
              </>
            ) : ctrl.isModified.get() ? (
              <>
                <Clock size={14} className="text-yellow-500" />
                <span className="text-yellow-500">
                  {ctrl.t.unsaved_changes}
                </span>
              </>
            ) : ctrl.lastSavedAt.get() ? (
              <>
                <Check size={14} className="text-green-500" />
                <span className="text-green-500">
                  {ctrl.t.all_changes_saved}
                </span>
              </>
            ) : null}
          </div>
        </div>

        {ctrl.lastSavedAt.get() && (
          <Row className="justify-end items-center w-full text-xs text-neutral-400">
            {ctrl.formatLastSaved(ctrl.lastSavedAt.get())}
          </Row>
        )}
      </Col>

      {/* Space Creation Section */}
      {!ctrl.disableSpaceSelector.get() && !ctrl.skipCreatingSpace.get() && (
        <SpaceTypeCarousel>{renderedForms}</SpaceTypeCarousel>
      )}

      {ctrl.disableSpaceSelector.get() && (
        <SpaceTypeItem
          key={ctrl.spaceDefinitions[ctrl.selected.get()].labelKey}
          spaceDefinition={ctrl.spaceDefinitions[ctrl.selected.get()]}
          selected={true}
          onClick={() => {}}
        />
      )}

      {/* Bottom Actions */}
      <div className="flex gap-4 justify-end items-center">
        {!ctrl.disableSpaceSelector.get() && (
          <Checkbox
            id="skip-space"
            value={ctrl.skipCreatingSpace.get()}
            onChange={(checked) => ctrl.skipCreatingSpace.set(checked)}
          >
            <span
              className="text-sm cursor-pointer text-text-primary"
              onClick={() =>
                ctrl.skipCreatingSpace.set(!ctrl.skipCreatingSpace.get())
              }
            >
              {ctrl.t.skip_creating_space}
            </span>
          </Checkbox>
        )}

        <Button
          variant="rounded_primary"
          size="default"
          onClick={ctrl.handleSubmit}
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
