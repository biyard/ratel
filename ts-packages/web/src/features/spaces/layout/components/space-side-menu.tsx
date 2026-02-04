import Card from '@/components/card';
import { cn } from '@/lib/utils';
import { Link } from 'react-router';
import { Badge } from '@/components/ui/badge';
import { Unlock1 } from '@/assets/icons/security';

export type SpaceSideMenuProps = React.HTMLAttributes<HTMLDivElement> & {
  menus?: SideMenuProps[];
  selectedMenu: SideMenuProps;
  onMenuClick?: () => void;
};

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
  tag?: {
    label: string;
    visible: boolean;
  };
};

export default function SpaceSideMenu({
  menus,
  selectedMenu,
  onMenuClick,
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
            'w-full cursor-pointer flex items-center justify-between rounded-sm data-[active=true]:bg-primary/5',
            'flex-row gap-3 px-1 py-2',
          )}
          onClick={() => {
            onMenuClick?.();
          }}
        >
          <div className="flex items-center gap-3">
            <item.Icon className="[&>path]:stroke-neutral-500 [&>rect]:stroke-neutral-500 size-5" />
            <div className="text-sm font-bold text-text-primary">
              {item.label}
            </div>
          </div>

          {item.tag?.visible && (
            <Badge
              size="sm"
              className="rounded-full shrink-0 bg-transparent text-primary border-primary px-2 py-3"
            >
              <Unlock1 className="size-4 *:stroke-primary" />
              {item.tag.label}
            </Badge>
          )}
        </Link>
      ))}
    </Card>
  );
}
