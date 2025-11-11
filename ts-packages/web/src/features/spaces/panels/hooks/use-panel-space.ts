import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpacePanelResponse } from '../types/space-panel-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.panels(spacePk),
    queryFn: async () => {
      const panel = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels`,
      );
      return new SpacePanelResponse(panel);
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePanelSpace(
  spacePk: string,
): UseSuspenseQueryResult<SpacePanelResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
