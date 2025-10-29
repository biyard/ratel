import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpacePanelResponse } from '../types/space-panel-response';

export function getOption(spacePk: string, panelPk: string) {
  return {
    queryKey: spaceKeys.panel(spacePk, panelPk),
    queryFn: async () => {
      const discussion = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels/${encodeURIComponent(panelPk)}
        `,
      );
      return new SpacePanelResponse(discussion);
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePanel(
  spacePk: string,
  discussionPk: string,
): UseSuspenseQueryResult<SpacePanelResponse> {
  const query = useSuspenseQuery(getOption(spacePk, discussionPk));
  return query;
}
