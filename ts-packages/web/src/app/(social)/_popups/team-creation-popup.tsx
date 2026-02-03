import FileUploader from '@/features/spaces/files/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import {
  // InvalidDuplicatedUsername,
  InvalidLowerAlphaNumeric,
  InvalidTooShort,
} from '@/errors';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { checkLowerAlphaNumeric } from '@/lib/valid-utils';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { useCreateTeam } from '@/features/teams/hooks/use-create-team';

const defaultProfile =
  'https://metadata.ratel.foundation/ratel/default-profile.png';

export default function TeamCreationPopup() {
  const { t } = useTranslation('Home');
  const popup = usePopup();
  const navigate = useNavigate();

  const [profileUrl, setProfileUrl] = useState('');
  const [username, setUsername] = useState('');
  const [nickname, setNickname] = useState('');
  const [invalid, setInvalid] = useState<Error | undefined>(undefined);
  const [htmlContents, setHtmlContents] = useState('');

  const createTeamMutation = useCreateTeam().mutateAsync;

  const handleContents = (evt: React.FormEvent<HTMLTextAreaElement>) => {
    setHtmlContents(evt.currentTarget.value);
  };

  const handleCreate = async () => {
    if (
      checkString(nickname) ||
      checkString(username) ||
      checkString(htmlContents)
    ) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    try {
      logger.debug('Team creation button clicked');
      await createTeamMutation({
        username,
        nickname,
        profile_url: profileUrl || defaultProfile,
        description: htmlContents,
      });

      popup.close();

      // Redirect to the newly created team's home page
      navigate(route.teamByUsername(username));
    } catch (error) {
      logger.error('Failed to create team:', error);
      showErrorToast('Failed to create team. Please try again.');
    }
  };

  const handleUsername = async (evt: React.FormEvent<HTMLInputElement>) => {
    const username = evt.currentTarget.value;
    logger.debug('username', username);
    if (username.length < 3) {
      setInvalid(InvalidTooShort);
      return;
    }

    if (!checkLowerAlphaNumeric(username)) {
      setInvalid(InvalidLowerAlphaNumeric);
      return;
    }

    // console.log('users: ', users);

    // logger.debug('graphql respons: ', users);

    // if (users.length > 0) {
    //   setInvalid(InvalidDuplicatedUsername);
    //   return;
    // }

    setInvalid(undefined);
    setUsername(username);
  };

  const handleNickname = (evt: React.FormEvent<HTMLInputElement>) => {
    setNickname(evt.currentTarget.value);
  };

  return (
    <div className="w-100 max-tablet:w-full flex flex-col gap-10 items-center">
      <FileUploader onUploadSuccess={setProfileUrl}>
        <div className="group relative flex items-center justify-center size-40 max-mobile:size-20 mx-auto">
          <img
            src={profileUrl || defaultProfile}
            alt="logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer relative group max-mobile:size-20"
          />

          <div className="absolute w-40 h-40 inset-0 bg-component-bg/50 flex items-center justify-center text-center rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300 text-white font-semibold">
            {t('upload_logo')}
          </div>
        </div>
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Input
          type="text"
          placeholder={t('team_display_name')}
          onInput={handleNickname}
          data-pw="team-nickname-input"
        />
        <Col className="gap-0.25">
          <div className="relative">
            <span className="absolute left-3 top-1/2 transform -translate-y-1/2 ">
              @
            </span>
            <Input
              type="text"
              className="pl-8"
              placeholder={`${t('team_id')} (ex. ratel)`}
              onChange={handleUsername}
              aria-invalid={invalid !== undefined}
              data-pw="team-username-input"
            />
          </div>
          {invalid && (
            <div className="text-error text-sm font-light">
              {invalid.message}
            </div>
          )}
        </Col>
        <Textarea
          placeholder={t('team_description')}
          onChange={handleContents}
          data-pw="team-description-input"
        />
      </Col>
      <Row className="w-full grid grid-cols-2">
        <Button
          variant="rounded_secondary"
          className="w-full"
          onClick={() => popup.close()}
        >
          {t('cancel')}
        </Button>
        <Button
          className={
            checkString(nickname) ||
            checkString(username) ||
            checkString(htmlContents)
              ? 'cursor-not-allowed bg-neutral-600'
              : 'cursor-pointer bg-primary'
          }
          variant={'rounded_primary'}
          onClick={handleCreate}
          data-pw="team-create-button"
        >
          {t('create')}
        </Button>
      </Row>
    </div>
  );
}
