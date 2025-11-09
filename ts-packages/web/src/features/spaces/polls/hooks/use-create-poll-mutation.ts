import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';

export function useCreatePollSpaceMutation() {
  const qc = useQueryClient();

  return useMutation<Poll, Error, { spacePk: string; default: boolean }>({
    mutationFn: ({
      spacePk,
      default: defaultValue,
    }: {
      spacePk: string;
      default: boolean;
    }) =>
      call<{ default: boolean }, Poll>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls`,
        { default: defaultValue },
      ),
    onSuccess: (_data, variables) => {
      const qk = spaceKeys.polls(variables.spacePk);
      qc.invalidateQueries({ queryKey: qk });
    },
  });
}
