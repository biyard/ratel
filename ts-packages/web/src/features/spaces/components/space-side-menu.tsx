import Card from '@/components/card';
import { useState } from 'react';
import { Link } from 'react-router';

export type SpaceSideMenuProps = React.HTMLAttributes<HTMLDivElement> & {
  menus?: SideMenuProps[];
  iconOnly?: boolean;
};

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
};

export default function SpaceSideMenu({
  menus,
  iconOnly = false,
}: SpaceSideMenuProps) {
  const currentPath = window.location.pathname;

  const [selected, setSelected] = useState(
    menus?.findIndex((item) => item.to === currentPath) ?? 0,
  );

  return (
    <>
      <Card
        className={
          iconOnly
            ? 'flex flex-col gap-2.5 w-fit p-2'
            : 'flex flex-col gap-2.5 w-full'
        }
      >
        {menus.map((item, i) => (
          <Link
            key={`side-menu-item-${item.label}`}
            type="button"
            to={item.to}
            data-active={selected === i}
            className={
              iconOnly
                ? 'w-full cursor-pointer flex items-center justify-center p-2 rounded-sm data-[active=true]:bg-neutral-800 light:data-[active=true]:bg-neutral-300'
                : 'w-full cursor-pointer flex flex-row gap-3 items-center px-1 py-2 rounded-sm data-[active=true]:bg-neutral-800 light:data-[active=true]:bg-neutral-300'
            }
            onClick={() => {
              setSelected(i);
            }}
          >
            <item.Icon className="[&>path]:stroke-neutral-80 [&>rect]:stroke-neutral-80 w-5 h-5" />
            {!iconOnly && (
              <div className="text-sm font-bold text-text-primary">
                {item.label}
              </div>
            )}
          </Link>
        ))}
      </Card>
    </>
  );
}
