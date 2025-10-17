import Card from '@/components/card';
import { useState } from 'react';
import { Link } from 'react-router';

export type SpaceSideMenuProps = React.HTMLAttributes<HTMLDivElement> & {
  menus?: SideMenuProps[];
};

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
};

export default function SpaceSideMenu({ menus }: SpaceSideMenuProps) {
  const [selected, setSelected] = useState(0);

  return (
    <>
      <Card className="flex flex-col gap-2.5 w-full max-w-[250px]">
        {menus.map((item, i) => (
          <Link
            key={`side-menu-item-${item.label}`}
            type="button"
            to={item.to}
            data-active={selected === i}
            className="w-full cursor-pointer flex flex-row gap-3 items-center px-1 py-2 rounded-sm data-[active=true]:bg-neutral-800 light:data-[active=true]:bg-neutral-300"
            onClick={() => {
              setSelected(i);
            }}
          >
            <item.Icon className="[&>path]:stroke-neutral-80 [&>rect]:stroke-neutral-80 w-5 h-5" />
            <div className="text-sm font-bold text-text-primary">
              {item.label}
            </div>
          </Link>
        ))}
      </Card>
    </>
  );
}
