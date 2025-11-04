import { spaceKeys } from '@/constants';
import { resentVerificationCode } from '@/lib/api/ratel/invitations.spaces.v3';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type Vars = {
  spacePk: string;
  email: string;
};

export function useResentVerificationMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ['resent-verification'],
    mutationFn: async (v: Vars) => {
      const { spacePk, email } = v;
      await resentVerificationCode(spacePk, email);
      return v;
    },
    onSuccess: async (_, { spacePk }) => {
      const invitationQk = spaceKeys.invitations(spacePk);
      await queryClient.invalidateQueries({ queryKey: invitationQk });

      queryClient.invalidateQueries({ queryKey: invitationQk });
    },
  });
}
