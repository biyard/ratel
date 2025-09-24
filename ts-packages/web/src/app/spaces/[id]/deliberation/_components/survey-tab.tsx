'use client';
import React from 'react';
import { useDeliberationSpaceContext } from '../provider.client';
import { Space } from '@/lib/api/models/spaces';
import SpaceSurvey from '../../_components/space-survey';
import { getTimeWithFormat } from '@/lib/time-utils';
import { Settings } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import SetSchedulePopup from '../../_components/modal/set-schedule';

export function DeliberationSurveyPage({ space }: { space: Space }) {
  const popup = usePopup();

  const {
    isEdit,
    startedAt,
    endedAt,
    survey,
    answer,
    status,
    handleUpdateEndDate,
    handleUpdateStartDate,
    handleSetAnswers,
    handleSend,
    handleUpdateSurvey,
  } = useDeliberationSpaceContext();
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2.5 w-full">
        <div className="hidden max-tablet:flex flex-row w-full justify-end items-center font-medium text-neutral-80 text-xs/[12px] gap-[10px]">
          <div>{getTimeWithFormat(startedAt ?? 0)}</div>
          <div>~</div>
          <div>{getTimeWithFormat(endedAt ?? 0)}</div>

          {isEdit ? (
            <div
              className="cursor-pointer w-fit h-fit"
              onClick={() => {
                popup
                  .open(
                    <SetSchedulePopup
                      startedAt={startedAt}
                      endedAt={endedAt}
                      onconfirm={(startDate: number, endDate: number) => {
                        handleUpdateStartDate(Math.floor(startDate / 1000));
                        handleUpdateEndDate(Math.floor(endDate / 1000));
                        popup.close();
                      }}
                    />,
                  )
                  .overflow(true);
              }}
            >
              <Settings
                width={20}
                height={20}
                className="text-neutral-500 w-5 h-5"
              />
            </div>
          ) : (
            <></>
          )}
        </div>
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
