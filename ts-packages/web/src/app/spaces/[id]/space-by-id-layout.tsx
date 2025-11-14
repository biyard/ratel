import { createContext, useState } from 'react';
import { Outlet, useLocation, useParams } from 'react-router';
import {
  SpaceHomeController,
  useSpaceHomeController,
} from './use-space-home-controller';
import { Row } from '@/components/ui/row';
import SpaceSideMenu from '@/features/spaces/components/space-side-menu';
import { Col } from '@/components/ui/col';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';
import TimelineMenu from '@/features/spaces/components/side-menu/timeline';
import { SpaceActions } from '@/features/spaces/components/space-actions';
import SpaceParticipantProfile from '@/features/spaces/components/space-participant-profile';
import { cn } from '@/lib/utils';
import { useSpaceLayoutContext } from './use-space-layout-context';
import { Requirements } from '@/features/spaces/components/requirements';
import { SafeArea } from '@/components/ui/safe-area';
import { Sheet, SheetContent } from '@/components/ui/sheet';
import { Button } from '@/components/ui/button';
import { useIsMobile } from '@/hooks/use-mobile';
import { Bullet1 } from '@/components/icons';

export const Context = createContext<SpaceHomeController | undefined>(
  undefined,
);

function GeneralLayout() {
  const ctrl = useSpaceLayoutContext();
  const location = useLocation();
  const showInfo = !/\/boards\/posts(\/|$)/.test(location.pathname);
  const isMobile = useIsMobile();
  const [sheetOpen, setSheetOpen] = useState(false);

  const participantProfileProps =
    ctrl.space.participated &&
    ctrl.space.participantDisplayName &&
    ctrl.space.participantProfileUrl &&
    ctrl.space.participantUsername
      ? {
          displayName: ctrl.space.participantDisplayName,
          profileUrl: ctrl.space.participantProfileUrl,
          username: ctrl.space.participantUsername,
        }
      : null;

  return (
    <Row>
      <Col className="gap-4 w-full">
        {showInfo && (
          <Col className="gap-4 w-full">
            <TitleSection
              canEdit={ctrl.isAdmin}
              title={ctrl.space.title}
              setTitle={ctrl.handleTitleChange}
            />
            <AuthorSection
              type={ctrl.space.authorType}
              profileImage={ctrl.space.authorProfileUrl}
              name={ctrl.space.authorDisplayName}
              isCertified={ctrl.space.certified}
              createdAt={ctrl.space.createdAt}
            />

            <PostInfoSection
              likes={ctrl.space.likes}
              shares={ctrl.space.shares}
              comments={ctrl.space.comments}
              rewards={ctrl.space.rewards ?? 0}
              isDraft={ctrl.space.isDraft}
              isPublic={ctrl.space.isPublic}
            />
          </Col>
        )}

        <Outlet />
      </Col>

      <Col
        className={cn(
          'gap-2.5 transition-all',
          isMobile ? 'w-auto items-center' : 'w-full max-w-[250px]',
        )}
      >
        {/* Mobile expand button */}
        {isMobile && (
          <Button
            onClick={() => setSheetOpen(true)}
            variant="default"
            className="size-12"
            aria-label="Expand space menu"
          >
            <Bullet1 className="size-4" />
          </Button>
        )}

        {/* Desktop actions (hidden on mobile) */}
        {!isMobile && ctrl.actions.length > 0 && (
          <SpaceActions actions={ctrl.actions} />
        )}

        {/* Participant profile - icon only on mobile */}
        {participantProfileProps && (
          <SpaceParticipantProfile
            {...participantProfileProps}
            iconOnly={isMobile}
          />
        )}

        {/* Side menu - icon only on mobile */}
        <SpaceSideMenu menus={ctrl.menus} iconOnly={isMobile} />

        {/* Timeline - desktop only */}
        {!isMobile && (
          <TimelineMenu
            isEditing={false}
            handleSetting={() => {}}
            items={ctrl.timelineItems}
            titleLabel={ctrl.t('timeline_title')}
          />
        )}

        {/* Mobile sheet with full content */}
        {isMobile && (
          <Sheet open={sheetOpen} onOpenChange={setSheetOpen}>
            <SheetContent side="right" className="w-full overflow-y-auto p-5">
              <Col className="gap-4 mt-4" onClick={() => setSheetOpen(false)}>
                {ctrl.actions.length > 0 && (
                  <SpaceActions actions={ctrl.actions} />
                )}

                {participantProfileProps && (
                  <SpaceParticipantProfile {...participantProfileProps} />
                )}

                <SpaceSideMenu menus={ctrl.menus} />

                <TimelineMenu
                  isEditing={false}
                  handleSetting={() => {}}
                  items={ctrl.timelineItems}
                  titleLabel={ctrl.t('timeline_title')}
                />
              </Col>
            </SheetContent>
          </Sheet>
        )}
      </Col>
    </Row>
  );
}

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk ?? '');

  // NOTE: Must authorize permission for viewer/participant/admin before
  return (
    <Context.Provider value={ctrl}>
      <SafeArea>
        {ctrl.space.havePreTasks() && !ctrl.space.isAdmin() ? (
          <Requirements />
        ) : (
          <GeneralLayout />
        )}
      </SafeArea>
    </Context.Provider>
  );
}
