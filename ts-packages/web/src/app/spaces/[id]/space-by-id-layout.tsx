import { Outlet, useParams } from 'react-router';
import { useSpaceHomeController } from './use-space-home-controller';
import { Row } from '@/components/ui/row';
import SpaceSideMenu from '@/features/spaces/components/space-side-menu';
import { Col } from '@/components/ui/col';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk);

  return (
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
          rewards={ctrl.space.rewards}
          isDraft={ctrl.space.isDraft}
          isPublic={ctrl.space.isPublic}
        />

        <Outlet />
      </Col>
      <SpaceSideMenu menus={ctrl.menus} />
    </Row>
  );
}
