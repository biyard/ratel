'use client';
import React from 'react';
import { usePollSpaceContext } from '../provider.client';
import { Space } from '@/lib/api/models/spaces';
import SpaceSurvey from '../../_components/space-survey';

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
