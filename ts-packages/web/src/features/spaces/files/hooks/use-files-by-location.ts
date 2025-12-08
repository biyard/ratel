import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { FileResponse } from '../types/file-response';
import { FileLocation } from '../types/file';

export function getFilesByLocationOption(
  spacePk: string,
  location?: FileLocation,
) {
  const queryKey = location
    ? [...spaceKeys.files(spacePk), location]
    : spaceKeys.files(spacePk);

  return {
    queryKey,
    queryFn: async () => {
      const url = location
        ? `/v3/spaces/${encodeURIComponent(spacePk)}/files?location=${location}`
        : `/v3/spaces/${encodeURIComponent(spacePk)}/files`;

      const file = await call('GET', url);
      return new FileResponse(file);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useFilesByLocation(
  spacePk: string,
  location?: FileLocation,
): UseSuspenseQueryResult<FileResponse> {
  const query = useSuspenseQuery(getFilesByLocationOption(spacePk, location));
  return query;
}
