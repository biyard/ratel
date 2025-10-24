import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { FileResponse } from '../types/file-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.files(spacePk),
    queryFn: async () => {
      const file = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files`,
      );
      return new FileResponse(file);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useFileSpace(
  spacePk: string,
): UseSuspenseQueryResult<FileResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
