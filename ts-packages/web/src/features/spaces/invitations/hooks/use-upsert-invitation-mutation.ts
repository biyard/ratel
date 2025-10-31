import { spaceKeys } from '@/constants';
import { upsertSpaceInvitation } from '@/lib/api/ratel/invitations.spaces.v3';
import { useMutation } from '@tanstack/react-query';
import { ListInvitationMemberResponse } from '../types/list-invitation-member-response';
import { optimisticUpdate } from '@/lib/hook-utils';

type Vars = {
  spacePk: string;
  user_pks: string[];
};

export function useUpsertInvitationMutation<
  T extends ListInvitationMemberResponse,
>() {
  return useMutation({
    mutationKey: ['upsert-invitation'],
    mutationFn: async (v: Vars) => {
      const { spacePk, user_pks } = v;

      await upsertSpaceInvitation(spacePk, user_pks);
      return v;
    },

    onSuccess: async (_, { spacePk }) => {
      const invitationQk = spaceKeys.invitations(spacePk);
      await optimisticUpdate<T>({ queryKey: invitationQk }, (response) => {
        return response;
      });
    },
  });
}
