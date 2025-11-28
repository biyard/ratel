import { createContext, useMemo, useState } from 'react';
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
import { useSpaceLayoutContext } from './use-space-layout-context';
import { Requirements } from '@/features/spaces/components/requirements';
import { SafeArea } from '@/components/ui/safe-area';
import { Sheet, SheetContent } from '@/components/ui/sheet';
import { useIsMobile } from '@/hooks/use-mobile';
import SpaceMobileHeader from '@/features/spaces/components/space-mobile-header';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import RewardMenu from '@/features/spaces/rewards/components/reward-menu';

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
    !ctrl.space.isAdmin() &&
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

  const currentTab = useMemo(() => {
    const ret = ctrl.menus
      ?.filter((menu) => {
        return menu.label !== 'Overview' && menu.label !== '개요';
      })
      .find((menu) => {
        return location.pathname.startsWith(menu.to);
      });

    // If no match found, return Overview menu
    if (!ret) {
      return ctrl.menus?.find((menu) => {
        return menu.label === 'Overview' || menu.label === '개요';
      });
    }

    return ret;
  }, [ctrl.menus, location.pathname]);

  return (
    <Row className="max-mobile:gap-1">
      <Col className="gap-4 w-full">
        {/* Mobile Header */}
        {isMobile && (
          <SpaceMobileHeader
            participantProfile={participantProfileProps ?? undefined}
            currentTab={currentTab}
            onMenuClick={() => setSheetOpen(true)}
          />
        )}

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

      {/* Desktop Side Menu */}
      {!isMobile && (
        <Col className="gap-2.5 w-full max-w-[250px]">
          {ctrl.actions.length > 0 && <SpaceActions actions={ctrl.actions} />}

          {participantProfileProps && ctrl.space.anonymous_participation && (
            <SpaceParticipantProfile {...participantProfileProps} />
          )}

          <SpaceSideMenu menus={ctrl.menus} />
          {ctrl.space.booster && (
            <RewardMenu
              boosting={ctrl.space.booster}
              estimatedDate={0}
              rewardItems={[
                { label: 'Sample Reward 1', point: 5000 },
                { label: 'Sample Reward 2', point: 3000 },
                { label: 'Sample Reward 3', point: 2000 },
              ]}
            />
          )}
          <TimelineMenu
            isEditing={false}
            handleSetting={() => {}}
            items={ctrl.timelineItems}
            titleLabel={ctrl.t('timeline_title')}
          />
        </Col>
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
    </Row>
  );
}

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk ?? '');

  const content =
    ctrl.space.status !== SpaceStatus.InProgress && !ctrl.space.isAdmin() ? (
      <></>
    ) : !ctrl.space.havePreTasks() || ctrl.space.isAdmin() ? (
      <GeneralLayout />
    ) : ctrl.space.participated ? (
      <Requirements />
    ) : (
      <></>
    );

  // NOTE: Must authorize permission for viewer/participant/admin before
  return (
    <Context.Provider value={ctrl}>
      <SafeArea>{content}</SafeArea>
    </Context.Provider>
  );
}
