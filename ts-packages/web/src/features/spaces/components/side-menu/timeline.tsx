import { Settings2 } from '@/assets/icons/settings';
import { Clock } from '@/assets/icons/time';
import Card from '@/components/card';
import { getTimeWithFormat } from '@/lib/time-utils';

interface TimeItems {
  label: string;
  time: number;
}
export default function TimelineMenu({
  items,
  isEditing,
  titleLabel,
  handleSetting,
}: {
  items: TimeItems[];
  isEditing: boolean;
  titleLabel: string;
  handleSetting: () => void;
}) {
  const sortedItems = [...items].sort((a, b) => a.time - b.time);
  return (
    <Card>
      <div className="flex flex-col w-full text-neutral-500 gap-5">
        <div className="flex flex-row font-bold text-sm justify-between">
          <div className="flex flex-row gap-2">
            <Clock className="size-5 *:stroke-neutral-500" />
            {titleLabel}
          </div>
          {isEditing && (
            <button
              type="button"
              aria-label="Timeline settings"
              onClick={handleSetting}
              className="cursor-pointer"
            >
              <Settings2 className="size-5 *:stroke-neutral-500" />
            </button>
          )}
        </div>
        <div className="flex flex-col pl-3.25 gap-5">
          {sortedItems.map((item) => (
            <div
              className="flex flex-col gap-1 text-text-primary"
              key={item.label}
            >
              <div className="font-medium text-text-primary text-[15px]/[12px]">
                {item.label}
              </div>
              <div className="font-medium text-neutral-80 text-xs/[12px]">
                {getTimeWithFormat(item.time)}
              </div>
            </div>
          ))}
        </div>
      </div>
    </Card>
  );
}
