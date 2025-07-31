'use client';

import React from 'react';
import SpaceSurvey from '../space-survey';
import { SpaceContextType } from '../../type';
// import { Poll } from '../page.client';

export default function PollPage({ context }: { context: SpaceContextType }) {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceSurvey context={context} />
      </div>
    </div>
  );
}
