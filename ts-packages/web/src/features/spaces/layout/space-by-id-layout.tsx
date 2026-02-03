import { Suspense } from 'react';
import { Outlet, useParams, useMatches } from 'react-router';
import { Row } from '@/components/ui/row';

import { Col } from '@/components/ui/col';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';
import TimelineMenu from '@/features/spaces/components/side-menu/timeline';

import {
  useSpaceLayoutController,
  SpaceLayoutController,
  Role,
} from '@/features/spaces/layout/use-space-layout-controller';
import { ErrorBoundary } from '@/components/error-boundary';
import { SafeArea } from '@/components/ui/safe-area';
import SpaceParticipantProfile from './components/space-participant-profile';
import SpaceSideMenu from './components/space-side-menu';

import AdminActionCard from './components/admin-action-card';
import ViewerActionCard from './components/viewer-action-card';

function GeneralLayout({ ctrl }: { ctrl: SpaceLayoutController }) {
  const matches = useMatches();
  const hideHeader = matches.some(
    (match) =>
      match.handle && (match.handle as { hideHeader?: boolean }).hideHeader,
  );
  const isAdmin = ctrl.role == Role.Admin;

  // Determine role for ActionCard
  return (
    <Row
      data-testid="space-layout-root"
      className="flex flex-row items-start gap-0 flex-nowrap h-[calc(100vh-var(--header-height))]"
    >
      {/* Left Sidebar - Fixed */}
      <Col className="max-w-[250px] shrink-0 bg-component-bg flex flex-col h-full divide-y divide-divider py-2">
        <div className="flex-1 overflow-y-auto flex flex-col gap-4 px-3 divide-y divide-divider">
          {ctrl.role == Role.Admin && (
            <AdminActionCard
              title={ctrl.i18n.admin_title}
              description={ctrl.i18n.admin_description}
              actions={ctrl.adminActions}
            />
          )}
          {ctrl.role == Role.Viewer && (
            <ViewerActionCard
              title={ctrl.i18n.viewer_title}
              description={ctrl.i18n.viewer_description}
              verifiedCredentials={[]}
              actions={ctrl.viewerActions}
            />
          )}

          <SpaceSideMenu menus={ctrl.menus} selectedMenu={ctrl.currentMenu} />

          <div className="flex flex-col" />

          <TimelineMenu
            isEditing={false}
            handleSetting={() => {}}
            items={ctrl.timelineItems}
            titleLabel={ctrl.i18n.timeline_title}
          />
        </div>

        <div className="shrink-0">
          {ctrl.profile && (
            <SpaceParticipantProfile
              profileUrl={ctrl.profile.profileUrl}
              displayName={ctrl.profile.displayName}
              username={ctrl.profile.username}
            />
          )}
        </div>
      </Col>

      {/* Main Content */}
      <Col className="flex-1 h-full overflow-y-auto">
        <Col className="gap-4 min-w-0 max-w-desktop mx-auto p-2">
          {/* Title, Author - Sticky Header */}
          {!hideHeader && (
            <div className="sticky top-0 bg-bg z-10 pb-4">
              <Col className="gap-10 w-full min-w-0">
                <TitleSection
                  canEdit={isAdmin}
                  title={ctrl.space.title}
                  setTitle={ctrl.handleTitleChange}
                />
                <Row className="flex justify-around">
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
                </Row>
              </Col>
            </div>
          )}
          {/* Outlet - Scrollable */}
          <ErrorBoundary>
            <Suspense fallback={<div className="p-4">Loading...</div>}>
              <Outlet />
            </Suspense>
          </ErrorBoundary>
        </Col>
      </Col>
    </Row>
  );
}

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceLayoutController(spacePk!);
  // const participateSpace = useParticipateSpaceMutation();
  // const popup = usePopup();
  // const { t } = useTranslation('Space');
  // const participationAttemptedRef = useRef(false);

  // useEffect(() => {
  //   if (participationAttemptedRef.current || participateSpace.isPending) {
  //     return;
  //   }

  //   const space = ctrl.space;

  //   if (!space) return;

  //   const shouldAutoParticipate = space.canParticipate;

  //   if (!shouldAutoParticipate) return;

  //   participationAttemptedRef.current = true;

  //   (async () => {
  //     try {
  //       await participateSpace.mutateAsync({
  //         spacePk: spacePk ?? '',
  //         verifiablePresentation: '',
  //       });
  //     } catch (err) {
  //       logger.debug('auto participate failed: ', err);
  //       console.log('auto participate failed: ', err);

  //       popup.open(<SpaceAuthorizePopup />).withTitle(t('authorize_title'));
  //     }
  //   })();
  // }, [
  //   spacePk,
  //   ctrl.space.pk,
  //   ctrl.space.canParticipate,
  //   ctrl.space.status,
  //   participateSpace,
  //   popup,
  //   t,
  //   ctrl.space,
  // ]);

  // NOTE: Must authorize permission for viewer/participant/admin before
  return (
    <SafeArea>
      <GeneralLayout ctrl={ctrl} />
    </SafeArea>
  );
}
