import { spaceKeys } from '@/constants';
import { verifySpaceCode } from '@/lib/api/ratel/invitations.spaces.v3';
import { useMutation } from '@tanstack/react-query';
import { optimisticUpdate } from '@/lib/hook-utils';
import { ListInvitationMemberResponse } from '../types/list-invitation-member-response';

type Vars = {
  spacePk: string;
  code: string;
};

export function useVerifySpaceCodeMutation<
  T extends ListInvitationMemberResponse,
>() {
  return useMutation({
    mutationKey: ['verify-space-code'],
    mutationFn: async (v: Vars) => {
      const { spacePk, code } = v;

      await verifySpaceCode(spacePk, code);
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
