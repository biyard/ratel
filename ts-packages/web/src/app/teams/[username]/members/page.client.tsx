import { checkString } from '@/lib/string-filter-utils';
import { X } from 'lucide-react';
import { useQueryClient } from '@tanstack/react-query';
import { showSuccessToast, showErrorToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSuspenseTeamMembers } from '@/features/teams/hooks/use-team-members';
import { useRemoveGroupMember } from '@/features/teams/hooks/use-remove-group-member';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';

export default function TeamMembers({ username }: { username: string }) {
  const { t } = useTranslation('Team');
  const { data: team } = useSuspenseFindTeam(username);
  const { data: teamMembers } = useSuspenseTeamMembers(username); // Pass username directly
  const queryClient = useQueryClient();
  const [removingMember, setRemovingMember] = useState<string | null>(null);

  const removeGroupMemberMutation = useRemoveGroupMember().mutateAsync;

  const members =
    teamMembers?.items?.filter(
      (member) =>
        member !== undefined &&
        !(checkString(member.display_name) || checkString(member.username)),
    ) ?? [];

  const handleRemoveFromGroup = async (
    memberUserId: string,
    groupPk: string,
    groupName: string,
  ) => {
    const key = `${memberUserId}-${groupPk}`;
    if (removingMember === key) return; // Prevent double clicks

    setRemovingMember(key);
    try {
      // Use team PK (with TEAM# prefix) directly - no need to extract UUID
      await removeGroupMemberMutation({
        teamPk: team.pk,
        groupPk,
        request: {
          user_pks: [memberUserId],
        },
      });

      showSuccessToast(t('member_removed_from_group', { groupName }));

      // Invalidate team members query to refresh the list
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

  return (
    <div
      className="flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-card-bg border border-card-border rounded-lg h-fit"
      data-pw="team-members-list"
    >
      {members.map((member) => (
        <div
          key={member.user_id}
          data-pw={`member-item-${member.user_id}`}
          className="flex flex-col w-full h-fit gap-[15px] bg-transparent rounded-sm border border-card-border p-5"
        >
          <div
            key={member.user_id}
            className="flex flex-row w-full h-fit gap-[15px] bg-transparent"
          >
            {!member.profile_url ||
            member.profile_url.includes('test') ||
            member.profile_url === '' ? (
              <div className="w-12 h-12 rounded-full bg-profile-bg" />
            ) : (
              <img
                src={member.profile_url}
                alt={member.username}
                width={48}
                height={48}
                className="rounded-lg object-cover w-12 h-12"
              />
            )}

            <div className="flex flex-col justify-between items-start flex-1 min-w-0">
              <div className="font-bold text-text-primary text-base/[20px]">
                {member.username}
              </div>
              <div className="font-semibold text-desc-text text-sm/[20px]">
                {member.display_name}
              </div>
              {member.is_owner && (
                <div className="text-xs text-blue-500 font-medium">
                  {t('team_owner')}
                </div>
              )}
            </div>
          </div>

          <div className="flex flex-wrap w-full justify-start items-center gap-[10px]">
            {member.groups
              .filter((group) => !checkString(group.group_name))
              .map((group) => {
                const isRemoving =
                  removingMember === `${member.user_id}-${group.group_id}`;
                return (
                  <div
                    key={group.group_id}
                    data-pw={`member-group-${member.user_id}-${group.group_id}`}
                    className="flex flex-row w-fit h-fit items-center gap-1 px-[8px] py-[4px] border border-neutral-800 bg-black light:bg-neutral-600 light:border-transparent rounded-lg font-medium text-sm text-white"
                  >
                    <span>{group.group_name}</span>
                    {!member.is_owner && (
                      <button
                        data-pw={`remove-member-from-group-${member.user_id}-${group.group_id}`}
                        onClick={() =>
                          handleRemoveFromGroup(
                            member.user_id,
                            group.group_id,
                            group.group_name,
                          )
                        }
                        disabled={isRemoving}
                        className="ml-1 hover:bg-neutral-700 rounded-full p-0.5 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        title={t('remove_from_group')}
                      >
                        <X className="w-3 h-3" />
                      </button>
                    )}
                  </div>
                );
              })}
          </div>
        </div>
      ))}
    </div>
  );
}
