import Card from '@/components/card';
import { cn } from '@/lib/utils';
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
  const currentPath = window.location.pathname;

  const [selected, setSelected] = useState(
    menus?.findIndex((item) => item.to === currentPath) ?? 0,
  );

  return (
    <>
      <Card className={cn('flex flex-col gap-2.5 w-full')}>
        {menus.map((item, i) => (
          <Link
            key={`side-menu-item-${item.label}`}
            type="button"
            data-testid={`space-sidemenu-${item.label}`.toLowerCase()}
            to={item.to}
            data-active={selected === i}
            className={cn(
              'w-full cursor-pointer flex items-center rounded-sm data-[active=true]:bg-neutral-800 light:data-[active=true]:bg-neutral-300',
              'flex-row gap-3 px-1 py-2',
            )}
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
