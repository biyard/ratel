import { useState, useEffect } from 'react';
import {
  TeamGroupPermission,
  TeamGroupPermissions,
} from '@/features/auth/utils/team-group-permissions';
import { useTeamSettingsI18n } from '../i18n';
import { useSuspenseFindTeam } from '../../hooks/use-find-team';
import { useUpdateTeam } from '../../hooks/use-update-team';
import { useDeleteTeam } from '../../hooks/use-delete-team';
import { showSuccessToast, showErrorToast } from '@/lib/toast';
import { usePopup } from '@/lib/contexts/popup-service';
import DeleteTeamPopup from '../components/delete-team-popup';

export function useTeamSettingsPageController(username: string) {
  const i18n = useTeamSettingsI18n();
  const updateTeamMutation = useUpdateTeam();
  const deleteTeamMutation = useDeleteTeam();
  const popup = usePopup();

  // Fetch team data
  const { data: team } = useSuspenseFindTeam(username);

  const [isEditing, setIsEditing] = useState(false);
  // Form state
  const [profileUrl, setProfileUrl] = useState<string>('');
  const [nickname, setNickname] = useState<string>('');
  const [htmlContents, setHtmlContents] = useState<string>('');

  // Initialize form state from team data
  useEffect(() => {
    if (team) {
      setProfileUrl(team.profile_url || '');
      setNickname(team.nickname || '');
      setHtmlContents(team.html_contents || '');
    }
  }, [team]);

  const permissions = new TeamGroupPermissions(team?.permissions);

  // Computed values
  const deleteTeamPermission = permissions.has(TeamGroupPermission.TeamAdmin);
  const isSaving = updateTeamMutation.isPending;

  // Validation
  const validateForm = () => {
    const errors: string[] = [];

    if (!nickname || nickname.trim().length === 0) {
      errors.push(i18n.validation_nickname_required);
    }

    if (htmlContents && htmlContents.length > 0 && htmlContents.length < 10) {
      errors.push(i18n.validation_description_min_length);
    }

    return errors;
  };

  // Event handlers
  const handleEdit = () => {
    // Sync form state with current team data
    setProfileUrl(team?.profile_url || '');
    setNickname(team?.nickname || '');
    setHtmlContents(team?.html_contents || '');
    setIsEditing(true);
  };

  const handleSave = async () => {
    const errors = validateForm();
    if (errors.length > 0) {
      showErrorToast(errors.join(', '));
      return;
    }

    try {
      await updateTeamMutation.mutateAsync({
        teamPk: team.pk,
        request: {
          nickname: nickname || undefined,
          description: htmlContents || undefined,
          profile_url: profileUrl || undefined,
        },
      });
      showSuccessToast('Team settings updated successfully');
      setIsEditing(false);
    } catch (error) {
      console.error('Failed to update team:', error);
      showErrorToast(
        'Failed to update team: ' +
          (error instanceof Error ? error.message : 'Unknown error'),
      );
    }
  };

  const handleCancel = () => {
    // Reset form to original team values
    setProfileUrl(team?.profile_url || '');
    setNickname(team?.nickname || '');
    setHtmlContents(team?.html_contents || '');
    setIsEditing(false);
  };

  const updateContents = (evt: React.FormEvent<HTMLTextAreaElement>) => {
    setHtmlContents(evt.currentTarget.value);
  };

  const updateNickname = (evt: React.FormEvent<HTMLInputElement>) => {
    setNickname(evt.currentTarget.value);
  };

  const updateProfileUrl = (url: string) => {
    setProfileUrl(url);
  };

  const handleDeleteTeam = async () => {
    try {
      await deleteTeamMutation.mutateAsync(team.username);

      popup.close();
      showSuccessToast(i18n.success_delete_team);

      // Navigate to home page after successful deletion
      window.location.href = '/';
    } catch (error) {
      console.error('Failed to delete team:', error);
      popup.close();
      showErrorToast(
        i18n.failed_delete_team +
          ': ' +
          (error instanceof Error ? error.message : 'Unknown error'),
      );
    }
  };

  const openDeletePopup = () => {
    popup
      .open(
        <DeleteTeamPopup
          onConfirm={handleDeleteTeam}
          onCancel={() => popup.close()}
          i18n={i18n}
        />,
      )
      .withoutBackdropClose();
  };

  return {
    // Data
    team,
    permissions,

    // Form State
    profileUrl,
    nickname,
    htmlContents,

    isEditing,
    isSaving,
    deleteTeamPermission,

    // Event handlers
    handleEdit,
    handleSave,
    handleCancel,
    updateContents,
    updateNickname,
    updateProfileUrl,
    openDeletePopup,

    // i18n
    i18n,
  };
}
