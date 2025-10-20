'use client';
import { Edit1, Extra, User } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import CreateGroupPopup from './_components/create-group-popup';
import { GroupPermission } from '@/lib/api/models/group';
import { logger } from '@/lib/logger';
import type { TeamGroupResponse } from '@/lib/api/ratel/teams.v3';
import InviteMemberPopup from './_components/invite-member-popup';
import { useTeamDetailByUsername } from '@/features/teams/hooks/use-team';
import { Folder } from 'lucide-react';
import { checkString } from '@/lib/string-filter-utils';
import { useTranslation } from 'react-i18next';
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import { useTeamPermissionsFromDetail } from '@/features/teams/hooks/use-team';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';
import { useQueryClient } from '@tanstack/react-query';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export default function TeamGroups({ username }: { username: string }) {
  const { t } = useTranslation('Team');
  const teamDetailQuery = useTeamDetailByUsername(username);
  const popup = usePopup();
  const queryClient = useQueryClient();

  // Get permissions directly from team detail response (no API calls!)
  const permissions = useTeamPermissionsFromDetail(teamDetailQuery.data);

  if (teamDetailQuery.isLoading) {
    return <div className="flex justify-center p-8">Loading team...</div>;
  }

  if (teamDetailQuery.error) {
    return (
      <div className="flex justify-center p-8 text-red-500">
        Error loading team
      </div>
    );
  }

  const teamDetail = teamDetailQuery.data;
  const canCreateGroup =
    permissions?.has(TeamGroupPermission.TeamEdit) ?? false;
  const canDeleteGroup =
    permissions?.has(TeamGroupPermission.TeamEdit) ?? false;

  // Use v3 groups directly - no more legacy conversion
  const groups = teamDetail?.groups ?? [];

  const deleteGroup = async (groupId: string) => {
    if (!teamDetail) return;

    try {
      // groupId is now just the UUID (not TEAM_GROUP#uuid format)
      await teamsV3Api.deleteGroup(username, groupId);

      // Invalidate all team-related queries to ensure fresh data
      await queryClient.invalidateQueries({
        predicate: (query) => {
          const queryKey = query.queryKey;
          return (
            queryKey.includes(username) ||
            queryKey.includes('team') ||
            queryKey.includes('group')
          );
        },
      });

      // Also force refetch the current query
      await teamDetailQuery.refetch();
    } catch (error) {
      logger.error('Failed to delete group:', error);
    }
  };

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full justify-end items-end gap-[10px]">
        <InviteMemberButton
          data-pw="invite-member-button"
          onClick={() => {
            if (!teamDetail) return;

            popup
              .open(
                <InviteMemberPopup
                  teamId={teamDetail.id}
                  username={username}
                  groups={groups}
                />,
              )
              .withoutBackdropClose();
          }}
        />
        {canCreateGroup && (
          <CreateGroupButton
            data-pw="create-group-button"
            onClick={() => {
              if (!teamDetail) return;

              popup
                .open(
                  <CreateGroupPopup
                    onCreate={async (
                      profileUrl: string,
                      groupName: string,
                      groupDescription: string,
                      groupPermissions: GroupPermission[],
                    ) => {
                      try {
                        // Convert legacy GroupPermission to TeamGroupPermission array
                        const teamGroupPermissions = [];
                        for (const permission of groupPermissions) {
                          // Map old permissions to new TeamGroupPermission values
                          // This is a simplification - you may need to adjust based on actual mappings
                          if (permission === GroupPermission.WritePosts) {
                            teamGroupPermissions.push(1); // TeamGroupPermission.PostWrite
                          }
                          if (permission === GroupPermission.DeletePosts) {
                            teamGroupPermissions.push(2); // TeamGroupPermission.PostDelete
                          }
                        }

                        await teamsV3Api.createGroup(teamDetail.id, {
                          name: groupName,
                          description: groupDescription,
                          image_url: profileUrl,
                          permissions: teamGroupPermissions,
                        });

                        // Invalidate all team-related queries to ensure fresh data
                        await queryClient.invalidateQueries({
                          predicate: (query) => {
                            const queryKey = query.queryKey;
                            return (
                              queryKey.includes(username) ||
                              queryKey.includes('team') ||
                              queryKey.includes('group')
                            );
                          },
                        });

                        // Also force refetch the current query
                        await teamDetailQuery.refetch();

                        popup.close();
                      } catch (err) {
                        logger.error('request failed with error: ', err);
                      }
                    }}
                  />,
                )
                .withTitle(t('create_group'));
            }}
          />
        )}
      </div>

      <ListGroups
        groups={groups ?? []}
        permission={canDeleteGroup}
        deleteGroup={deleteGroup}
      />
    </div>
  );
}

