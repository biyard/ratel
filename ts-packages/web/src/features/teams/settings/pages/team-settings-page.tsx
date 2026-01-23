import { useTeamSettingsPageController } from '../hooks/use-team-settings-page-controller';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';
import { ProfileSection } from '../components/profile-section';
import { BasicInfoSection } from '../components/basic-info-section';

export default function TeamSettingsPage({ username }: { username: string }) {
  const ctrl = useTeamSettingsPageController(username);

  if (!ctrl.team) {
    return <></>;
  }

  return (
    <div className="w-full max-tablet:w-full flex flex-col gap-10 items-center">
      <ProfileSection
        profileUrl={ctrl.profileUrl}
        onProfileUrlChange={ctrl.updateProfileUrl}
        uploadLogoText={ctrl.i18n.upload_logo}
        isEditing={ctrl.isEditing}
      />

      <Col className="w-full gap-2.5">
        <BasicInfoSection
          username={username}
          nickname={ctrl.nickname}
          htmlContents={ctrl.htmlContents}
          onNicknameChange={ctrl.updateNickname}
          onDescriptionChange={ctrl.updateContents}
          isEditing={ctrl.isEditing}
          i18n={ctrl.i18n}
        />

        {!ctrl.isEditing ? (
          // Read-only mode - show Edit and Delete buttons
          <Row className="justify-end py-5 gap-2">
            <Button
              onClick={ctrl.handleEdit}
              variant="rounded_primary"
              data-pw="team-settings-edit-button"
            >
              {ctrl.i18n.edit}
            </Button>
            {ctrl.deleteTeamPermission && (
              <Button
                onClick={ctrl.openDeletePopup}
                className="bg-red-600 hover:bg-red-600/90"
                variant="rounded_primary"
                data-pw="team-delete-button"
              >
                {ctrl.i18n.delete}
              </Button>
            )}
          </Row>
        ) : (
          // Edit mode - show Save and Cancel buttons (Delete hidden)
          <Row className="justify-end py-5 gap-2">
            <Button
              onClick={ctrl.handleCancel}
              variant="rounded_primary"
              data-pw="team-settings-cancel-button"
            >
              {ctrl.i18n.cancel}
            </Button>
            <Button
              onClick={ctrl.handleSave}
              disabled={ctrl.isSaving}
              className="bg-primary"
              variant="rounded_primary"
              data-pw="team-settings-save-button"
            >
              {ctrl.isSaving ? 'Saving...' : ctrl.i18n.save_changes}
            </Button>
          </Row>
        )}
      </Col>
    </div>
  );
}
