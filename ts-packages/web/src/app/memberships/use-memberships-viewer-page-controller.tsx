import { MembershipResponse } from '@/features/membership/dto/membership-response';
import { useMembershipsData } from '../admin/memberships/use-memberships-data';
import { useGetMyMemberships } from '@/features/membership/hooks/use-get-my-memberships';
import { UserMembershipResponse } from '@/features/membership/dto/user-membership-response';

export class MembershipsViewerPageController {
  constructor(
    public memberships: MembershipResponse[],
    public myMembership: UserMembershipResponse,
    public isLoading: boolean,
    public error: Error | null,
  ) {}
}

export function useMembershipsViewerPageController() {
  const { memberships, isLoading, error } = useMembershipsData();
  const { data: myMembership } = useGetMyMemberships();

  return new MembershipsViewerPageController(
    memberships,
    myMembership,
    isLoading,
    error,
  );
}
