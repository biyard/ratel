'use client';

import FileUploader from '@/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import { userEditProfileRequest } from '@/lib/api/models/user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';

import React, { useContext, useMemo, useState } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast, showInfoToast } from '@/lib/toast';
import Image from 'next/image';
import { useTranslations } from 'next-intl';
import { usePopup } from '@/lib/contexts/popup-service';
import DeleteTeamPopup from './_components/delete-team-popup';
import { deleteTeamRequest } from '@/lib/api/models/team';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { logger } from '@/lib/logger';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { FeedStatus } from '@/lib/api/models/feeds';
import { GroupPermission } from '@/lib/api/models/group';
import { usePermission } from '@/app/(social)/_hooks/use-permission';

export default function SettingsPage({ username }: { username: string }) {
  const t = useTranslations('Team');
  const popup = usePopup();
  const queryClient = getQueryClient();
  const { teams, updateSelectedTeam, setSelectedTeam } =
    useContext(TeamContext);

  const team = useMemo(() => {
    return teams.find((t) => t.username === username);
  }, [teams, username]);

  const { post } = useApiCall();
  const router = useRouter();
  const userInfo = useUserInfo();

  const [profileUrl, setProfileUrl] = useState(team?.profile_url || '');
  const [nickname, setNickname] = useState(team?.nickname);
  const [htmlContents, setHtmlContents] = useState(team?.html_contents);

  // TODO: Update to use v3 permissions with username instead of id
  const deleteTeamPermission =
    usePermission(team?.username ?? '', GroupPermission.DeleteGroup).data
      .has_permission ?? false;

  if (!team) {
    return <></>;
  }
  const handleContents = (evt: React.FormEvent<HTMLTextAreaElement>) => {
    setHtmlContents(evt.currentTarget.value);
  };

  const handleNickname = (evt: React.FormEvent<HTMLInputElement>) => {
    setNickname(evt.currentTarget.value);
  };

  const handleProfileUrl = (url: string) => {
    setProfileUrl(url);
  };

  const openDeletePopup = () => {
    popup
      .open(
        <DeleteTeamPopup
          onConfirm={async () => {
            try {
              await post(
                ratelApi.teams.deleteTeam(),
                deleteTeamRequest(team!.id),
              );
              showInfoToast(t('success_delete_team'));
              // Invalidate all published feeds after deleting team
              await queryClient.invalidateQueries({
                queryKey: feedKeys.list({
                  status: FeedStatus.Published,
                }),
              });
              userInfo.refetch();
              setSelectedTeam(0);
              router.push('/');
            } catch (e) {
              logger.error('failed to delete team with error: ', e);
              showErrorToast(t('failed_delete_team'));
            } finally {
              popup.close();
            }
          }}
          onCancel={() => {
            popup.close();
          }}
        />,
      )
      .withTitle('');
  };

  const handleSave = async () => {
    if (checkString(nickname ?? '') || checkString(htmlContents ?? '')) {
      showErrorToast(t('remove_test_keyword'));
      return;
    }

    await post(
      ratelApi.users.editProfile(team!.id),
      userEditProfileRequest(nickname!, htmlContents!, profileUrl),
    );

    updateSelectedTeam({
      ...team!,
      nickname: nickname!,
      html_contents: htmlContents!,
      profile_url: profileUrl,
    });

    router.push(route.teamByUsername(username));
  };

  const invalidInput =
    checkString(nickname ?? '') || checkString(htmlContents ?? '');

  return (
    <div className="w-full max-tablet:w-full flex flex-col gap-10 items-center">
      <FileUploader onUploadSuccess={handleProfileUrl}>
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
            {t('upload_logo')}
          </button>
        )}
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('username')}
          </label>
          <Input
            type="text"
            className="text-text-primary bg-input-box-bg border border-input-box-border"
            disabled
            value={`@${team?.username}`}
          />
        </Row>
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('display_name')}
          </label>
          <Input
            type="text"
            className="text-text-primary bg-input-box-bg border border-input-box-border"
            placeholder={t('display_name_hint')}
            value={nickname}
            onInput={handleNickname}
          />
        </Row>
        <Col>
          <label className="w-40 font-bold text-text-primary">
            {t('description')}
          </label>
          <Textarea
            className="text-text-primary bg-input-box-bg border border-input-box-border"
            placeholder={t('team_description_hint')}
            value={htmlContents}
            onChange={handleContents}
          />
        </Col>
        <Row className="justify-end py-5">
          <Button
            disabled={invalidInput}
            className={invalidInput ? 'bg-neutral-600' : 'bg-primary'}
            variant={'rounded_primary'}
            onClick={handleSave}
          >
            {t('save')}
          </Button>

          {deleteTeamPermission && (
            <Button
              disabled={invalidInput}
              className={
                invalidInput
                  ? 'bg-neutral-600'
                  : 'bg-red-600 hover:bg-red-600/90'
              }
              variant={'rounded_primary'}
              onClick={openDeletePopup}
            >
              {t('delete')}
            </Button>
          )}
        </Row>
      </Col>
    </div>
  );
}
