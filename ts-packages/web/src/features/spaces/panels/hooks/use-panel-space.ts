import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListPanelResponse } from '../types/list-panel-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.panels(spacePk),
    queryFn: async () => {
      const panel = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels`,
      );
      return new ListPanelResponse(panel);
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePanelSpace(
  spacePk: string,
): UseSuspenseQueryResult<ListPanelResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
