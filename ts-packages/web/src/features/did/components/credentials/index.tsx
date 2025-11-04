import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { useController } from './use-controller';
import { Verified } from '@/components/icons';
import { useCredentialsI18n } from './i18n';
import Heading from '@/components/ui/heading';
import { Button } from '@/components/ui/button';
import Card from '@/components/card';
import VerifiedItem from './verified_item';

export function Credentials() {
  const ctrl = useController();
  const t = useCredentialsI18n();

  return (
    <>
      <Col className="gap-4">
        {/* Verifiable Credential Card */}
        <Col
          mainAxisAlignment="center"
          crossAxisAlignment="center"
          className="overflow-hidden relative py-6 gap-[17.5px]"
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
              {t.id}: {ctrl.did}
            </p>
          </Col>
        </Col>

        {/* My DID Section Header */}
        <Col rounded="default" padding="sm" className="gap-5 bg-component-bg">
          <Heading variant="heading6">{t.my_did}</Heading>

          {ctrl.attributes.map((attr) => (
            <VerifiedItem {...attr} />
          ))}
          {ctrl.attributes.length === 0 && (
            <Card variant="outlined" className="flex items-center">
              {t.no_data}
            </Card>
          )}

          <Row mainAxisAlignment="center" className="">
            <Button
              variant="text"
              className="text-primary"
              onClick={ctrl.handleVerify}
            >
              {t.verify}
            </Button>
          </Row>
        </Col>
      </Col>
    </>
  );
}
