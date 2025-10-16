import { Outlet, useParams } from 'react-router';
import { useSpaceHomeController } from './use-space-home-controller';
import { Row } from '@/components/ui/row';
import SpaceSideMenu from '@/features/spaces/components/space-side-menu';
import { Col } from '@/components/ui/col';

export default function SpaceByIdLayout() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk);

  return (
    <Row className="my-5 mx-auto w-full max-w-desktop">
      <Col className="gap-4 w-full">
        <Outlet />
      </Col>
      <SpaceSideMenu menus={ctrl.menus} />
    </Row>
  );
}
