'use client';

import React from 'react';
import SpaceSurvey from '../space-survey';
import { SpaceContextType } from '../../type';
import { Space } from '@/lib/api/models/spaces';
// import { Poll } from '../page.client';

export default function PollPage({
  context,
  space,
}: {
  context: SpaceContextType;
  space: Space;
}) {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceSurvey context={context} space={space} />
      </div>
    </div>
  );
}
