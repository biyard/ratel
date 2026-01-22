import { useTeamSettingsPageController } from '../hooks/use-team-settings-page-controller';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';
import { ProfileSection } from '../components/profile-section';
import { BasicInfoSection } from '../components/basic-info-section';
import { DaoSection } from '../components/dao-section';

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
      />

      <Col className="w-full gap-2.5">
        <BasicInfoSection
          username={username}
          nickname={ctrl.nickname}
          htmlContents={ctrl.htmlContents}
          onNicknameChange={ctrl.updateNickname}
          onDescriptionChange={ctrl.updateContents}
          i18n={ctrl.i18n}
        />

        {/* <DaoSection
          daoAddress={ctrl.daoAddress}
          isDaoConnected={ctrl.isDaoConnected}
          onActivate={ctrl.handleActivateDao}
          i18n={ctrl.i18n}
        />

        <Row className="justify-end py-5">
          <Button
            disabled={ctrl.invalidInput}
            className={ctrl.invalidInput ? 'bg-neutral-600' : 'bg-primary'}
            variant={'rounded_primary'}
            onClick={async () => {
              await ctrl.handleSave();
            }}
            data-pw="team-settings-save-button"
          >
            {ctrl.i18n.save}
          </Button>

          {ctrl.deleteTeamPermission && (
            <Button
              disabled={ctrl.invalidInput}
              className={
                ctrl.invalidInput
                  ? 'bg-neutral-600'
                  : 'bg-red-600 hover:bg-red-600/90'
              }
              variant={'rounded_primary'}
              onClick={ctrl.openDeletePopup}
              data-pw="team-delete-button"
            >
              {ctrl.i18n.delete}
            </Button>
          )} */}
        {/* </Row> */}
      </Col>
    </div>
  );
}
