import { MembershipResponse } from '@/features/membership/dto/membership-response';
import { useMembershipsData } from '../admin/memberships/use-memberships-data';

export class MembershipsViewerPageController {
  constructor(
    public memberships: MembershipResponse[],
    public isLoading: boolean,
    public error: Error | null,
  ) {}
}

export function useMembershipsViewerPageController() {
  const { memberships, isLoading, error } = useMembershipsData();

  return new MembershipsViewerPageController(memberships, isLoading, error);
}
