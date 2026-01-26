import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import {
  findUserByEmail,
  findUserByPhoneNumber,
  findUserByUsername,
  UserDetailResponse,
} from '@/lib/api/ratel/users.v3';
import { useAddGroupMember } from '@/features/teams/hooks/use-add-group-member';

export interface FoundUser {
  pk: string;
  nickname: string;
  username: string;
  profile_url: string;
}

export interface InviteMemberData {
  searchUser: (input: string) => Promise<FoundUser | null>;
  addMembersToGroup: (
    teamPk: string,
    groupPk: string,
    userPks: string[],
  ) => Promise<{ total_added: number; failed_pks: string[] }>;
  isAdding: boolean;
  error: Error | null;
}

export function useInviteMemberData(): InviteMemberData {
  const addMemberMutation = useAddGroupMember();

  const searchUser = async (input: string): Promise<FoundUser | null> => {
    if (checkString(input)) return null;

    const isEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input);
    const isPhone = /^\+?[0-9]\d{7,14}$/.test(input);

    try {
      let userResponse: UserDetailResponse;

      if (isEmail) {
        userResponse = await findUserByEmail(input);
      } else if (isPhone) {
        userResponse = await findUserByPhoneNumber(input);
      } else {
        userResponse = await findUserByUsername(input);
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
        showErrorToast('Failed to search user');
      }

      return null;
    }
  };

  const addMembersToGroup = async (
    teamPk: string,
    groupPk: string,
    userPks: string[],
  ) => {
    return await addMemberMutation.mutateAsync({
      teamPk,
      groupPk,
      request: { user_pks: userPks },
    });
  };

  return {
    searchUser,
    addMembersToGroup,
    isAdding: addMemberMutation.isPending,
    error: addMemberMutation.error,
  };
}
