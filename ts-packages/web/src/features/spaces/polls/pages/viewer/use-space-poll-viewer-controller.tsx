import { useState } from 'react';
import { State } from '@/types/state';

export class SpacePollViewerController {
  constructor(
    /* public data: ReturnType<typeof useSpacePollViewerData>, */
    public state: State<boolean>,
  ) {}
}

export function useSpacePollViewerController() {
  // TODO: use or define hooks
  const state = useState(false);

  return new SpacePollViewerController(new State(state));
}
