'use client';

import React from 'react';
import SpaceSurvey from '../space-survey';
import { Space } from '@/lib/api/models/spaces';
import { usePollSpaceContext } from '../../poll/provider.client';
import { useDeliberationSpaceContext } from '../../deliberation/provider.client';
// import { Poll } from '../page.client';

export function PollSurveyPage({ space }: { space: Space }) {
  const {
    isEdit,
    startedAt,
    endedAt,
    survey,
    answer,
    status,
    handleSetAnswers,
    handleSend,
    handleUpdateSurvey,
  } = usePollSpaceContext();

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceSurvey
          isEdit={isEdit}
          startDate={startedAt}
          endDate={endedAt}
          survey={survey}
          answer={answer}
          status={status}
          space={space}
          handleSetAnswers={handleSetAnswers}
          handleSend={handleSend}
          handleUpdateSurvey={handleUpdateSurvey}
        />
      </div>
    </div>
  );
}

export function DeliberationSurveyPage({ space }: { space: Space }) {
  const {
    isEdit,
    startedAt,
    endedAt,
    survey,
    answer,
    status,
    handleSetAnswers,
    handleSend,
    handleUpdateSurvey,
  } = useDeliberationSpaceContext();
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <SpaceSurvey
          isEdit={isEdit}
          startDate={startedAt}
          endDate={endedAt}
          survey={survey}
          answer={answer}
          status={status}
          space={space}
          handleSetAnswers={handleSetAnswers}
          handleSend={handleSend}
          handleUpdateSurvey={handleUpdateSurvey}
        />
      </div>
    </div>
  );
}
