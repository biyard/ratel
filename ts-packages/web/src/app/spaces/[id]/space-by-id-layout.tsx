import { createContext } from 'react';
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

export const Context = createContext<SpaceHomeController | undefined>(
  undefined,
);

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk ?? '');
  const location = useLocation();
  const showInfo = !/\/boards\/posts(\/|$)/.test(location.pathname);

  // Check if prerequisites are completed
  /* const { data: prerequisites, isLoading: isLoadingPrerequisites } =
   *   useCheckPrerequisites(spacePk ?? ''); */

  // Redirect to poll if prerequisites are not completed
  /* useEffect(() => {
   *   if (
   *     !isLoadingPrerequisites &&
   *     prerequisites &&
   *     !prerequisites.completed &&
   *     prerequisites.poll_pk
   *   ) {
   *     // Only redirect if not already on the poll page
   *     const pollPagePattern = new RegExp(
   *       `/spaces/${encodeURIComponent(spacePk ?? '')}/polls/${encodeURIComponent(prerequisites.poll_pk)}`,
   *     );
   *     if (!pollPagePattern.test(location.pathname)) {
   *       navigate(route.spacePollById(spacePk ?? '', prerequisites.poll_pk));
   *     }
   *   }
   * }, [
   *   prerequisites,
   *   isLoadingPrerequisites,
   *   spacePk,
   *   location.pathname,
   *   navigate,
   * ]); */

  // Check if we should show side menu and content
  /* const shouldShowSideMenu =
   *   isLoadingPrerequisites || !prerequisites || prerequisites.completed; */

  return (
    <Context.Provider value={ctrl}>
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
        <Col className="gap-2.5 w-full max-w-[250px]">
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
    </Context.Provider>
  );
}
