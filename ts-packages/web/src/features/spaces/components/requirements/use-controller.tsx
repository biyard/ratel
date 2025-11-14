import { useState } from 'react';
import { State } from '@/types/state';
import { useSpaceLayoutContext } from '@/app/spaces/[id]/use-space-layout-context';
import {
  SpaceRequirement,
  SpaceRequirementType,
} from '../../types/space-requirement';
import PollRequirement from './poll-requirement';
import { logger } from '@/lib/logger';

export class Controller {
  constructor(
    public current: State<number>,
    public layout: ReturnType<typeof useSpaceLayoutContext>,
  ) {}

  get requirements() {
    return this.layout.space.requirements;
  }

  get component() {
    return getComponent(
      this.layout.space.requirements[this.current.get()],
      this.handleNext,
    );
  }

  handleNext = () => {
    logger.debug('Controller handleNext called');

    const currentIdx = this.current.get();

    if (currentIdx >= this.requirements.length - 1) {
      this.current.set(this.current.get() + 1);
    } else {
      // Navigate
    }
  };
}

export function useController() {
  const layoutCtrl = useSpaceLayoutContext();
  const [idx, setIdx] = useState(
    layoutCtrl.space.requirements.findIndex((el) => !el.responded),
  );

  layoutCtrl.hiding.set(true);

  return new Controller(new State([idx, setIdx]), layoutCtrl);
}

function getComponent(req: SpaceRequirement, onNext: () => void) {
  if (req.typ === SpaceRequirementType.PrePoll) {
    return (
      <PollRequirement
        spacePk={req.related_pk}
        pollSk={req.related_sk}
        onNext={onNext}
      />
    );
  }

  return <></>;
}
