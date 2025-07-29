'use client';

import React from 'react';
import SpaceSurvey from './space_survey';
// import { Poll } from '../page.client';

export default function PollPage() {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceSurvey />
      </div>
    </div>
  );
}
