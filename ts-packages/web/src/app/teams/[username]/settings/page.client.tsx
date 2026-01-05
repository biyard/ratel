'use client';

import FileUploader from '@/features/spaces/files/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';

import { useContext, useMemo, useState } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import { useNavigate } from 'react-router';
// import { route } from '@/route';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast, showInfoToast } from '@/lib/toast';

import { useTranslation } from 'react-i18next';
import { usePopup } from '@/lib/contexts/popup-service';
import DeleteTeamPopup from './_components/delete-team-popup';
import { logger } from '@/lib/logger';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { FeedStatus } from '@/features/posts/types/post';
import {
  useTeamDetailByUsername,
  useTeamPermissionsFromDetail,
} from '@/features/teams/hooks/use-team';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import { useUserInfo } from '@/hooks/use-user-info';

import daoRegistryArtifact from '../../../../contracts/DaoRegistry.json';
import daoRegistryStateV1Artifact from '../../../../contracts/DaoRegistryStateV1.json';
import { ethers } from 'ethers';
import { getKaiaSigner } from '@/lib/service/kaia-wallet-service';
import { config } from '@/config';

export default function SettingsPage({ username }: { username: string }) {
  const { t } = useTranslation('Team');
  const popup = usePopup();
  const queryClient = getQueryClient();
  const { teams, updateSelectedTeam, setSelectedTeam } =
    useContext(TeamContext);

  // Use v3 API to get team details with permissions
  const teamDetailQuery = useTeamDetailByUsername(username);
  const userInfo = useUserInfo();

  // Get permissions directly from team detail response (no API calls!)
  const permissions = useTeamPermissionsFromDetail(teamDetailQuery.data);

  // Get legacy team from context for backward compatibility
  const team = useMemo(() => {
    return teams.find((t) => t.username === username);
  }, [teams, username]);

  const navigate = useNavigate();

  const [profileUrl, setProfileUrl] = useState(team?.profile_url || '');
  const [nickname, setNickname] = useState(team?.nickname);
  const [htmlContents, setHtmlContents] = useState(team?.html_contents);
  const [daoAddress, setDaoAddress] = useState(team?.dao_address);
  const [isConnectingWallet, setIsConnectingWallet] = useState(false);
  const deleteTeamPermission =
    permissions?.has(TeamGroupPermission.TeamAdmin) ?? false;

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
            if (!teamDetailQuery.data) return;

            try {
              await teamsV3Api.deleteTeam(username);
              showInfoToast(t('success_delete_team'));

              // Invalidate all team-related queries
              await queryClient.invalidateQueries({
                predicate: (query) =>
                  query.queryKey[0]?.toString().includes('team') ||
                  query.queryKey[0]?.toString().includes('user-info'),
              });

              // Invalidate all published feeds after deleting team
              await queryClient.invalidateQueries({
                queryKey: feedKeys.list({
                  status: FeedStatus.Published,
                }),
              });

              userInfo.refetch();
              setSelectedTeam(0);
              navigate('/');
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

  const handleSave = async (nextDaoAddress?: string) => {
    if (!teamDetailQuery.data) return;

    if (checkString(nickname ?? '') || checkString(htmlContents ?? '')) {
      showErrorToast(t('remove_test_keyword'));
      return;
    }

    const mergedDaoAddress =
      nextDaoAddress ?? daoAddress ?? team.dao_address ?? undefined;

    try {
      await teamsV3Api.updateTeam(teamDetailQuery.data.id, {
        nickname: nickname || undefined,
        description: htmlContents || undefined,
        profile_url: profileUrl || undefined,
        dao_address: mergedDaoAddress,
      });

      teamDetailQuery.refetch();

      updateSelectedTeam({
        ...team!,
        nickname: nickname!,
        html_contents: htmlContents!,
        profile_url: profileUrl,
        dao_address: mergedDaoAddress ?? null,
      });

      setDaoAddress(mergedDaoAddress ?? null);

      // navigate(route.teamByUsername(username));
    } catch (e) {
      logger.error('Failed to update team:', e);
      showErrorToast(t('failed_update_team') || 'Failed to update team');
    }
  };

  const handleActivateDao = async () => {
    try {
      setIsConnectingWallet(true);

      const { signer, account } = await getKaiaSigner(
        config.env === 'dev' || config.env === 'local' ? 'testnet' : 'mainnet',
      );

      const { abi: stateAbi, bytecode: stateBytecode } =
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        daoRegistryStateV1Artifact as any;
      const stateFactory = new ethers.ContractFactory(
        stateAbi,
        stateBytecode,
        signer,
      );
      const stateContract = await stateFactory.deploy(account);
      const stateDeployed = await stateContract.waitForDeployment();
      const stateAddress = await stateDeployed.getAddress();

      const { abi: registryAbi, bytecode: registryBytecode } =
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        daoRegistryArtifact as any;
      const daoName =
        teamDetailQuery.data.nickname || teamDetailQuery.data.username || 'DAO';

      const registryFactory = new ethers.ContractFactory(
        registryAbi,
        registryBytecode,
        signer,
      );
      const registryContract = await registryFactory.deploy(
        daoName,
        stateAddress,
        config.operator_address,
      );
      const registryDeployed = await registryContract.waitForDeployment();
      const registryAddress = await registryDeployed.getAddress();

      await handleSave(registryAddress);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (e: any) {
      logger.error('Failed to activate DAO (deploy DaoRegistry)', e);
      showErrorToast(t('failed_active_dao'));
    } finally {
      setIsConnectingWallet(false);
    }
  };

  const invalidInput =
    checkString(nickname ?? '') || checkString(htmlContents ?? '');

  return (
    <div className="w-full max-tablet:w-full flex flex-col gap-10 items-center">
      <FileUploader
        onUploadSuccess={handleProfileUrl}
        data-pw="team-profile-uploader"
      >
        {profileUrl ? (
          <img
            src={profileUrl}
            alt="Team Logo"
            width={80}
            height={80}
            className="w-40 h-40 rounded-full object-cover cursor-pointer"
            data-pw="team-profile-image"
          />
        ) : (
          <button
            className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50"
            data-pw="team-profile-upload-button"
          >
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
            data-pw="team-username-display"
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
            data-pw="team-nickname-input"
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
            data-pw="team-description-input"
          />
        </Col>
        {config.experiment && (
          <Row className="items-center">
            <label className="w-35 font-bold text-text-primary">
              {t('dao_address')}
            </label>
            {team.dao_address ? (
              <span className="text-sm text-text-primary break-all">
                {team.dao_address}
              </span>
            ) : (
              <Button
                variant="primary"
                onClick={handleActivateDao}
                disabled={isConnectingWallet}
                data-pw="team-dao-activate-button"
              >
                {isConnectingWallet ? t('activating_dao') : t('activate_dao')}
              </Button>
            )}
          </Row>
        )}

        <Row className="justify-end py-5">
          <Button
            disabled={invalidInput}
            className={invalidInput ? 'bg-neutral-600' : 'bg-primary'}
            variant={'rounded_primary'}
            onClick={async () => {
              await handleSave();
            }}
            data-pw="team-settings-save-button"
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
              data-pw="team-delete-button"
            >
              {t('delete')}
            </Button>
          )}
        </Row>
      </Col>
    </div>
  );
}
