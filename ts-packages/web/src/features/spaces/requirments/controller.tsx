import PollRequirement from './poll-requirement';
import { useSpaceById } from '../hooks/use-space-by-id';
import { SpaceRequirementType } from './types';
import { useCallback, useState } from 'react';
import { useNavigate } from 'react-router';
import { route } from '@/route';

export interface RequirmentController {
  isRequired: boolean;
  getComponent: () => React.ReactNode;
}

export function useRequirmentController(spacePk): RequirmentController {
  const { data: space } = useSpaceById(spacePk);
  const navigator = useNavigate();
  const [index, setIndex] = useState(
    space.requirements.findIndex((el) => !el.responded),
  );
  const isRequired = space.havePreTasks();
  const requirement = space.requirements;

  const getComponent = useCallback(() => {
    const req = requirement[index];
    if (req.typ === SpaceRequirementType.PrePoll) {
      return (
        <PollRequirement
          spacePk={req.related_pk}
          pollSk={req.related_sk}
          onNext={() => {
            setIndex((prev) => {
              const nextIndex = prev + 1;
              if (nextIndex >= requirement.length) {
                navigator(route.spaceHome(space.pk));
              }
              return nextIndex;
            });
          }}
        />
      );
    }

    return <></>;
  }, [index, requirement, navigator, space.pk]);

  return { isRequired, getComponent };
}
