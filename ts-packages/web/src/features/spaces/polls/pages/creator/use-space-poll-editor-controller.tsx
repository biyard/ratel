import { useState } from 'react';
/* import { useSpacePollEditorData } from './use-space-poll-editor-data'; */
import { State } from '@/types/state';
import usePollSpace from '../../hooks/use-poll-space';
import { Poll } from '../../types/poll';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpacePollEditorController {
  constructor(
    public space: Space,
    public poll: Poll,
    public state: State<boolean>,
  ) {}

  handleAddQuestion = async () => {};

  handleUpdateQuestion = async () => {};

  handleRemoveQuestion = async () => {};
}

export function useSpacePollEditorController(spacePk: string, pollPk: string) {
  // TODO: use or define hooks
  /* const data = useSpacePollEditorData(); */
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const state = useState(false);

  return new SpacePollEditorController(space, poll, new State(state));
}
