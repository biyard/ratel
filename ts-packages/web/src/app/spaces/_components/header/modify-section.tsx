import {
  BackButton,
  EditButton,
  MakePublicButton,
  PublishSpaceButton,
  SaveButton,
} from '@/components/post-header/buttons';

interface SpaceModifySectionProps {
  isEditable: boolean;
  isPublished: boolean;

  canMakePublic: boolean;

  isEditingMode: boolean;
  hasUnsavedChanges: boolean;
  hasEditPermission: boolean;

  onGoBack: () => void;
  onStartEdit: () => void;
  onSave: () => void;

  onPublishButtonClick: () => void;
  onMakePublicButtonClick: () => void;
}

export default function SpaceModifySection({
  isEditable,
  isPublished,
  canMakePublic,

  isEditingMode,
  hasUnsavedChanges,
  hasEditPermission,

  onGoBack,
  onStartEdit,
  onSave,

  onPublishButtonClick,
  onMakePublicButtonClick,
}: SpaceModifySectionProps) {
  return (
    <div className="flex flex-row justify-between items-center w-full">
      <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
        <BackButton onClick={onGoBack} />
      </div>

      {hasEditPermission && (
        <div className="flex flex-row items-center gap-2 text-sm text-white">
          {isEditable ? (
            isEditingMode ? (
              <SaveButton onClick={onSave} disabled={!hasUnsavedChanges} />
            ) : (
              <EditButton onClick={onStartEdit} />
            )
          ) : (
            <></>
          )}

          {!isPublished && (
            <PublishSpaceButton onClick={onPublishButtonClick} />
          )}
          {canMakePublic && (
            <MakePublicButton onClick={onMakePublicButtonClick} />
          )}
        </div>
      )}
    </div>
  );
}
