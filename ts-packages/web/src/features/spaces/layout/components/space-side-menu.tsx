import Card from '@/components/card';
import { cn } from '@/lib/utils';
import { Link } from 'react-router';

export type SpaceSideMenuProps = React.HTMLAttributes<HTMLDivElement> & {
  menus?: SideMenuProps[];
  selectedMenu: SideMenuProps;
};

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
};

export default function SpaceSideMenu({
  menus,
  selectedMenu,
}: SpaceSideMenuProps) {
  const selectedIndex = selectedMenu
    ? menus?.findIndex((item) => item.to === selectedMenu.to)
    : -1;

  return (
    <Card className={cn('flex flex-col gap-2.5 w-full p-0')}>
      {menus.map((item, i) => (
        <Link
          key={`side-menu-item-${item.label}`}
          type="button"
          data-testid={`space-sidemenu-${item.label}`.toLowerCase()}
          to={item.to}
          data-active={selectedIndex === i}
          className={cn(
            'w-full cursor-pointer flex items-center rounded-sm data-[active=true]:bg-primary/5',
            'flex-row gap-3 px-1 py-2',
          )}
          // onClick={() => {
          //   setSelected(i);
          // }}
        >
          <item.Icon className="[&>path]:stroke-neutral-500 [&>rect]:stroke-neutral-500 size-5" />
          <div className="text-sm font-bold text-text-primary">
            {item.label}
          </div>
        </Link>
      ))}
    </Card>
  );
}
