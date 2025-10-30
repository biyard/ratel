import {
  useCreateArtworkPageController,
  CreateArtworkPageController,
} from './use-create-artwork-page-controller';
import { Col } from '@/components/ui/col';
import { RequireAuth } from '@/components/auth/require-auth';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { Loader2, Check, Clock } from 'lucide-react';
import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { ArtworkTraits } from './artwork-traits';
import { ArtworkImageUpload } from './artwork-image-upload';
import { EditorStatus } from '../create-post-page/use-create-post-page-controller';
import { SPACE_DEFINITIONS } from '@/features/spaces/types/space-definition';
import { PostFormFields } from '../post-form-fields';

export default function CreateArtworkPage() {
  return (
    <RequireAuth>
      <CreateArtworkPageContent />
    </RequireAuth>
  );
}

function CreateArtworkPageContent() {
  const ctrl = useCreateArtworkPageController();
  const activeTab = ctrl.activeTab.get();

  return (
    <Col className="gap-5 py-5 px-4 mx-auto w-full max-w-[906px]">
      {/* Header */}
      <h1 className="text-2xl font-bold text-text-primary">
        {ctrl.t.page_title}
      </h1>

      {/* Tab Navigation */}
      <div className="flex gap-2 border-b border-border">
        <button
          onClick={() => ctrl.activeTab.set('content')}
          className={cn(
            'px-4 py-2 font-medium transition-colors',
            activeTab === 'content'
              ? 'border-b-2 border-primary text-text-primary'
              : 'text-text-secondary hover:text-text-primary',
          )}
        >
          Title / Description
        </button>
        <button
          onClick={() => ctrl.activeTab.set('image')}
          className={cn(
            'px-4 py-2 font-medium transition-colors',
            activeTab === 'image'
              ? 'border-b-2 border-primary text-text-primary'
              : 'text-text-secondary hover:text-text-primary',
          )}
        >
          Image
        </button>
        <button
          onClick={() => ctrl.activeTab.set('traits')}
          className={cn(
            'px-4 py-2 font-medium transition-colors',
            activeTab === 'traits'
              ? 'border-b-2 border-primary text-text-primary'
              : 'text-text-secondary hover:text-text-primary',
          )}
        >
          Traits
        </button>
      </div>

      {/* Tab Content */}
      {activeTab === 'content' && (
        <PostFormFields
          title={ctrl.title.get()}
          titleMaxLength={ctrl.TITLE_MAX_LENGTH}
          onTitleChange={ctrl.handleTitleChange}
          content={ctrl.content.get()}
          onContentUpdate={ctrl.handleContentUpdate}
          editorRef={ctrl.editorRef}
          imageUrl={null}
          onImageUpload={ctrl.handleImageUpload}
          onRemoveImage={ctrl.handleRemoveImage}
          status={ctrl.status.get()}
          lastSavedAt={ctrl.lastSavedAt.get()}
          isModified={ctrl.isModified.get()}
          formatLastSaved={ctrl.formatLastSaved}
          disabledImageUpload={true}
          t={ctrl.t}
        />
      )}

      {activeTab === 'image' && <ArtworkImageSection ctrl={ctrl} />}

      {activeTab === 'traits' && <ArtworkTraitsSection ctrl={ctrl} />}

      <SpaceTypeItem
        spaceDefinition={SPACE_DEFINITIONS[4]}
        selected={true}
        disabled={true}
        onClick={() => {}}
      />

      {/* Bottom Actions */}
      <div className="flex gap-4 justify-end items-center">
        <Button
          id="publish-artwork-button"
          variant="rounded_primary"
          size="default"
          onClick={ctrl.handleArtworkNext}
          disabled={ctrl.isPublishDisabled}
          className={cn(
            'min-w-[150px]',
            ctrl.isPublishDisabled && 'opacity-50',
          )}
        >
          {ctrl.status.get() === EditorStatus.Saving ? (
            <>
              <Loader2 className="animate-spin" size={16} />
              <span>{ctrl.t.saving}</span>
            </>
          ) : (
            <span>{ctrl.t.btn_next}</span>
          )}
        </Button>
      </div>
    </Col>
  );
}

function ArtworkImageSection({ ctrl }: { ctrl: CreateArtworkPageController }) {
  return (
    <Col className="gap-4">
      <ArtworkImageUpload
        imageUrl={ctrl.image.get()}
        onImageUpload={ctrl.handleImageUpload}
        t={ctrl.t}
      />
    </Col>
  );
}

function ArtworkTraitsSection({ ctrl }: { ctrl: CreateArtworkPageController }) {
  const formatTraitsLastSaved = (date: Date | null): string => {
    if (!date) return '';
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  };

  return (
    <Col className="gap-4">
      <ArtworkTraits
        traits={ctrl.traits.get()}
        onTraitAdd={ctrl.handleTraitAdd}
        onTraitUpdate={ctrl.handleTraitUpdate}
        onTraitRemove={ctrl.handleTraitRemove}
        t={ctrl.t}
      />

      {/* Traits Saving Status */}
      <div className="flex gap-2 items-center text-xs text-neutral-400">
        {ctrl.isTraitsSaving.get() ? (
          <>
            <Loader2 className="animate-spin" size={14} />
            <span>{ctrl.t.saving}</span>
          </>
        ) : ctrl.isTraitsModified ? (
          <>
            <Clock size={14} className="text-yellow-500" />
            <span className="text-yellow-500">{ctrl.t.unsaved_changes}</span>
          </>
        ) : ctrl.traitsLastSavedAt.get() ? (
          <>
            <Check size={14} className="text-green-500" />
            <span className="text-green-500">
              Traits saved at{' '}
              {formatTraitsLastSaved(ctrl.traitsLastSavedAt.get())}
            </span>
          </>
        ) : null}
      </div>
    </Col>
  );
}
