import { Col } from '@/components/ui/col';
import { useController } from './use-controller';
import Heading from '@/components/ui/heading';
import { SafeArea } from '@/components/ui/safe-area';
import { MembershipCard } from './membership-card';

export function MembershipPlan() {
  const ctrl = useController();

  const memberships = ctrl.t.memberships.map((membership, i) => {
    return (
      <MembershipCard
        key={membership.name}
        membership={membership}
        variant={
          i === ctrl.t.memberships.length - 1 ? 'horizontal' : 'vertical'
        }
        onClick={() => ctrl.handleGetMembership(i)}
      />
    );
  });

  return (
    <>
      <SafeArea>
        <MembershipPlanHeader {...ctrl} />
        <div className="grid grid-cols-4 gap-2.5">{memberships}</div>
      </SafeArea>
    </>
  );
}

export function MembershipPlanHeader(ctrl) {
  return (
    <>
      <Col
        mainAxisAlignment="start"
        crossAxisAlignment="center"
        className="gap-2.5"
      >
        <Heading>{ctrl.t.title}</Heading>
        <div dangerouslySetInnerHTML={ctrl.t.description} />
      </Col>
    </>
  );
}
