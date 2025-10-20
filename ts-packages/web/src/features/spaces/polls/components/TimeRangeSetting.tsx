import CalendarDropdown from '@/components/calendar-dropdown';
import TimeDropdown from '@/components/time-dropdown';
import { TimeRangeSettingProps } from './time-range-setting';

export default function TimeRangeSetting({ info }: TimeRangeSettingProps) {
  return (
    <>
      <div className="flex flex-row gap-2 max-tablet:flex-wrap max-tabletitems-center">
        <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto">
          <div className="w-full max-tablet:w-auto">
            <CalendarDropdown
              value={ctrl.startTimestamp.get()}
              onChange={ctrl.handleStartTime}
            />
          </div>
          <div className="w-full sm:w-auto">
            <TimeDropdown
              value={ctrl.startTimestamp.get()}
              onChange={ctrl.handleEndTime}
            />
          </div>
        </div>

        <div className="hidden self-center h-0.5 sm:block w-[15px] bg-neutral-600" />

        <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto max-tablet:mt-2.5">
          <div className="w-full sm:w-auto">
            <CalendarDropdown
              value={ctrl.endTimestamp.get()}
              onChange={ctrl.handleEndTime}
            />
          </div>
          <div className="w-full sm:w-auto">
            <TimeDropdown
              value={ctrl.endTimestamp.get()}
              onChange={ctrl.handleEndTime}
            />
          </div>
        </div>

        <div className="flex flex-row gap-2.5 items-center px-5 mt-2 w-full rounded-lg border sm:mt-0 border-select-date-border bg-select-date-bg py-[10.5px] sm:w-fit">
          <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
            {ctrl.timezone.get()}
          </div>
          <Internet
            className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
            width="20"
            height="20"
          />
        </div>
      </div>
    </>
  );
}
