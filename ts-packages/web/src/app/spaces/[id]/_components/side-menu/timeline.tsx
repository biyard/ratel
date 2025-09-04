import BlackBox from '@/app/(social)/_components/black-box';
import { Settings2 } from '@/assets/icons/settings';
import { Clock } from '@/assets/icons/time';
import { getTimeWithFormat } from '@/lib/time-utils';
import { useTranslations } from 'next-intl';

interface TimeItems {
  label: string;
  time: number;
}
export default function TimelineMenu({
  items,
  isEditing,
  handleSetting,
}: {
  items: TimeItems[];
  isEditing: boolean;
  handleSetting: () => void;
}) {
  const s = useTranslations('SprintSpace');
  const sortedItems = [...items].sort((a, b) => a.time - b.time);
  return (
    <BlackBox>
      <div className="flex flex-col w-full text-neutral-500 gap-5">
        <div className="flex flex-row font-bold text-sm justify-between">
          <div className="flex flex-row gap-2">
            <Clock className="size-5 [&>*]:stroke-neutral-500" />
            {s('timeline')}
          </div>
          {isEditing && (
            <button
              type="button"
              aria-label="Timeline settings"
              onClick={handleSetting}
              className="cursor-pointer"
            >
              <Settings2 className="size-5 [&>*]:stroke-neutral-500" />
            </button>
          )}
        </div>
        <div className="flex flex-col pl-3.25 gap-5">
          {sortedItems.map((item) => (
            <div className="flex flex-col gap-1 text-white" key={item.label}>
              <div className="font-medium text-white text-[15px]/[12px]">
                {item.label}
              </div>
              <div className="font-medium text-neutral-80 text-xs/[12px]">
                {getTimeWithFormat(item.time)}
              </div>
            </div>
          ))}
        </div>
      </div>
    </BlackBox>
  );
}
