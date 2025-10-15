import { useState } from 'react';
import { State } from '@/types/state';
import { BoosterType } from '../../types/booster-type';

export class SpaceSettingModalController {
  constructor(
    public timezone: State<string>,
    public startTimestamp: State<number>,
    public endTimestamp: State<number>,
    public activateBooster: State<boolean>,
    public boosterType: State<BoosterType>,
  ) {}

  handleStartTime = (timestamp: number) => {
    const delta = this.endTimestamp.get() - this.startTimestamp.get();
    this.startTimestamp.set(timestamp);
    this.endTimestamp.set(timestamp + delta);
  };

  handleEndTime = (timestamp: number) => {
    this.endTimestamp.set(timestamp);
  };
}

export function useSpaceSettingModalController() {
  const now = new Date();
  const hours = now.getHours();
  const startTime = new Date(now);
  startTime.setHours(hours + 1);
  startTime.setMinutes(0);
  startTime.setSeconds(0);
  startTime.setMilliseconds(0);

  const endTime = new Date(startTime.getTime() + 60 * 60 * 1000); // 1 hour later

  const timezone = useState<string>(
    Intl.DateTimeFormat().resolvedOptions().timeZone,
  );

  const activateBooster = useState<boolean>(false);

  const startTimestamp = useState<number>(startTime.getTime());
  const endTimestamp = useState<number>(endTime.getTime());
  const boosterType = useState<BoosterType>(BoosterType.NoBoost);

  return new SpaceSettingModalController(
    new State(timezone),
    new State(startTimestamp),
    new State(endTimestamp),
    new State(activateBooster),
    new State(boosterType),
  );
}
