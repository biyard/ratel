import { QK_GET_ARTWORK_CERTIFICATE } from '@/constants';

import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';

import { ArtworkCertificate } from '@/lib/api/models/artwork';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export async function getArtworkCertificate(artworkId: number) {
  return apiFetch<ArtworkCertificate | null>(
    `${config.api_url}${ratelApi.dagit.getArtworkCertificate(artworkId)}`,
    {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    },
  );
}

export const getQueryKey = (artworkId: number) => [
  QK_GET_ARTWORK_CERTIFICATE,
  artworkId,
];

export function getOption(artworkId: number) {
  return {
    queryKey: getQueryKey(artworkId),
    queryFn: async () => {
      const { data } = await getArtworkCertificate(artworkId);
      if (!data) {
        return null;
      }
      return data;
    },
  };
}

export function useArtworkCertificateById(
  artworkId: number,
  enabled = true,
): UseQueryResult<ArtworkCertificate | null> {
  const query = useQuery({ ...getOption(artworkId), enabled });
  return query;
}
