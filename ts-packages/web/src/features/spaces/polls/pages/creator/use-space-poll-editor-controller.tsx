import { useState } from 'react';
/* import { useSpacePollEditorData } from './use-space-poll-editor-data'; */
import { State } from '@/types/state';

export class SpacePollEditorController {
  constructor(
    /* public data: ReturnType<typeof useSpacePollEditorData>, */
    public state: State<boolean>,
  ) {}
}

export function useSpacePollEditorController() {
  // TODO: use or define hooks
  /* const data = useSpacePollEditorData(); */
  const state = useState(false);

  return new SpacePollEditorController(new State(state));
}
