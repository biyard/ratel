'use client';

import { useCallback, useEffect } from 'react';
import { CommonEditableData, useEditCoordinatorStore } from '../../space-store';
import { Tab, usePollStore } from '../store';
import useSpaceById, { useUpdateSpace } from '@/hooks/use-space-by-id';
import { SpaceStatus, spaceUpdateRequest } from '@/lib/api/models/spaces';
import { Poll, SurveyAnswer } from '../../type';

export default function Initial({ spaceId }: { spaceId: number }) {
  const { data: space } = useSpaceById(spaceId);
  const { isEdit, setPageSaveHandler } = useEditCoordinatorStore();
  const { initialize } = usePollStore();
  const { mutateAsync: updateMutateAsync } = useUpdateSpace(spaceId);

  const saveHandler = useCallback(
    async (commonData: Partial<CommonEditableData>) => {
      if (!space) {
        return false;
      }
      const survey = usePollStore.getState().survey;
      const surveys = survey.surveys.map((survey) => ({
        started_at: commonData.started_at || Math.floor(Date.now() / 1000),
        ended_at: commonData.ended_at || Math.floor(Date.now() / 1000),
        questions: survey.questions,
      }));
      try {
        await updateMutateAsync(
          spaceUpdateRequest(
            commonData.html_contents ?? '',
            [],
            [],
            [],
            surveys,
            [],
            commonData.title,
            commonData.started_at,
            commonData.ended_at,
          ),
        );
        return true;
      } catch (error) {
        console.error('Save failed:', error);
        return false;
      }
    },
    [space, updateMutateAsync],
  );

  useEffect(() => {
    if (isEdit) {
      setPageSaveHandler(saveHandler);
    }
  }, [isEdit, setPageSaveHandler, saveHandler]);

  useEffect(() => {
    const survey: Poll = {
      surveys: space.surveys.map((survey) => ({
        started_at: survey.started_at,
        ended_at: survey.ended_at,
        questions: survey.questions,
      })),
    };

    const answer: SurveyAnswer = {
      answers:
        space.user_responses.length != 0 ? space.user_responses[0].answers : [],
      is_completed:
        space.user_responses.length !== 0
          ? space.user_responses[0].survey_type === 1
            ? false
            : true
          : false,
    };
    const activeTab =
      space.status === SpaceStatus.Finish ? Tab.Analyze : Tab.Poll;
    initialize(survey, answer, activeTab);
  }, [initialize, space.status, space.surveys, space.user_responses]);

  return null;
}
