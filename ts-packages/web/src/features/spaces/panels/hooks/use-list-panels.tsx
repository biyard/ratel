import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { SpacePanel } from '../types/space-panel';

export function useListPanels(
  spacePk: string,
): UseSuspenseQueryResult<SpacePanel[]> {
  return useSuspenseQuery({
    queryKey: spaceKeys.panels(spacePk),
    queryFn: async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const res: any = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels`,
      );

      const panels = res.items.map((e) => new SpacePanel(e));

      return panels;
    },
  });
}
