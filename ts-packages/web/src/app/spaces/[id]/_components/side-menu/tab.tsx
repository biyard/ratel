import BlackBox from '@/app/(social)/_components/black-box';

interface TabMenuItem<T> {
  icon: React.ReactNode;
  label: string;
  tab: T;
}

export default function TabMenu<T>({
  items,
  onClick,
  activeTab,
}: {
  items: TabMenuItem<T>[];
  onClick: (tab: T) => void;
  activeTab: T;
}) {
  return (
    <BlackBox>
      <div className="flex flex-col gap-2.5 w-full">
        {items.map((item) => (
          <button
            key={item.label}
            type="button"
            data-active={activeTab === item.tab}
            className={`cursor-pointer flex flex-row gap-3 items-center px-1 py-2 rounded-sm data-[active=true]:bg-neutral-800 light:data-[active=true]:bg-neutral-300`}
            onClick={() => {
              onClick(item.tab);
            }}
          >
            {item.icon}
            <div className="font-bold text-text-primary text-sm">
              {item.label}
            </div>
          </button>
        ))}
      </div>
    </BlackBox>
  );
}
