import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { useController } from './use-controller';
import { Verified, Did } from '@/components/icons';
import { useCredentialsI18n } from './i18n';
import Heading from '@/components/ui/heading';
import { Button } from '@/components/ui/button';

export function Credentials() {
  const ctrl = useController();
  const t = useCredentialsI18n();

  // Truncate DID for display
  const truncateDid = (did: string) => {
    if (did.length <= 20) return did;
    return `${did.substring(0, 20)}...`;
  };

  return (
    <>
      <Col className="gap-4">
        {/* Verifiable Credential Card */}
        <Col
          mainAxisAlignment="center"
          crossAxisAlignment="center"
          className="py-6 gap-[17.5px] relative overflow-hidden"
          rounded="default"
          padding="sm"
          style={{
            background:
              'radial-gradient(circle at center, rgba(77, 92, 255, 0.5) 0%, rgba(30, 30, 30, 1) 100%)',
          }}
        >
          <Verified className="w-20 h-20" />
          <Col
            mainAxisAlignment="center"
            crossAxisAlignment="center"
            className="gap-1"
          >
            <Heading variant="heading4">{t.vc}</Heading>
            <p className="text-sm text-neutral-300">
              {t.id}: {truncateDid(ctrl.did)}
            </p>
          </Col>
        </Col>

        {/* My DID Section Header */}
        <Col rounded="default" padding="sm" className="bg-component-bg">
          <Heading variant="heading5">{t.my_did}</Heading>
        </Col>

        {/* Age Verification Card */}
        <Col rounded="default" padding="sm" className="bg-component-bg gap-3">
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <Col className="gap-1">
              <p className="text-base font-medium">{t.age_range}</p>
              <Row crossAxisAlignment="center" className="gap-1">
                <Verified className="w-4 h-4" />
                <p className="text-sm text-neutral-400">{t.verified}</p>
              </Row>
            </Col>
            <Col crossAxisAlignment="end" className="gap-1">
              <Did className="w-6 h-6" />
            </Col>
          </Row>
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <p className="text-sm text-neutral-400">{t.age}</p>
            <p className="text-sm text-neutral-400">0 {t.kaia}</p>
          </Row>
        </Col>

        {/* Gender Verification Card */}
        <Col rounded="default" padding="sm" className="bg-component-bg gap-3">
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <Col className="gap-1">
              <p className="text-base font-medium">{t.registration_required}</p>
            </Col>
            <Col crossAxisAlignment="end" className="gap-1">
              <Did className="w-6 h-6" />
            </Col>
          </Row>
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <p className="text-sm text-neutral-400">{t.gender}</p>
            <p className="text-sm text-neutral-400">0 {t.kaia}</p>
          </Row>
          <Button variant="default" size="sm" className="w-full">
            {t.verify}
          </Button>
        </Col>
      </Col>
    </>
  );
}
