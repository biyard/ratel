import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { usePopup } from '@/lib/contexts/popup-service';
import { route } from '@/route';
import { useNavigate } from 'react-router';
import { useSpaceLayoutI18n } from '../space-layout-i18n';

export default function SpaceAuthorizePopup() {
  const navigate = useNavigate();
  const popup = usePopup();
  const i18n = useSpaceLayoutI18n();
  return (
    <div className="w-100 max-tablet:w-full flex flex-col gap-10 items-center">
      <Col className="w-full gap-2.5">
        <p className="text-center wrap-break-word leading-relaxed">
          {i18n.authorize_modal_desc_1}
          <br />
          {i18n.authorize_modal_desc_2}
        </p>
      </Col>

      <Row className="w-full grid grid-cols-1">
        <Button
          variant="primary"
          onClick={() => {
            navigate(route.credentials());
            popup.close();
          }}
        >
          {i18n.authorize_modal_go_credentials}
        </Button>
      </Row>
    </div>
  );
}
