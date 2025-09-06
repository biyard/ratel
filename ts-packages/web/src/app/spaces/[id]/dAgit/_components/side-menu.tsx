'use client';

import useDagitBySpaceId from '@/hooks/use-dagit';
import { TimelineMenu } from '../../_components/side-menu';
import { useDagitStore, Tab } from '../dagit-store';

import { Content } from '@/assets/icons/notes-clipboard';
import { Palace } from '@/assets/icons/home';
import TabMenu from '../../_components/side-menu/tab';
export default function SideMenu({ spaceId }: { spaceId: number }) {
  const { data: dagit } = useDagitBySpaceId(spaceId);
  const { changeTab, activeTab } = useDagitStore();
  const items = [];
  items.push({
    label: 'Created',
    time: dagit.created_at,
  });
  if (dagit.started_at) {
    items.push({
      label: 'Start',
      time: dagit.started_at,
    });
  }

  const handleTimelineSetting = () => {};
  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-[10px]">
      <TabMenu<Tab>
        items={[
          {
            icon: <Content className="[&>path]:stroke-neutral-80 w-5 h-5" />,
            label: 'Content',
            tab: Tab.Content,
          },
          {
            icon: <Palace className="[&>*>path]:stroke-neutral-80 w-5 h-5" />,
            label: 'Artwork',
            tab: Tab.Artwork,
          },
        ]}
        onClick={(tab) => {
          changeTab(tab);
        }}
        activeTab={activeTab}
      />
      <TimelineMenu
        isEditing={false}
        handleSetting={handleTimelineSetting}
        items={items}
      />
    </div>
  );
}
