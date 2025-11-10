import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

export type CheckPrerequisitesResponse = {
  completed: boolean;
  prerequisite_type?: string;
  poll_pk?: string;
  message?: string;
};

export function useCheckPrerequisites(spacePk: string) {
  return useQuery({
    queryKey: spaceKeys.prerequisites(spacePk),
    queryFn: () =>
      call<never, CheckPrerequisitesResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/prerequisites`,
      ),
    enabled: !!spacePk,
    staleTime: 0, // Always check fresh status
    gcTime: 0, // Don't cache this
  });
}
