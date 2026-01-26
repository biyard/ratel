import { useState } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { useMembersData } from './use-members-data';
import { useRemoveGroupMember } from '@/features/teams/hooks/use-remove-group-member';
import { showSuccessToast, showErrorToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { useTranslation } from 'react-i18next';

export function useMembersPageController(username: string) {
  const data = useMembersData(username);
  const queryClient = useQueryClient();
  const { t } = useTranslation('Team');
  const [removingMember, setRemovingMember] = useState<string | null>(null);

  const removeGroupMemberMutation = useRemoveGroupMember().mutateAsync;

  const handleRemoveFromGroup = async (
    memberUserId: string,
    groupPk: string,
    groupName: string,
  ) => {
    const key = `${memberUserId}-${groupPk}`;
    if (removingMember === key) return;
    setRemovingMember(key);
    try {
      await removeGroupMemberMutation({
        teamPk: data.team.pk,
        groupPk,
        request: {
          user_pks: [memberUserId],
        },
      });

      showSuccessToast(t('member_removed_from_group', { groupName }));

      queryClient.invalidateQueries({
        queryKey: ['team', 'members', username],
      });
    } catch (err) {
      logger.error('Failed to remove member from group:', err);
      showErrorToast(t('failed_remove_member_from_group'));
    } finally {
      setRemovingMember(null);
    }
  };

  return {
    ...data,
    handleRemoveFromGroup,
    removingMember,
  };
}
