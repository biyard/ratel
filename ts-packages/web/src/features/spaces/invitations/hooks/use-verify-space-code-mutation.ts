import { spaceKeys } from '@/constants';
import { verifySpaceCode } from '@/lib/api/ratel/invitations.spaces.v3';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { optimisticUpdate } from '@/lib/hook-utils';
import { ListInvitationMemberResponse } from '../types/list-invitation-member-response';

type Vars = {
  spacePk: string;
  code: string;
};

export function useVerifySpaceCodeMutation<
  T extends ListInvitationMemberResponse,
>() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: ['verify-space-code'],
    mutationFn: async (v: Vars) => {
      const { spacePk, code } = v;

      await verifySpaceCode(spacePk, code);
      return v;
    },

    onSuccess: async (_, { spacePk }) => {
      const spaceQk = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQk }, (response) => {
        return response;
      });

      queryClient.invalidateQueries({ queryKey: spaceQk });
    },
  });
}
