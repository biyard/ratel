'use client';

import SelectBox from '@/components/selectbox/selectbox';
import { Group } from '@/lib/api/models/user';
import { useState } from 'react';
import { Clear } from '@/components/icons';
import SearchInput from '@/components/input/search-input';
import clsx from 'clsx';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useTranslation } from 'react-i18next';
import * as usersV3Api from '@/lib/api/ratel/users.v3';
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import type { TeamGroupResponse } from '@/lib/api/ratel/teams.v3';
import { useQueryClient } from '@tanstack/react-query';

interface FoundUser {
  pk: string;
  nickname: string;
  username: string;
  profile_url: string;
}

// Convert TeamGroupResponse to Group for UI compatibility
function convertToGroup(teamGroup: TeamGroupResponse): Group {
  // groupId is now just the UUID (not TEAM_GROUP#uuid format)
  const groupId = teamGroup.id;
  return {
    id: parseInt(groupId.replace(/\D/g, ''), 10) || 0,
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

export default function InviteMemberPopup({
  teamId,
  username,
  groups,
}: {
  teamId: string;
  username: string;
  groups: TeamGroupResponse[];
}) {
  const { t } = useTranslation('Team');
  const queryClient = useQueryClient();

  // Keep both original and converted groups to maintain sk reference
  const convertedGroups = groups.map(convertToGroup);
  const [groupIndex, setGroupIndex] = useState(0);
  const [selectedGroup, setSelectedGroup] = useState(convertedGroups[0]);

  const [selectedUsers, setSelectedUsers] = useState<FoundUser[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [isSearching, setIsSearching] = useState(false);

  const searchUser = async (input: string): Promise<FoundUser | null> => {
    if (checkString(input)) return null;

    const isEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input);
    const isPhone = /^\+?[0-9]\d{7,14}$/.test(input);

    try {
      setIsSearching(true);
      let userResponse;

      if (isEmail) {
        userResponse = await usersV3Api.findUserByEmail(input);
      } else if (isPhone) {
        userResponse = await usersV3Api.findUserByPhoneNumber(input);
      } else {
        userResponse = await usersV3Api.findUserByUsername(input);
      }

      logger.debug('User search response:', userResponse);

      // The response has user fields flattened at the root level (no nested "user" object)
      if (userResponse?.pk) {
        logger.debug('User found:', userResponse);
        return {
          pk: userResponse.pk,
          nickname: userResponse.nickname,
          username: userResponse.username,
          profile_url: userResponse.profile_url,
        };
      }

      logger.warn('User not found - invalid response structure:', userResponse);
      showErrorToast('User Not Found');
      return null;
    } catch (err: unknown) {
      logger.error('Failed to search user:', err);

      const error = err as { response?: { status?: number }; status?: number };
      if (error?.response?.status === 404 || error?.status === 404) {
        showErrorToast('User Not Found');
      } else {
        showErrorToast(t('failed_search_user'));
      }

      return null;
    } finally {
      setIsSearching(false);
    }
  };

  const addUser = async (value: string) => {
    if (!value.trim()) return;

    const identifiers = value
      .split(',')
      .map((v) => v.trim())
      .filter((v) => v !== '');

    for (const input of identifiers) {
      const foundUser = await searchUser(input);

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

  const handleInvite = async () => {
    if (selectedUsers.length === 0) {
      showErrorToast('Please select users to invite');
      return;
    }

    try {
      // Get original group id from groups array using the index
      const originalGroup = groups[groupIndex];
      const groupId = originalGroup.id; // Already just the UUID

      // Use team PK (with TEAM# prefix) directly - no need to extract UUID
      const result = await teamsV3Api.addGroupMember(teamId, groupId, {
        user_pks: selectedUsers.map((u) => u.pk),
      });

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
      }

      setSelectedUsers([]);
      setSearchValue('');
    } catch (err) {
      logger.error('Failed to invite members:', err);
      showErrorToast('Failed to invite members');
    }
  };

  return (
    <div className="flex flex-col w-[900px] min-h-[400px] max-w-[900px] min-w-[400px] max-mobile:!w-full max-mobile:!max-w-full gap-5">
      <div className="flex flex-col w-full gap-[10px]">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {t('select_group')}
        </div>
        <SelectBox
          groups={convertedGroups}
          groupIndex={groupIndex}
          setGroupIndex={setGroupIndex}
          selectedGroup={selectedGroup}
          setSelectedGroup={setSelectedGroup}
        />
      </div>

      <div className="flex flex-col w-full">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {t('email_label')}
        </div>
        <div className="mt-2.5">
          <SearchInput
            data-testid="invite-member-search-input"
            value={searchValue}
            placeholder={t('email_hint')}
            setValue={(value) => {
              setSearchValue(value);
            }}
            onenter={async () => {
              await addUser(searchValue);
            }}
          />
        </div>
        {isSearching && (
          <div className="text-sm text-gray-400 mt-2">Searching...</div>
        )}
      </div>

      <div className="flex flex-col w-full gap-[10px]">
        <div className="flex flex-wrap gap-1">
          {selectedUsers.map((user) => {
            return (
              <SelectedUserInfo
                key={user.pk}
                username={user.nickname}
                onremove={() => {
                  setSelectedUsers((prevUsers) =>
                    prevUsers.filter((u) => u.pk !== user.pk),
                  );
                }}
              />
            );
          })}
        </div>
      </div>

      <InviteMemberButton
        disabled={selectedUsers.length === 0}
        onclick={handleInvite}
      />
    </div>
  );
}

function InviteMemberButton({
  disabled,
  onclick,
}: {
  disabled: boolean;
  onclick: () => void;
}) {
  const { t } = useTranslation('Team');
  const containerClass = clsx(
    'flex flex-row w-full justify-center items-center my-[15px] py-[15px] rounded-lg font-bold text-[#000203] text-base',
    disabled
      ? 'cursor-not-allowed bg-neutral-500'
      : 'cursor-pointer bg-primary',
  );
  return (
    <div className="flex flex-col w-full">
      <div
        data-testid="send-invite-button"
        className={containerClass}
        onClick={() => {
          if (!disabled) {
            onclick();
          }
        }}
      >
        {t('send')}
      </div>
    </div>
  );
}

function SelectedUserInfo({
  username,
  onremove,
}: {
  username: string;
  onremove: () => void;
}) {
  return (
    <div className="flex flex-row w-fit gap-1 justify-start items-center bg-primary rounded-[100px] px-[12px] py-[2px]">
      <div className="font-medium text-neutral-900 text-[15px]/[24px]">
        {username}
      </div>
      <Clear
        width={24}
        height={24}
        className="w-6 h-6 cursor-pointer [&>path]:stroke-neutral-800"
        onClick={onremove}
      />
    </div>
  );
}
