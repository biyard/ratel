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
    public state: State<boolean>,
    public layout: ReturnType<typeof useSpaceLayoutContext>,
    public component: State<React.ReactNode>,
    _handleNext: () => void,
  ) {
    _handleNext = this.handleNext.bind(this);
  }

  get requirements() {
    return this.layout.space.requirements;
  }

  handleNext = () => {
    logger.debug('Controller handleNext called');
  };
}

export function useController() {
  const state = useState(false);
  const layoutCtrl = useSpaceLayoutContext();
  const handleNext = () => {};
  const component = useState(
    getComponent(layoutCtrl.space.requirements[0], handleNext),
  );

  layoutCtrl.hiding.set(true);

  return new Controller(
    new State(state),
    layoutCtrl,
    new State(component),
    handleNext,
  );
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
