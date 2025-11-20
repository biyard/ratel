import Card from '@/components/card';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';

export type VerifiedItemProps = React.HTMLAttributes<HTMLDivElement> & {
  Icon: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  attribute_name: string;
  attribute_value: string;
};

export default function VerifiedItem({
  attribute_name,
  attribute_value,
  Icon,
}: VerifiedItemProps) {
  return (
    <>
      <Card variant="outlined" rounded="sm" className="gap-3 p-0">
        <Row
          mainAxisAlignment="between"
          crossAxisAlignment="center"
          className="p-5 gap-[15px]"
        >
          <Icon className="w-6 [&>path]:stroke-icon" />
          <Col className="gap-1">
            <p className="text-base font-medium text-neutral-500">
              {attribute_value}
            </p>
            <p className="text-sm text-neutral-400">{attribute_name}</p>
          </Col>
        </Row>
      </Card>
    </>
  );
}
