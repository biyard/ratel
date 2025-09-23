'use client';
import { Edit1, Extra } from '@/components/icons';
import { User } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import CreateGroupPopup from './_components/create-group-popup';
import {
  createGroupRequest,
  deleteGroupRequest,
  GroupPermission,
  inviteMemberRequest,
} from '@/lib/api/models/group';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { logger } from '@/lib/logger';
import { Group } from '@/lib/api/models/user';
import InviteMemberPopup from './_components/invite-member-popup';
import { useTeamByUsername } from '../../_hooks/use-team';
import { Folder } from 'lucide-react';
import { checkString } from '@/lib/string-filter-utils';
import { useTranslations } from 'next-intl';
import { usePermission } from '@/app/(social)/_hooks/use-permission';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export default function TeamGroups({ username }: { username: string }) {
  const t = useTranslations('Team');
  const query = useTeamByUsername(username);
  const popup = usePopup();
  const { post } = useApiCall();

  const groups: Group[] = (query.data?.groups ?? [])
    .flat()
    .filter((g): g is Group => g !== undefined);

  const team = query.data;

  const inviteMemberPermission =
    usePermission(team?.id ?? 0, GroupPermission.InviteMember).data
      .has_permission ?? false;

  const updateGroupPermission =
    usePermission(team?.id ?? 0, GroupPermission.UpdateGroup).data
      .has_permission ?? false;

  const deleteGroupPermission =
    usePermission(team?.id ?? 0, GroupPermission.DeleteGroup).data
      .has_permission ?? false;

  const deleteGroup = async (groupId: number) => {
    await post(
      ratelApi.groups.delete_group(team.id, groupId),
      deleteGroupRequest(),
    );

    query.refetch();
  };

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full justify-end items-end gap-[10px]">
        {groups && groups.length != 0 && inviteMemberPermission && (
          <InviteMemberButton
            onClick={() => {
              popup
                .open(
                  <InviteMemberPopup
                    team_id={team.id}
                    groups={groups}
                    onclick={async (group_id, users) => {
                      try {
                        await post(
                          ratelApi.groups.invite_member(team.id, group_id),
                          inviteMemberRequest(users),
                        );
                        query.refetch();
                        popup.close();
                      } catch (err) {
                        logger.error('request failed with error: ', err);
                      }
                    }}
                  />,
                )
                .withTitle(t('invite_member'));
            }}
          />
        )}
        {updateGroupPermission && (
          <CreateGroupButton
            onClick={() => {
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
                        await post(
                          ratelApi.groups.create_group(team.id),
                          createGroupRequest(
                            groupName,
                            groupDescription,
                            profileUrl,
                            [],
                            groupPermissions,
                          ),
                        );

                        query.refetch();

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
        permission={deleteGroupPermission}
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
  groups: Group[];
  permission: boolean;
  deleteGroup: (groupId: number) => void;
}) {
  const t = useTranslations('Team');
  return (
    <div className="flex flex-col w-full px-4 py-5 gap-[10px] bg-component-bg rounded-lg">
      {groups
        .filter((d) => !checkString(d.name))
        .map((group) => (
          <div
            key={group.id}
            className="flex flex-row w-full h-fit justify-between items-center bg-transparent rounded-sm border border-card-enable-border p-5"
          >
            <div className="flex flex-row w-fit gap-[15px]">
              <Folder className="w-12 h-12 stroke-neutral-400" />

              <div className="flex flex-col justify-between items-start">
                <div className="font-bold text-text-primary text-base/[20px]">
                  {group.name}
                </div>
                <div className="font-semibold text-desc-text text-sm/[20px]">
                  {group.member_count} {t('member')}
                </div>
              </div>
            </div>

            {permission ? (
              <DropdownMenu modal={false}>
                <DropdownMenuTrigger asChild>
                  <button
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

function InviteMemberButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('Team');
  return (
    <div
      className="cursor-pointer flex flex-row w-fit justify-start items-center px-4 py-3 bg-primary rounded-[100px] gap-1"
      onClick={onClick}
    >
      <User className="w-4 h-4 [&>path]:stroke-[#000203]" />
      <div className="font-bold text-base/[22px] text-[#000203]">
        {t('invite_member')}
      </div>
    </div>
  );
}

function CreateGroupButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('Team');
  return (
    <div
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
