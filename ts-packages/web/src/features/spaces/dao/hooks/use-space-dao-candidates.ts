import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type SpaceDaoCandidateResponse = {
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
};

export type SpaceDaoCandidateListResponse = {
  dao_address: string | null;
  candidates: SpaceDaoCandidateResponse[];
};

export function useSpaceDaoCandidates(
  spacePk: string,
  enabled = true,
): UseQueryResult<SpaceDaoCandidateListResponse> {
  return useQuery({
    queryKey: spaceDaoKeys.candidates(spacePk),
    queryFn: async () =>
      call<void, SpaceDaoCandidateListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/candidates`,
      ),
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
  });
}
