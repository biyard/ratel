import { useEffect, useMemo } from 'react';
import {
  SpaceRequirement,
  SpaceRequirementType,
} from '../../types/space-requirement';
import PollRequirement from './poll-requirement';
import { logger } from '@/lib/logger';
import {
  SpaceLayoutController,
  useSpaceLayoutController,
} from '../use-space-layout-controller';

export class Controller {
  constructor(public layoutCtrl: ReturnType<typeof useSpaceLayoutController>) {}

  get requirements() {
    return this.layoutCtrl.space.requirements;
  }

  get currentIndex() {
    return this.layoutCtrl.requirementIndex;
  }

  get component() {
    const req = this.requirements[this.currentIndex];
    if (!req) {
      return <></>;
    }
    return getComponent(req, this.handleNext);
  }

  handleNext = () => {
    logger.debug('Controller handleNext called');
    this.layoutCtrl.handleNextRequirement();
  };
}

export function useRequirmentController(layoutCtrl: SpaceLayoutController) {
  // Hide layout when showing requirements
  useEffect(() => {
    layoutCtrl.setShouldHideLayout(true);
    return () => {
      layoutCtrl.setShouldHideLayout(false);
    };
  }, [layoutCtrl]);

  const controller = useMemo(() => new Controller(layoutCtrl), [layoutCtrl]);

  return controller;
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
