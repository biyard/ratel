import { useState } from 'react';
import { State } from '@/types/state';

export class SpacePollCreatorController {
  constructor(
    /* public data: ReturnType<typeof useSpacePollCreatorData>, */
    public state: State<boolean>,
  ) {}
}

export function useSpacePollCreatorController() {
  // TODO: use or define hooks
  /* const data = useSpacePollCreatorData(); */
  const state = useState(false);

  return new SpacePollCreatorController(new State(state));
}
