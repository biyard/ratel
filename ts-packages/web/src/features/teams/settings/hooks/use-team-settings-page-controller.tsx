import { useState } from 'react';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';
import { useTeamSettingsI18n } from '../i18n';
import { useSuspenseFindTeam } from '../../hooks/use-find-team';

export function useTeamSettingsPageController(username: string) {
  const i18n = useTeamSettingsI18n();
  // const popup = usePopup();
  // const navigate = useNavigate();
  // const { updateSelectedTeam, setSelectedTeam } = useContext(TeamContext);

  // Fetch team data
  const { data: team } = useSuspenseFindTeam(username);

  const [isEditing, setIsEditing] = useState(false);
  // // Form state
  const [profileUrl, setProfileUrl] = useState<string | null>(null);
  const [nickname, setNickname] = useState<string | null>(null);
  const [htmlContents, setHtmlContents] = useState<string | null>(null);

  const handleStartEdit = () => {
    setIsEditing(() => {
      setProfileUrl(team?.profile_url || '');
      setNickname(team?.nickname || '');
      setHtmlContents(team?.html_contents || '');
      return true;
    });
  };

  const permissions = new TeamGroupPermissions(team?.permissions);

  // Computed values
  // const deleteTeamPermission = permissions.has(TeamGroupPermission.TeamAdmin);

  // Event handlers
  const updateContents = (evt: React.FormEvent<HTMLTextAreaElement>) => {
    setHtmlContents(evt.currentTarget.value);
  };

  const updateNickname = (evt: React.FormEvent<HTMLInputElement>) => {
    setNickname(evt.currentTarget.value);
  };

  const updateProfileUrl = (url: string) => {
    setProfileUrl(url);
  };

  // eslint-disable-next-line unused-imports/no-unused-vars
  const handleSaveDao = () => {};
  // const handleSave = async (nextDaoAddress?: string) => {
  //   if (!team) return;

  //   if (validateInput) {
  //     showErrorToast(i18n.remove_test_keyword);
  //     return;
  //   }

  //   const mergedDaoAddress =
  //     nextDaoAddress ?? daoAddress ?? team?.dao_address ?? undefined;

  //   try {
  //     await teamsV3Api.updateTeam(teamDetailQuery.data.id, {
  //       nickname: nickname || undefined,
  //       description: htmlContents || undefined,
  //       profile_url: profileUrl || undefined,
  //       dao_address: mergedDaoAddress,
  //     });

  //     teamDetailQuery.refetch();

  //     if (team) {
  //       updateSelectedTeam({
  //         ...team,
  //         nickname: nickname!,
  //         html_contents: htmlContents!,
  //         profile_url: profileUrl,
  //         dao_address: mergedDaoAddress ?? null,
  //       });
  //     }

  //     setDaoAddress(mergedDaoAddress ?? null);
  //   } catch (e) {
  //     logger.error('Failed to update team:', e);
  //     showErrorToast(i18n.failed_update_team);
  //   }
  // };
  const openTeamDeletePopup = () => {};
  const openDaoCreatePopup = () => {};
  // const openDeletePopup = () => {
  //   popup
  //     .open(
  //       <DeleteTeamPopup
  //         onConfirm={async () => {
  //           if (!team) return;

  //           try {
  //             await teamsV3Api.deleteTeam(username);
  //             showInfoToast(i18n.success_delete_team);

  //             // Invalidate all team-related queries
  //             await queryClient.invalidateQueries({
  //               predicate: (query) =>
  //                 query.queryKey[0]?.toString().includes('team') ||
  //                 query.queryKey[0]?.toString().includes('user-info'),
  //             });

  //             // Invalidate all published feeds after deleting team
  //             await queryClient.invalidateQueries({
  //               queryKey: feedKeys.list({
  //                 status: FeedStatus.Published,
  //               }),
  //             });

  //             userInfo.refetch();
  //             setSelectedTeam(0);
  //             navigate('/');
  //           } catch (e) {
  //             logger.error('failed to delete team with error: ', e);
  //             showErrorToast(i18n.failed_delete_team);
  //           } finally {
  //             popup.close();
  //           }
  //         }}
  //         onCancel={() => {
  //           popup.close();
  //         }}
  //         i18n={i18n}
  //       />,
  //     )
  //     .withTitle('');
  // };

  return {
    // Data
    team,
    permissions,

    // Form State
    profileUrl,
    nickname,
    htmlContents,

    isEditing,

    // Event handlers
    handleStartEdit,
    updateContents,
    updateNickname,
    updateProfileUrl,
    openTeamDeletePopup,
    openDaoCreatePopup,

    // i18n
    i18n,
  };
}
