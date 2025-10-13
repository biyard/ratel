import FileUploader from '@/components/file-uploader';
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
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { checkLowerAlphaNumeric } from '@/lib/valid-utils';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useUserInfo } from '@/hooks/use-user-info';

export default function TeamCreationPopup() {
  const { t } = useTranslation('Home');
  const popup = usePopup();
  const userInfo = useUserInfo();

  const [profileUrl, setProfileUrl] = useState('');
  const [username, setUsername] = useState('');
  const [nickname, setNickname] = useState('');
  const [invalid, setInvalid] = useState<Error | undefined>(undefined);
  const [htmlContents, setHtmlContents] = useState('');

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
      await teamsV3Api.createTeam({
        username,
        nickname,
        profile_url: profileUrl,
        description: htmlContents,
      });
      
      userInfo.refetch();
      popup.close();
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
        {profileUrl ? (
          <img
            src={profileUrl}
            alt="Team Logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer"
          />
        ) : (
          <button className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50">
            {t('upload_logo')}
          </button>
        )}
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Input
          type="text"
          placeholder={t('team_display_name')}
          onInput={handleNickname}
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
        >
          {t('create')}
        </Button>
      </Row>
    </div>
  );
}
