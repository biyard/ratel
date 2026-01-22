import { Add, Check } from '@/assets/icons/validations';
import { Trophy } from '@/assets/icons/game';
import Card from '@/components/card';

import { useSpaceRewardsI18n } from '../i18n';
import { cn } from '@/lib/utils';

interface RewardItem {
  label: string;
  point: number;
  isUserRewared: boolean;
}

interface RewardMenuProps {
  rewardItems: RewardItem[];
}

export default function RewardMenu({ rewardItems }: RewardMenuProps) {
  const i18n = useSpaceRewardsI18n();
  const totalEstimatedValue = rewardItems.reduce(
    (sum, item) => sum + item.point,
    0,
  );
  return (
    <Card>
      <div className="flex flex-col w-full text-neutral-500 gap-5">
        <div className="flex flex-row gap-1 items-center">
          <Trophy className="size-5 *:stroke-neutral-500" />
          <span>{i18n.sidemenu.title}</span>
        </div>

        <div className="flex flex-col gap-5">
          {rewardItems.map((item) => (
            <div
              key={item.label}
              className={cn(
                'flex flex-row gap-2.5 text-text-primary',
                item.isUserRewared && 'text-primary',
              )}
            >
              <div className="flex flex-row gap-2.5 items-center">
                {item.isUserRewared ? (
                  <Check className="size-4 *:stroke-primary" />
                ) : (
                  <Add className="size-4 *:stroke-neutral-500" />
                )}
                <div className="flex flex-col gap-1">
                  <div className="font-medium text-base">
                    {item.point.toLocaleString()} P
                  </div>
                  <div className="font-medium text-xs">{item.label}</div>
                </div>
              </div>
            </div>
          ))}
        </div>

        <div className="w-full h-px bg-neutral-800" />

        <div className="flex flex-col gap-3.5">
          <div className="text-neutral-500 font-semibold text-sm">
            {i18n.sidemenu.totalPoint}
          </div>
          <div className="flex flex-col">
            <div className="text-text-primary font-bold text-2xl">
              {totalEstimatedValue.toLocaleString()} P
            </div>
            {/* <div className="flex flex-col">
              <span className="text-neutral-80 font-medium text-xs">
                {getTimeWithFormat(estimatedDate)}
              </span>
            </div> */}
          </div>
        </div>
      </div>
    </Card>
  );
}
