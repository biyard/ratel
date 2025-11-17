import { Button } from '@/components/ui/button';
import { Row } from '@/components/ui/row';
import { Bullet1 } from '@/components/icons';
import { cn } from '@/lib/utils';
import SpaceParticipantProfile, {
  SpaceParticipantProfileProps,
} from './space-participant-profile';
import { Col } from '@/components/ui/col';
import { SideMenuProps } from './space-side-menu';

export type SpaceMobileHeaderProps = {
  participantProfile?: SpaceParticipantProfileProps;
  currentTab?: SideMenuProps;
  onMenuClick: () => void;
  className?: string;
};

export default function SpaceMobileHeader({
  participantProfile,
  currentTab,
  onMenuClick,
  className,
}: SpaceMobileHeaderProps) {
  return (
    <Col
      className={cn(
        'w-full items-center justify-between gap-3 py-3',
        className,
      )}
    >
      {participantProfile && (
        <SpaceParticipantProfile {...participantProfile} />
      )}
      <Row className="justify-between items-center p-2 w-full">
        {/* Current Tab with Icon */}
        {currentTab && (
          <Row className="items-center gap-2 flex-1 min-w-0 px-3 py-2 rounded-sm bg-neutral-800 light:bg-neutral-300">
            <currentTab.Icon className="[&>path]:stroke-neutral-80 [&>rect]:stroke-neutral-80 w-5 h-5 shrink-0" />
            <span className="text-sm font-bold text-text-primary truncate">
              {currentTab.label}
            </span>
          </Row>
        )}

        {/* Menu Button */}
        <Button
          onClick={onMenuClick}
          className="shrink-0 ml-2"
          aria-label="Open menu"
        >
          <Bullet1 className="size-5" />
        </Button>
      </Row>
    </Col>
  );
}
