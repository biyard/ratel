import { createContext, useContext } from 'react';
import { Outlet, useLocation, useParams, useNavigate } from 'react-router';
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

export const Context = createContext<SpaceHomeController | undefined>(
  undefined,
);

function ParticipantLayout() {
  const ctrl = useSpaceLayoutContext();
  const location = useLocation();
  const showInfo = !/\/boards\/posts(\/|$)/.test(location.pathname);

  return (
    <Row className="my-5 mx-auto w-full max-w-desktop">
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

      <Col className={cn('gap-2.5 w-full transition-all max-w-[250px]')}>
        {ctrl.actions.length > 0 && <SpaceActions actions={ctrl.actions} />}

        {ctrl.space.participated &&
          ctrl.space.participantDisplayName &&
          ctrl.space.participantProfileUrl &&
          ctrl.space.participantUsername && (
            <SpaceParticipantProfile
              displayName={ctrl.space.participantDisplayName}
              profileUrl={ctrl.space.participantProfileUrl}
              username={ctrl.space.participantUsername}
            />
          )}

        <SpaceSideMenu menus={ctrl.menus} />
        <TimelineMenu
          isEditing={false}
          handleSetting={() => {}}
          items={ctrl.timelineItems}
          titleLabel={ctrl.t('timeline_title')}
        />
      </Col>
    </Row>
  );
}

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk ?? '');
  // FIXME: use state to force re-render when hiding

  return (
    <Context.Provider value={ctrl}>
      <ParticipantLayout />
    </Context.Provider>
  );
}
