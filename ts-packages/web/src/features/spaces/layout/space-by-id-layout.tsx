import { useEffect, useMemo, useRef, useState } from 'react';
import { Outlet, useLocation, useParams } from 'react-router';
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
import { Requirements } from './requirements';
import { SafeArea } from '@/components/ui/safe-area';
import { Sheet, SheetContent } from '@/components/ui/sheet';
import { useIsMobile } from '@/hooks/use-mobile';
import SpaceMobileHeader from '@/features/spaces/components/space-mobile-header';
import { cn } from '@/lib/utils';
import {
  useSpaceLayoutController,
  SpaceLayoutController,
} from '@/features/spaces/layout/use-space-layout-controller';
import { useParticipateSpaceMutation } from '@/features/spaces/hooks/use-participate-space-mutation';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import SpaceAuthorizePopup from './components/space-authorize-popup';
import { useTranslation } from 'react-i18next';

function GeneralLayout({ ctrl }: { ctrl: SpaceLayoutController }) {
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

  // When Tab Changed, Read Current Feautures Rewards
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
    <Row
      data-testid="space-layout-root"
      className={cn(
        'flex flex-row items-start gap-4 flex-nowrap',
        isMobile && 'flex-col gap-1',
      )}
    >
      <Col className="gap-4 flex-1 min-w-0 basis-0">
        {/* Mobile Header */}
        {isMobile && (
          <SpaceMobileHeader
            participantProfile={participantProfileProps ?? undefined}
            currentTab={currentTab}
            onMenuClick={() => setSheetOpen(true)}
          />
        )}

        {ctrl.shouldShowLayout && showInfo && (
          <Col className="gap-4 w-full min-w-0">
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
              hasRewards={!!ctrl.space.rewards}
            />
          </Col>
        )}

        <Outlet />
      </Col>

      {/* Desktop Side Menu */}
      {!isMobile && ctrl.shouldShowLayout && (
        <Col className="gap-2.5 w-[250px] shrink-0">
          {ctrl.actions.length > 0 && <SpaceActions actions={ctrl.actions} />}

          {participantProfileProps && ctrl.space.anonymous_participation && (
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
      )}

      {/* Mobile sheet with full content */}
      {isMobile && ctrl.shouldShowLayout && (
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
  const ctrl = useSpaceLayoutController(spacePk!);
  const participateSpace = useParticipateSpaceMutation();
  const popup = usePopup();
  const { t } = useTranslation('Space');
  const participationAttemptedRef = useRef(false);

  // Authorization check - runs for ALL page access patterns
  useEffect(() => {
    if (participationAttemptedRef.current || participateSpace.isPending) {
      return;
    }

    const space = ctrl.space;

    if (!space) return;

    const shouldAutoParticipate = space.canParticipate;

    if (!shouldAutoParticipate) return;

    participationAttemptedRef.current = true;

    (async () => {
      try {
        await participateSpace.mutateAsync({
          spacePk: spacePk ?? '',
          verifiablePresentation: '',
        });
      } catch (err) {
        logger.debug('auto participate failed: ', err);
        console.log('auto participate failed: ', err);

        popup.open(<SpaceAuthorizePopup />).withTitle(t('authorize_title'));
      }
    })();
  }, [
    spacePk,
    ctrl.space.pk,
    ctrl.space.canParticipate,
    ctrl.space.status,
    participateSpace,
    popup,
    t,
    ctrl.space,
  ]);

  const content =
    !ctrl.space.havePreTasks() ||
    ctrl.space.isAdmin() ||
    ctrl.space.isFinished ? (
      <GeneralLayout ctrl={ctrl} />
    ) : ctrl.space.participated ? (
      <Requirements layoutCtrl={ctrl} />
    ) : (
      <></>
    );

  // NOTE: Must authorize permission for viewer/participant/admin before
  return <SafeArea>{content}</SafeArea>;
}
