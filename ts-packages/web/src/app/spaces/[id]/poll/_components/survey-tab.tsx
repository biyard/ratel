'use client';
import React from 'react';
import { Space } from '@/lib/api/models/spaces';
import SpaceSurvey from '../../_components/space-survey';
import { useEditCoordinatorStore } from '../../space-store';
import { usePollStore } from '../store';
import { usePollMutation } from '@/hooks/use-poll';
import { getTimeWithFormat } from '@/lib/time-utils';

export function PollSurveyPage({ space }: { space: Space }) {
  const { isEdit } = useEditCoordinatorStore();
  const { survey, answer, updateSurvey, updateAnswer } = usePollStore();
  const { status, started_at, ended_at } = space;
  const { mutateAsync } = usePollMutation();
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5">
        <div className="hidden max-tablet:flex flex-row w-full justify-end items-center font-medium text-neutral-80 text-xs/[12px] gap-[10px]">
          <div>{getTimeWithFormat(started_at ?? 0)}</div>
          <div>~</div>
          <div>{getTimeWithFormat(ended_at ?? 0)}</div>
        </div>
        <SpaceSurvey
          isEdit={isEdit}
          startDate={started_at || 0}
          endDate={ended_at || 0}
          survey={survey}
          answer={answer}
          status={status}
          space={space}
          handleSetAnswers={updateAnswer}
          handleSend={async () => {
            await mutateAsync({
              spaceId: space.id,
              questions: survey.surveys[0].questions,
              answer,
            });
          }}
          handleUpdateSurvey={updateSurvey}
        />
      </div>
    </div>
  );
}
