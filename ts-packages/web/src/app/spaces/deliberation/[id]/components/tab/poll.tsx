'use client';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { getTimeWithFormat } from '@/lib/time-utils';
import { Settings } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import SetSchedulePopup from '@/app/spaces/[id]/_components/modal/set-schedule';
import SpaceSurvey from '../space-survey';
import { DeliberationSpaceResponse } from '@/lib/api/ratel/deliberation.spaces.v3';
import { useSpaceHeaderStore } from '@/app/spaces/_components/header/store';
import { Poll } from '../../types';
import { SurveyAnswer } from '@/app/spaces/[id]/type';
import { Answer } from '@/lib/api/models/response';
import { TFunction } from 'i18next';

export type DeliberationSurveyPageProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  space: DeliberationSpaceResponse;
  startedAt: number;
  endedAt: number;
  survey: Poll;
  answer: SurveyAnswer;

  setStartDate: (startedAt: number) => void;
  setEndDate: (endedAt: number) => void;
  setSurvey: (survey: Poll) => void;
  setAnswers: (answers: Answer[]) => void;
  handleSend: () => Promise<void>;
};

export function DeliberationSurveyPage({
  space,
  startedAt,
  endedAt,
  survey,
  answer,
  setStartDate,
  setEndDate,
  setSurvey,
  setAnswers,
  handleSend,
}: DeliberationSurveyPageProps) {
  const store = useSpaceHeaderStore();
  const isEdit = store.isEditingMode;
  const popup = usePopup();

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
                        setStartDate(Math.floor(startDate / 1000));
                        setEndDate(Math.floor(endDate / 1000));
                        store.onModifyContent();
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
          status={SpaceStatus.Draft}
          space={space}
          handleSetAnswers={setAnswers}
          handleSend={handleSend}
          handleUpdateSurvey={(survey: Poll) => {
            setSurvey(survey);
            store.onModifyContent();
          }}
        />
      </div>
    </div>
  );
}
