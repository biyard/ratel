import {
  useCreatePostPageController,
  EditorStatus,
} from './use-create-post-page-controller';
import { Col } from '@/components/ui/col';
import { RequireAuth } from '@/components/auth/require-auth';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/checkbox/checkbox';
import { cn } from '@/lib/utils';
import { Loader2 } from 'lucide-react';
import { SpaceTypeCarousel } from './space-type-carousel';
import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { PostFormFields } from '../post-form-fields';

export default function CreatePostPage() {
  return (
    <RequireAuth>
      <CreatePostPageContent />
    </RequireAuth>
  );
}

function CreatePostPageContent() {
  const ctrl = useCreatePostPageController();

  return (
    <Col className="gap-5 py-5 px-4 mx-auto w-full max-w-[906px]">
      {/* Header */}
      <h1 className="text-2xl font-bold text-text-primary">
        {ctrl.t.page_title}
      </h1>

      <PostFormFields
        title={ctrl.title.get()}
        titleMaxLength={ctrl.TITLE_MAX_LENGTH}
        onTitleChange={ctrl.handleTitleChange}
        content={ctrl.content.get()}
        onContentUpdate={ctrl.handleContentUpdate}
        editorRef={ctrl.editorRef}
        imageUrl={ctrl.image.get()}
        onImageUpload={ctrl.handleImageUpload}
        onRemoveImage={ctrl.handleRemoveImage}
        status={ctrl.status.get()}
        lastSavedAt={ctrl.lastSavedAt.get()}
        isModified={ctrl.isModified.get()}
        formatLastSaved={ctrl.formatLastSaved}
        t={ctrl.t}
      />

      {/* Space Creation Section */}
      {!ctrl.disableSpaceSelector.get() && !ctrl.skipCreatingSpace.get() && (
        <SpaceTypeCarousel>
          {ctrl.spaceDefinitions.map((form, i) => (
            <SpaceTypeItem
              key={form.labelKey}
              spaceDefinition={form}
              selected={i === ctrl.selected.get()}
              onClick={() => ctrl.handleSelectSpace(i)}
            />
          ))}
        </SpaceTypeCarousel>
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
            onChange={(checked: boolean) => ctrl.skipCreatingSpace.set(checked)}
            data-testid="skip-creating-space-checkbox"
          >
            <span className="text-sm text-text-primary">
              {ctrl.t.skip_creating_space}
            </span>
          </Checkbox>
        )}

        <Button
          id="publish-post-button"
          data-testid="publish-post-button"
          variant="rounded_primary"
          size="default"
          onClick={ctrl.handleSubmit}
          disabled={ctrl.isPublishDisabled}
          className={cn(
            'min-w-[150px] text-base',
            ctrl.isPublishDisabled
              ? 'dark:!bg-post-save-button-disabled-bg dark:!text-post-save-button-disabled-text opacity-100'
              : '',
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
