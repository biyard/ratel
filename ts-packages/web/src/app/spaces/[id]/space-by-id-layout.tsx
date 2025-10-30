import { createContext } from 'react';
import { Outlet, useParams } from 'react-router';
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

export const Context = createContext<SpaceHomeController | undefined>(
  undefined,
);

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk ?? '');

  return (
    <Context.Provider value={ctrl}>
      <Row className="my-5 mx-auto w-full max-w-desktop">
        <Col className="gap-4 w-full">
          <TitleSection
            canEdit={ctrl.isAdmin}
            title={ctrl.space.title}
            setTitle={ctrl.handleTitleChange}
            handleShare={ctrl.handleShare}
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

          <Outlet />
        </Col>
        <Col className="gap-2.5 w-full max-w-[250px]">
          {ctrl.space.isAdmin() && <SpaceActions actions={ctrl.actions} />}

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
