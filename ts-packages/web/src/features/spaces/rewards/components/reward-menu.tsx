import { History } from '@/assets/icons/time';
import { Add, Clear } from '@/assets/icons/validations';
import { Trophy } from '@/assets/icons/game';
import Card from '@/components/card';
import { getTimeWithFormat } from '@/lib/time-utils';
import { BoosterType, BoosterTypeToMultiplier } from '../../types/booster-type';
import { useRewardsI18n } from '../i18n';

interface RewardItem {
  label: string;
  point: number;
}

interface RewardMenuProps {
  rewardItems: RewardItem[];
  boosting?: BoosterType;
  estimatedDate: number; // timestamp in milliseconds
}

export default function RewardMenu({
  rewardItems,
  boosting,
  // estimatedDate,
}: RewardMenuProps) {
  const i18n = useRewardsI18n();
  const multiplierValue = boosting ? BoosterTypeToMultiplier(boosting) : null;
  const totalEstimatedValue =
    rewardItems.reduce((sum, item) => sum + item.point, 0) *
    (multiplierValue ?? 1);
  return (
    <Card>
      <div className="flex flex-col w-full text-neutral-500 gap-5">
        <div className="flex flex-row gap-1 items-center">
          <History className="size-5 *:stroke-neutral-500" />
          <span>{i18n.rewardSideMenuTitle}</span>
        </div>

        <div className="flex flex-col gap-5">
          {rewardItems.map((item) => (
            <div key={item.label} className="flex flex-row gap-2.5 ">
              <Add className="size-4 *:stroke-neutral-500" />
              <div className="flex flex-col gap-1">
                <div className="font-medium text-text-primary text-base">
                  {item.point.toLocaleString()} P
                </div>
                <div className="font-medium text-neutral-500 text-xs">
                  {item.label}
                </div>
              </div>
            </div>
          ))}
          {/* FIXME: Use space_common.rewards, Not boosting */}
          {/* {boosting && (
            <div className="flex flex-col gap-2.5 py-2.5 px-1.5 bg-primary/5 rounded">
              <div className="flex flex-row justify-between items-center">
                <div className="flex flex-row items-center justify-center gap-1">
                  <Trophy className="size-6 *:stroke-primary" />
                  <Clear className="size-4 *:stroke-primary" />
                  <span className="text-primary font-bold text-base">
                    {multiplierValue}
                  </span>
                </div>
                <div className="flex flex-col">
                  <span className="text-primary">
                    {i18n.rewardSideMenuBoosting}
                  </span>
                </div>
              </div>
            </div>
          )} */}
        </div>

        <div className="w-full h-px bg-neutral-800" />

        <div className="flex flex-col gap-3.5">
          <div className="text-neutral-500 font-semibold text-sm">
            {i18n.rewardSideMenuTotalEstimatedValue}
          </div>
          <div className="flex flex-col gap-2">
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
