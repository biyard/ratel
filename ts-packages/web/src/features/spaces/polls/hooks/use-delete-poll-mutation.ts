import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export type DeletePollResponse = {
  status: string;
};

export function useDeletePollSpaceMutation() {
  const qc = useQueryClient();

  return useMutation<
    DeletePollResponse,
    Error,
    { spacePk: string; pollSk: string }
  >({
    mutationFn: ({ spacePk, pollSk }: { spacePk: string; pollSk: string }) =>
      call<never, DeletePollResponse>(
        'DELETE',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollSk)}`,
      ),
    onSuccess: (_data, variables) => {
      const qk = spaceKeys.polls(variables.spacePk);
      qc.invalidateQueries({ queryKey: qk });
    },
  });
}
