import FileUploader from '@/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import {
  InvalidDuplicatedUsername,
  InvalidLowerAlphaNumeric,
  InvalidTooShort,
} from '@/errors';
import { useUserInfo } from '@/lib/api/hooks/users';
import { createTeamRequest } from '@/lib/api/models/team';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { checkLowerAlphaNumeric } from '@/lib/valid-utils';
import { useApolloClient } from '@apollo/client';
import React, { useState } from 'react';
import Image from 'next/image';

export default function TeamCreationPopup() {
  const popup = usePopup();
  const { post } = useApiCall();
  const client = useApolloClient();
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
    logger.debug('Team creation button clicked');
    await post(
      ratelApi.teams.createTeam(),
      createTeamRequest(profileUrl, username, nickname, htmlContents),
    );
    userInfo.refetch();

    popup.close();
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

    const {
      data: { users },
    } = await client.query(ratelApi.graphql.getUserByUsername(username));
    logger.debug('graphql respons: ', users);

    if (users.length > 0) {
      setInvalid(InvalidDuplicatedUsername);
      return;
    }

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
          <Image
            src={profileUrl}
            alt="Team Logo"
            width={80}
            height={80}
            className="w-40 h-40 rounded-full object-cover cursor-pointer"
          />
        ) : (
          <button className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50">
            Upload logo
          </button>
        )}
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Input
          type="text"
          placeholder="Team display name"
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
              placeholder="Team ID (ex. ratel)"
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
          placeholder="Please type description of your team."
          onChange={handleContents}
        />
      </Col>
      <Row className="w-full grid grid-cols-2">
        <Button
          variant="rounded_secondary"
          className="w-full"
          onClick={() => popup.close()}
        >
          Cancel
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
          Create
        </Button>
      </Row>
    </div>
  );
}
