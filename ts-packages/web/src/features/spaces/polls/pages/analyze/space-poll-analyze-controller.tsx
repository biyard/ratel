import { Space } from '@/features/spaces/types/space';
import { Poll } from '../../types/poll';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePollSpace from '../../hooks/use-poll-space';
import usePollSpaceSummaries from '../../hooks/use-poll-space-summary';
import { PollSurveySummariesResponse } from '@/lib/api/ratel/poll.spaces.v3';

export class SpacePollAnalyzeController {
  constructor(
    public space: Space,
    public poll: Poll,
    public summary: PollSurveySummariesResponse,
  ) {}
}

export function useSpacePollAnalyzeController(spacePk: string, pollPk: string) {
  // Fetching data from remote
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: summary } = usePollSpaceSummaries(spacePk, pollPk);

  return new SpacePollAnalyzeController(space, poll, summary);
}
