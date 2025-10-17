import { useState } from 'react';
import { useSpaceSettingsData } from './use-space-settings-data';
import { State } from '@/types/state';

export class SpaceSettingsController {
  constructor(
    public data: ReturnType<typeof useSpaceSettingsData>,
    public state: State<boolean>,
  ) {}
}

export function useSpaceSettingsController(spacePk: string) {
  const data = useSpaceSettingsData(spacePk);
  const state = useState(false);

  return new SpaceSettingsController(data, new State(state));
}