function ListGroups({
  groups,
  permission,
  deleteGroup,
}: {
  groups: TeamGroupResponse[];
  permission: boolean;
  deleteGroup: (groupSk: string) => void;
}) {
  const { t } = useTranslation('Team');
  return (
    <div className="flex flex-col w-full px-4 py-5 gap-[10px] bg-component-bg rounded-lg">
      {groups
        .filter((d) => !checkString(d.name))
        .map((group) => (
          <div
            key={group.id}
            data-pw={`group-item-${group.id}`}
            className="flex flex-row w-full h-fit justify-between items-center bg-transparent rounded-sm border border-card-enable-border p-5"
          >
            <div className="flex flex-row w-fit gap-[15px]">
              <Folder className="w-12 h-12 stroke-neutral-400" />

              <div className="flex flex-col justify-between items-start">
                <div className="font-bold text-text-primary text-base/[20px]">
                  {group.name}
                </div>
                <div className="font-semibold text-desc-text text-sm/[20px]">
                  {group.members} {t('member')}
                </div>
              </div>
            </div>

            {permission ? (
              <DropdownMenu modal={false}>
                <DropdownMenuTrigger asChild>
                  <button
                    data-pw={`group-options-${group.id}`}
                    className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
                    aria-haspopup="true"
                    aria-label="Post options"
                  >
                    <Extra className="size-6 text-gray-400" />
                  </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent
                  align="end"
                  className="w-40 border-gray-700 transition ease-out duration-100"
                >
                  <DropdownMenuItem>
                    <button
                      data-pw={`delete-group-${group.id}`}
                      onClick={() => {
                        deleteGroup(group.id);
                      }}
                      className="flex items-center w-full px-4 py-2 text-sm text-text-primary hover:bg-hover cursor-pointer"
                    >
                      <div>{t('delete_group')}</div>
                    </button>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            ) : (
              <></>
            )}
          </div>
        ))}
    </div>
  );
}

function InviteMemberButton({
  onClick,
  'data-pw': dataPw,
}: {
  onClick: () => void;
  'data-pw'?: string;
}) {
  const { t } = useTranslation('Team');
  return (
    <div
      data-pw={dataPw}
      className="cursor-pointer flex flex-row w-fit justify-start items-center px-4 py-3 bg-white border border-foreground rounded-[100px] gap-1"
      onClick={onClick}
    >
      <User className="w-4 h-4" />
      <div className="font-bold text-base/[22px] text-neutral-900 light:text-black">
        {t('invite_member')}
      </div>
    </div>
  );
}

function CreateGroupButton({
  onClick,
  'data-pw': dataPw,
}: {
  onClick: () => void;
  'data-pw'?: string;
}) {
  const { t } = useTranslation('Team');
  return (
    <div
      data-pw={dataPw}
      className="cursor-pointer flex flex-row w-fit justify-start items-center px-4 py-3 bg-white border border-foreground rounded-[100px] gap-1"
      onClick={onClick}
    >
      <Edit1 className="w-4 h-4" />
      <div className="font-bold text-base/[22px] text-neutral-900 light:text-black">
        {t('create_group')}
      </div>
    </div>
  );
}
