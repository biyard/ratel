import { Suspense, useState } from 'react';
import { Outlet, useParams, useMatches } from 'react-router';
import { Row } from '@/components/ui/row';
import { cn } from '@/lib/utils';

import { Col } from '@/components/ui/col';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';

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
import { Hamburger } from '@/components/icons';
import Logo from '@/assets/icons/logo/logo-letter.svg?react';
import { Button } from '@/components/ui/button';
import TimelineMenu from './components/timeline';

function GeneralLayout({ ctrl }: { ctrl: SpaceLayoutController }) {
  const matches = useMatches();
  const hideSpaceHeader = matches.some(
    (match) =>
      match.handle &&
      (match.handle as { hideSpaceHeader?: boolean }).hideSpaceHeader,
  );
  const isAdmin = ctrl.role == Role.Admin;

  // Mobile Menu State
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  // Determine role for ActionCard
  return (
    <Row
      data-testid="space-layout-root"
      className="flex flex-row items-start gap-0 flex-nowrap h-screen relative"
    >
      {/* Mobile Header - Only visible on mobile */}
      <div className="tablet:hidden fixed top-0 left-0 right-0 z-50 bg-bg border-b border-divider">
        <div className="grid grid-cols-3 items-center px-4 py-3">
          <button
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            aria-label="Toggle menu"
            className="p-2 justify-self-start"
          >
            <Hamburger className="size-6 *:stroke-text-primary" />
          </button>

          <button
            onClick={ctrl.handleBackToHome}
            className="justify-self-center w-full"
          >
            <Logo className="w-full" />
          </button>

          <div />
        </div>
      </div>

      {/* Backdrop overlay - Only on mobile when menu is open */}
      {isMobileMenuOpen && (
        <div
          className="tablet:hidden fixed inset-0 bg-black/50 z-30"
          onClick={() => setIsMobileMenuOpen(false)}
        />
      )}

      {/* Left Sidebar - Responsive */}
      <Col
        className={cn(
          'max-w-[250px] shrink-0 bg-component-bg flex flex-col divide-y divide-divider py-2.5',
          // Mobile: fixed positioning with slide animation, below mobile header
          'fixed tablet:relative top-14 tablet:top-0 left-0 z-40',
          'h-[calc(100vh-3.5rem)] tablet:h-full',
          'transition-transform duration-300 ease-in-out',
          isMobileMenuOpen
            ? 'translate-x-0'
            : '-translate-x-full tablet:translate-x-0',
        )}
      >
        <div className="flex-1 overflow-y-auto flex flex-col gap-4 px-3 divide-y divide-divider">
          <button
            className="px-4 py-5 hidden tablet:block"
            onClick={ctrl.handleBackToHome}
          >
            <Logo className="w-[95px] h-[35px]" />
          </button>
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

          <SpaceSideMenu
            menus={ctrl.menus}
            selectedMenu={ctrl.currentMenu}
            onMenuClick={() => setIsMobileMenuOpen(false)}
          />

          <TimelineMenu
            isEditing={false}
            handleSetting={() => {}}
            items={ctrl.timelineItems}
            titleLabel={ctrl.i18n.timeline_title}
          />
        </div>

        <div className="shrink-0 p-3">
          {ctrl.profile ? (
            <SpaceParticipantProfile
              profileUrl={ctrl.profile.profileUrl}
              displayName={ctrl.profile.displayName}
              username={ctrl.profile.username}
            />
          ) : (
            <Button
              variant="primary"
              className="w-full"
              onClick={ctrl.handleLogin}
            >
              {ctrl.i18n.login}
            </Button>
          )}
        </div>
      </Col>

      {/* Main Content */}
      <Col className="flex-1 h-full overflow-y-auto">
        <Col className="gap-4 min-w-0 w-full max-w-desktop mx-auto p-2 pt-5">
          {/* Title, Author - Sticky Header */}
          {!hideSpaceHeader && (
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

  // NOTE: Must authorize permission for viewer/participant/admin before
  return (
    <SafeArea>
      <GeneralLayout ctrl={ctrl} />
    </SafeArea>
  );
}
