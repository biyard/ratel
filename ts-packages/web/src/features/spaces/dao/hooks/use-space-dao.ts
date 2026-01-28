import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call, RatelSdkError } from '@/lib/api/ratel/call';

export type SpaceDaoResponse = {
  contract_address: string;
  sampling_count: number;
  reward_amount: number;
  created_at?: number;
  updated_at?: number;
};

function isNotFound(error: unknown) {
  if (!error || typeof error !== 'object') return false;
  const message = (error as RatelSdkError).message as unknown;
  const text =
    typeof message === 'string'
      ? message
      : // eslint-disable-next-line @typescript-eslint/no-explicit-any
        typeof (message as any)?.message === 'string'
        ? // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (message as any).message
        : '';
  return text.toLowerCase().includes('not found');
}

export function useSpaceDao(
  spacePk: string,
): UseQueryResult<SpaceDaoResponse | null> {
  return useQuery({
    queryKey: ['space-dao', spacePk],
    queryFn: async () => {
      try {
        return await call<void, SpaceDaoResponse>(
          'GET',
          `/v3/spaces/${encodeURIComponent(spacePk)}/dao`,
        );
      } catch (error) {
        if (isNotFound(error)) {
          return null;
        }
        throw error;
      }
    },
    enabled: Boolean(spacePk),
    refetchOnWindowFocus: false,
  });
}
