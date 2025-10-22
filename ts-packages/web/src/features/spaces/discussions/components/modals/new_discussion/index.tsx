import { useState } from 'react';
import { useNewDiscussionModalController } from './use-new-discussion-modal-controller';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import CalendarDropdown from '@/components/calendar-dropdown';
import TimeDropdown from '@/components/time-dropdown';
import { showErrorToast } from '@/lib/toast';
import { Internet } from '@/components/icons';

export default function NewDiscussion({
  spacePk,
  discussionPk,

  startedAt,
  endedAt,
  name,
  description,
}: {
  spacePk: string;
  discussionPk: string | null | undefined;

  startedAt: number;
  endedAt: number;
  name: string;
  description: string;
}) {
  const ctrl = useNewDiscussionModalController(
    spacePk,
    discussionPk,
    startedAt,
    endedAt,
    name,
    description,
  );

  const [diff, setDiff] = useState<number>(endedAt - startedAt);
  const t = ctrl.t;

  return (
    <div className="max-w-[900px] w-full max-tablet:w-full">
      <div className="flex flex-col py-2.5 gap-[5px] w-full max-tablet:w-full max-tablet:h-[350px] overflow-y-auto">
        <div className="flex flex-col ">
          <label className="flex flex-row justify-start items-center text-[15px]/[28px] text-modal-label-text font-bold  gap-1">
            {t('title')} <span className="text-error">*</span>
          </label>
          <Input
            className="px-5 py-[10.5px] bg-input-box-bg border border-input-box-border font-medium text-[15px]/[22.5px] placeholder:text-neutral-600 text-text-primary"
            placeholder={t('title_hint')}
            value={ctrl.name.get()}
            onChange={(e) => ctrl.handleChangeName(e.target.value)}
            maxLength={100}
          />
          <div className="text-right text-[15px]/[22.5px] font-medium text-neutral-600 ">
            {ctrl.name.get().length}/100
          </div>
        </div>

        <div className="flex flex-col py-2.5 gap-[5px]">
          <label className="text-[15px]/[28px] font-bold text-modal-label-text">
            {t('description')}
          </label>
          <Textarea
            className="px-5 py-[10.5px] bg-input-box-bg border border-input-box-border font-normal text-sm placeholder:text-neutral-600 text-text-primary max-h-[100px] overflow-y-auto"
            placeholder={t('description_hint')}
            value={ctrl.description.get()}
            onChange={(e) => ctrl.handleChangeDescription(e.target.value)}
            maxLength={100}
          />
          <div className="text-right text-[15px]/[22.5px] font-medium text-neutral-600">
            {ctrl.description.get().length}/100
          </div>
        </div>

        <div className="flex flex-col py-2.5 gap-[5px]">
          <label className="flex flex-row justify-start items-center text-[15px]/[28px]font-bold  gap-1 text-modal-label-text">
            {t('date')} <span className="text-error">*</span>
          </label>
          <div className="flex flex-row max-tablet:flex-col gap-2.5 items-center">
            <CalendarDropdown
              value={ctrl.startedAt.get()}
              onChange={(date) => {
                const selected = new Date(date);
                const current = new Date(ctrl.startedAt.get());

                selected.setHours(current.getHours());
                selected.setMinutes(current.getMinutes());
                selected.setSeconds(0);
                selected.setMilliseconds(0);

                const newStart = Math.floor(selected.getTime());

                ctrl.handleChangeStartedAt(newStart);
                ctrl.handleChangeEndedAt(newStart + diff);
              }}
            />
            <div className="max-tablet:mb-[20px] max-tablet:w-full">
              <TimeDropdown
                value={ctrl.startedAt.get()}
                onChange={(timestamp) => {
                  const newStart = Math.floor(timestamp);

                  ctrl.handleChangeStartedAt(newStart);
                  ctrl.handleChangeEndedAt(newStart + diff);
                }}
              />
            </div>
            <div className="w-[15px] h-0.25 bg-neutral-600 max-tablet:hidden" />
            <CalendarDropdown
              value={ctrl.endedAt.get()}
              onChange={(date) => {
                const selected = new Date(date);
                const current = new Date(ctrl.endedAt.get());

                selected.setHours(current.getHours());
                selected.setMinutes(current.getMinutes());
                selected.setSeconds(0);
                selected.setMilliseconds(0);

                const newEnd = Math.floor(selected.getTime());
                const diff = newEnd - ctrl.startedAt.get();

                if (newEnd < ctrl.startedAt.get()) {
                  showErrorToast(
                    'The end date must be later than the start date.',
                  );
                  return;
                }

                setDiff(diff);
                ctrl.handleChangeEndedAt(newEnd);
              }}
            />
            <TimeDropdown
              value={ctrl.endedAt.get()}
              onChange={(timestamp) => {
                const newEnd = Math.floor(timestamp);
                const diff = newEnd - ctrl.startedAt.get();

                if (newEnd < ctrl.startedAt.get()) {
                  showErrorToast(
                    'The end date must be later than the start date.',
                  );
                  return;
                }

                setDiff(diff);
                ctrl.handleChangeEndedAt(newEnd);
              }}
            />
            <div className="flex flex-row items-center w-fit max-tablet:w-full max-tablet:justify-between border border-select-date-border bg-select-date-bg rounded-lg px-5 py-[10.5px] gap-2.5 mt-2 sm:mt-0">
              <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
                Pacific Time
              </div>
              <Internet
                className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
                width="20"
                height="20"
              />
            </div>
          </div>
        </div>
      </div>

      {/* <div className="flex flex-row w-full py-5 items-start gap-2.5">
        <CustomCheckbox
          checked={reminderEnabled}
          onChange={() => setReminderEnabled(!reminderEnabled)}
          disabled={false}
        />
        <div className="text-[15px]/[24px]">
          <div className="font-medium text-white">Reminder Notification</div>
          <div className="font-normal text-neutral-300">
            A reminder email will be sent 10 minutes prior to the discussion.
          </div>
        </div>
      </div> */}

      <div className="flex justify-end mt-2.5">
        <button
          className="w-fit px-10 py-[14.5px] rounded-[10px] bg-primary hover:bg-hover text-black text-bold text-base hover:text-black cursor-pointer"
          onClick={ctrl.handleNext}
        >
          {t('continue')}
        </button>
      </div>
    </div>
  );
}
