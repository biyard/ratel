import { QK_GET_ARTWORK } from '@/constants';

import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';

import { ArtworkDetail } from '@/lib/api/models/artwork';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export async function getArtworkDetail(artworkId: number) {
  return apiFetch<ArtworkDetail | null>(
    `${config.api_url}${ratelApi.dagit.getArtworkById(artworkId)}`,
    {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    },
  );
}

export const getQueryKey = (artworkId: number) => [QK_GET_ARTWORK, artworkId];
export function getOption(artworkId: number) {
  return {
    queryKey: getQueryKey(artworkId),
    queryFn: async () => {
      const { data } = await getArtworkDetail(artworkId);
      if (!data) {
        return null;
      }
      return data;
    },
  };
}

export function useArtworkDetailById(
  artworkId: number,
  enabled = true,
): UseQueryResult<ArtworkDetail | null> {
  const query = useQuery({ ...getOption(artworkId), enabled });
  return query;
}
