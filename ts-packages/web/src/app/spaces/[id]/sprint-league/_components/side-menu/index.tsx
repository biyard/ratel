'use client';

import { BoosterType } from '@/lib/api/models/notice';
import { Trophy } from '@/assets/icons/game';
import useSpaceById from '@/hooks/use-space-by-id';
import { RewardMenu, TimelineMenu } from '../../../_components/side-menu';

export default function SpaceSideMenu({ spaceId }: { spaceId: number }) {
  const { data: space } = useSpaceById(spaceId);

  const boosterType = (space.booster_type ??= BoosterType.NoBoost);
  const createdAt = space.created_at;
  const startedAt = space.started_at ?? Date.now();
  const endedAt = space.ended_at ?? Date.now();

  const modifierItem = {
    icon: <Trophy className="[&>*]:stroke-(--color-primary)" />,
    multiple: 1,
    text: 'Boosting',
    color: 'primary',
  };
  switch (boosterType) {
    case BoosterType.X2:
      modifierItem.multiple = 2;
      break;
    case BoosterType.X10:
      modifierItem.multiple = 10;
      break;
    case BoosterType.X100:
      modifierItem.multiple = 100;
      break;
  }
  const handleRewardSetting = () => {};
  const handleTimelineSetting = () => {};
  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <RewardMenu
        isEditing={false}
        handleSetting={handleRewardSetting}
        rewardItems={[
          { amount: 1000, text: 'Reposting' },
          { amount: 10000, text: 'Voting' },
        ]}
        modifierItems={[modifierItem]}
      />
      <TimelineMenu
        isEditing={false}
        handleSetting={handleTimelineSetting}
        items={[
          { label: 'Created', time: createdAt },
          { label: 'Start', time: startedAt },
          { label: 'End', time: endedAt },
        ]}
      />
    </div>
  );
}
