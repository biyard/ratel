import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { usePopup } from '@/lib/contexts/popup-service';
import { route } from '@/route';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';

export default function SpaceEndPopup() {
  const navigate = useNavigate();
  const popup = usePopup();
  const { t } = useTranslation('Space');
  return (
    <div className="w-100 max-tablet:w-full flex flex-col gap-10 items-center">
      <Col className="w-full gap-2.5">
        <p className="text-center break-words leading-relaxed">
          {t('end_space_desc')}
        </p>
      </Col>

      <Row className="w-full grid grid-cols-1">
        <Button
          variant="primary"
          onClick={() => {
            navigate(route.home());
            popup.close();
          }}
        >
          {t('go_home')}
        </Button>
      </Row>
    </div>
  );
}
