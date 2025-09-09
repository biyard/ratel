'use client';
import { Edit1 } from '@/components/icons';
import { User } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import CreateGroupPopup from './_components/create-group-popup';
import {
  createGroupRequest,
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

export default function TeamGroups({ username }: { username: string }) {
  const t = useTranslations('Team');
  const query = useTeamByUsername(username);
  const popup = usePopup();
  const { post } = useApiCall();

  const groups: Group[] = (query.data?.groups ?? [])
    .flat()
    .filter((g): g is Group => g !== undefined);

  const team = query.data;

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full justify-end items-end gap-[10px]">
        {groups && groups.length != 0 && (
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
      </div>

      <ListGroups groups={groups ?? []} />
    </div>
  );
}

function ListGroups({ groups }: { groups: Group[] }) {
  const t = useTranslations('Team');
  return (
    <div className="flex flex-col w-full px-4 py-5 gap-[10px] bg-component-bg rounded-lg">
      {groups
        .filter((d) => !checkString(d.name))
        .map((group) => (
          <div
            key={group.id}
            className="flex flex-row w-full h-fit gap-[15px] bg-transparent rounded-sm border border-neutral-800 p-5"
          >
            <Folder className="w-12 h-12 stroke-neutral-400" />

            <div className="flex flex-col justify-between items-start">
              <div className="font-bold text-foreground text-base/[20px]">
                {group.name}
              </div>
              <div className="font-semibold text-neutral-400 text-sm/[20px]">
                {group.member_count} {t('member')}
              </div>
            </div>
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
