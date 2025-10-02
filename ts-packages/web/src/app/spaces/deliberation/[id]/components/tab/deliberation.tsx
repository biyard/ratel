'use client';

import React from 'react';
import SpaceDiscussion from '../space-discussion';
import SpaceElearning from '../space-elearning';

export default function DeliberationPage() {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceDiscussion />
        <SpaceElearning />
      </div>
    </div>
  );
}
