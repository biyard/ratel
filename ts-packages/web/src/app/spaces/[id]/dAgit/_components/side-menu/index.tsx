'use client';

import { TimelineMenu } from '../../../_components/side-menu';
import useDagitBySpaceId from '@/hooks/use-dagit';

export default function SideMenu({ spaceId }: { spaceId: number }) {
  const { data: dagit } = useDagitBySpaceId(spaceId);
  console.log('dagit', dagit);
  const items = [];
  items.push({
    label: 'Created',
    time: dagit.created_at,
  });
  //   if (dagit.started_at) {
  //     items.push({
  //       label: 'Start',
  //       time: dagit.started_at,
  //     });
  //   }

  const handleTimelineSetting = () => {};
  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <TimelineMenu
        isEditing={false}
        handleSetting={handleTimelineSetting}
        items={items}
      />
    </div>
  );
}
