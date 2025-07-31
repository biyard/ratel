import { useDeliberationSpaceContext } from './deliberation/provider.client';
import { usePollSpaceContext } from './poll/provider.client';

export type PollContextType = ReturnType<typeof usePollSpaceContext>;
export type DeliberationContextType = ReturnType<
  typeof useDeliberationSpaceContext
>;

export type SpaceContextType = PollContextType | DeliberationContextType;
