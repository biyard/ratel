import { useState } from 'react';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useQueryClient } from '@tanstack/react-query';
import { TeamGroup } from '@/features/teams/types/team_group';
import { Group } from '@/lib/api/models/user';
import { useInviteMemberData, FoundUser } from './use-invite-member-data';

function convertToGroup(teamGroup: TeamGroup): Group {
  // groupPk is now just the UUID (not TEAM_GROUP#uuid format)
  const groupPk = teamGroup.id;
  return {
    id: parseInt(groupPk.replace(/\D/g, ''), 10) || 0,
    name: teamGroup.name,
    description: teamGroup.description,
    image_url: '',
    permissions: teamGroup.permissions,
    created_at: Date.now(),
    updated_at: Date.now(),
    creator_id: 0, // Not available in v3 API
    member_count: teamGroup.members,
  };
}

export class InviteMemberPopupController {
  constructor(
    public groupIndex: number,
    public selectedGroup: Group,
    public selectedUsers: FoundUser[],
    public searchValue: string,
    public isSearching: boolean,
    public isSubmitting: boolean,
    public convertedGroups: Group[],
    public handleSearchUser: (input: string) => Promise<void>,
    public handleRemoveUser: (userPk: string) => void,
    public handleGroupChange: (index: number, group: Group) => void,
    public handleSearchValueChange: (value: string) => void,
    public handleInvite: () => Promise<void>,
  ) {}
}

export function useInviteMemberPopupController({
  teamPk,
  username,
  groups,
  onClose,
}: {
  teamPk: string;
  username: string;
  groups: TeamGroup[];
  onClose: () => void;
}): InviteMemberPopupController {
  const queryClient = useQueryClient();
  const data = useInviteMemberData();

  // Keep both original and converted groups to maintain reference
  const convertedGroups = groups.map(convertToGroup);
  const [groupIndex, setGroupIndex] = useState(0);
  const [selectedGroup, setSelectedGroup] = useState(convertedGroups[0]);

  const [selectedUsers, setSelectedUsers] = useState<FoundUser[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [isSearching, setIsSearching] = useState(false);

  const handleSearchUser = async (input: string) => {
    if (!input.trim()) return;

    const identifiers = input
      .split(',')
      .map((v) => v.trim())
      .filter((v) => v !== '');

    for (const identifier of identifiers) {
      setIsSearching(true);
      const foundUser = await data.searchUser(identifier);
      setIsSearching(false);

      if (foundUser) {
        const exists = selectedUsers.some((u) => u.pk === foundUser.pk);

        if (!exists) {
          setSelectedUsers((prev) => [...prev, foundUser]);
        } else {
          showErrorToast(`${foundUser.nickname} is already added`);
        }
      }
    }

    setSearchValue('');
  };

  const handleRemoveUser = (userPk: string) => {
    setSelectedUsers((prevUsers) => prevUsers.filter((u) => u.pk !== userPk));
  };

  const handleGroupChange = (index: number, group: Group) => {
    setGroupIndex(index);
    setSelectedGroup(group);
  };

  const handleSearchValueChange = (value: string) => {
    setSearchValue(value);
  };

  const handleInvite = async () => {
    if (selectedUsers.length === 0) {
      showErrorToast('Please select users to invite');
      return;
    }

    try {
      // Get original group id from groups array using the index
      const originalGroup = groups[groupIndex];
      const groupPk = originalGroup.id; // Already just the UUID

      const result = await data.addMembersToGroup(
        teamPk,
        groupPk,
        selectedUsers.map((u) => u.pk),
      );

      if (result.failed_pks.length > 0) {
        showErrorToast(`Failed to add ${result.failed_pks.length} user(s)`);
      }

      if (result.total_added > 0) {
        showSuccessToast('Members invited successfully');

        // Invalidate queries to refresh member counts
        await queryClient.invalidateQueries({
          predicate: (query) => {
            const queryKey = query.queryKey;
            return (
              queryKey.includes(username) ||
              queryKey.includes('team') ||
              queryKey.includes('group') ||
              queryKey.includes('members')
            );
          },
        });

        // Close the popup after successful invite
        onClose();
      }
    } catch (err) {
      logger.error('Failed to invite members:', err);
      showErrorToast('Failed to invite members');
    }
  };

  return new InviteMemberPopupController(
    groupIndex,
    selectedGroup,
    selectedUsers,
    searchValue,
    isSearching,
    data.isAdding,
    convertedGroups,
    handleSearchUser,
    handleRemoveUser,
    handleGroupChange,
    handleSearchValueChange,
    handleInvite,
  );
}
