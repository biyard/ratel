import { Edit1, Extra, User } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import CreateGroupPopup from './_components/create-group-popup';
import { logger } from '@/lib/logger';
import InviteMemberPopup from './_components/invite-member-popup';
import { Folder } from 'lucide-react';
import { checkString } from '@/lib/string-filter-utils';
import { useTranslation } from 'react-i18next';
import {
  TeamGroupPermission,
  TeamGroupPermissions,
} from '@/features/auth/utils/team-group-permissions';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useDeleteGroup } from '@/features/teams/hooks/use-delete-group';
import { TeamGroup } from '@/features/teams/types/team_group';
import { useSuspenseTeamGroups } from '@/features/teams/hooks/use-team-groups';
import { useCreateGroup } from '@/features/teams/hooks/use-create-group';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';

export default function TeamGroups({ username }: { username: string }) {
  const { t } = useTranslation('Team');
  const { data: team } = useSuspenseFindTeam(username);
  const { data: groups } = useSuspenseTeamGroups(username);
  const popup = usePopup();

  const deleteGroupMutation = useDeleteGroup().mutateAsync;
  const createGroupMutation = useCreateGroup().mutateAsync;

  // Get permissions directly from team detail response (no API calls!)
  const permissions = new TeamGroupPermissions(team?.permissions);

  const canEditGroup = permissions?.has(TeamGroupPermission.GroupEdit) ?? false;

  const canEditTeam = permissions?.has(TeamGroupPermission.TeamEdit) ?? false;

  const deleteGroup = async (groupPk: string) => {
    if (!team) return;

    try {
      // groupPk is now just the UUID (not TEAM_GROUP#uuid format)
      await deleteGroupMutation({
        teamUsername: username,
        groupPk,
      });
    } catch (error) {
      logger.error('Failed to delete group:', error);
    }
  };

  const handleInviteMember = () => {
    if (!team || !canEditGroup) return;
    popup
      .open(
        <InviteMemberPopup
          teamPk={team.pk}
          username={username}
          groups={groups.items}
          onClose={() => popup.close()}
        />,
      )
      .withoutBackdropClose();
  };

  /*
  export interface CreateGroupRequest {
  name: string;
  description: string;
  image_url: string;
  permissions: number[]; // TeamGroupPermission values
}
  */
  const handleCreateGroup = () => {
    if (!team || !canEditGroup) return;
    popup
      .open(
        <CreateGroupPopup
          onCreate={(
            profileUrl,
            groupName,
            groupDescription,
            groupPermissions,
          ) =>
            createGroupMutation({
              teamPk: team.pk,
              request: {
                name: groupName,
                description: groupDescription,
                image_url: profileUrl,
                permissions: groupPermissions,
              },
            })
          }
        />,
      )
      .withTitle(t('create_group'));
  };

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full justify-end items-end gap-2.5">
        <InviteMemberButton
          data-pw="invite-member-button"
          onClick={handleInviteMember}
        />
        {canEditTeam && (
          <CreateGroupButton
            data-pw="create-group-button"
            onClick={handleCreateGroup}
          />
        )}
      </div>

      <ListGroups
        groups={groups.items ?? []}
        permission={canEditTeam}
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
  groups: TeamGroup[];
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
      data-testid={dataPw}
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
