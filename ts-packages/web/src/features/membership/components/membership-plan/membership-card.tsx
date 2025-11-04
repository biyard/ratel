import * as React from 'react';

import { cn } from '@/lib/utils';
import { cva, VariantProps } from 'class-variance-authority';
import { MembershipPlanItem } from './i18n';
import Card from '@/components/card';
import { Col } from '@/components/ui/col';
import Heading from '@/components/ui/heading';
import { Paragraph } from '@/components/ui/paragraph';
import { Row } from '@/components/ui/row';
import { CheckboxIcon } from '@/components/icons';
import { Button } from '@/components/ui/button';

export const membershipCardVariants = cva('w-full bg-primary default-classes', {
  variants: {
    variant: {
      vertical: '',
      horizontal: '',
    },
  },
  defaultVariants: {
    variant: 'vertical',
  },
});

export type MembershipCardProps = React.ComponentProps<'button'> &
  VariantProps<typeof membershipCardVariants> & {
    asChild?: boolean;
  } & {
    membership: MembershipPlanItem;
  };

export function MembershipCard({ variant, ...props }: MembershipCardProps) {
  if (variant === 'vertical') {
    return <MembershipVerticalCard {...props} />;
  }

  return <MembershipHorizontalCard {...props} />;
}

function MembershipVerticalCard({ membership, onClick }: MembershipCardProps) {
  return (
    <Card className={cn('col-span-4 md:col-span-2 lg:col-span-1 min-h-140')}>
      <Col className="gap-5 h-full">
        <Heading variant="heading5">{membership.name}</Heading>
        <Paragraph variant="strong">{membership.description}</Paragraph>
        <Col className="flex-1 gap-3">
          {membership.features.map((feature) => (
            <Row key={feature} crossAxisAlignment="center" className="gap-3">
              <CheckboxIcon className="min-w-3 [&>path]:stroke-primary" />
              {feature}
            </Row>
          ))}
        </Col>
        {membership.price && (
          <Paragraph variant="strong">{membership.price}</Paragraph>
        )}
        {membership.btn && (
          <Row mainAxisAlignment="end">
            <Button variant="rounded_secondary" onClick={onClick}>
              {membership.btn}
            </Button>
          </Row>
        )}
      </Col>
    </Card>
  );
}

function MembershipHorizontalCard({
  membership,
  onClick,
}: MembershipCardProps) {
  return (
    <Card className={cn('col-span-full min-h-70')}>
      <Col className="gap-5 h-full">
        <Heading variant="heading5">{membership.name}</Heading>
        <Paragraph variant="strong">{membership.description}</Paragraph>
        <Col className="flex-1 gap-3">
          {membership.features.map((feature) => (
            <Row key={feature} crossAxisAlignment="center" className="gap-3">
              <CheckboxIcon className="min-w-3 [&>path]:stroke-primary" />
              {feature}
            </Row>
          ))}
        </Col>
        {membership.price && (
          <Row mainAxisAlignment="between">
            <Paragraph variant="strong">{membership.price}</Paragraph>
            <Button variant="rounded_secondary" onClick={onClick}>
              {membership.btn}
            </Button>
          </Row>
        )}
      </Col>
    </Card>
  );
}
