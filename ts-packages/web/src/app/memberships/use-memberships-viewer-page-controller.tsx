import { MembershipResponse } from '@/features/membership/dto/membership-response';
import { useMembershipsData } from '../admin/memberships/use-memberships-data';
import { useGetMyMemberships } from '@/features/membership/hooks/use-get-my-memberships';
import { UserMembershipResponse } from '@/features/membership/dto/user-membership-response';
import { PurchaseMembershipRequest } from '@/features/membership/dto/purchase-membership-request';

export class MembershipsViewerPageController {
  constructor(
    public memberships: MembershipResponse[],
    public myMembership: UserMembershipResponse,
    public isLoading: boolean,
    public error: Error | null,
    public handlePurchaseMembership: (
      request: PurchaseMembershipRequest,
    ) => Promise<void>,
  ) {}
}

export function useMembershipsViewerPageController() {
  const { memberships, isLoading, error, purchaseMembership } =
    useMembershipsData();
  const { data: myMembership } = useGetMyMemberships();

  const toH5 = (deeplink: string) => {
    const https = deeplink.replace(/^bnc:\/\//, 'https://');
    const u = new URL(https);

    u.hostname = 'app.binance.com';

    let path = u.pathname;
    if (!path.startsWith('/en/')) {
      path = '/en' + (path.startsWith('/') ? path : `/${path}`);
    }
    u.pathname = path;

    return u.toString();
  };

  const handlePurchaseMembership = async (
    request: PurchaseMembershipRequest,
  ) => {
    const data = await purchaseMembership(request);

    if (data?.deeplink) {
      window.location.assign(toH5(data.deeplink));
      return;
    }
    if (data?.checkout_url) {
      window.location.assign(data.checkout_url);
      return;
    }
    if (data?.qr_content) {
      window.open(data.qr_content, '_blank', 'noopener,noreferrer');
      return;
    }
    if (data?.qrcode_link) {
      window.open(data.qrcode_link, '_blank', 'noopener,noreferrer');
      return;
    }
  };

  return new MembershipsViewerPageController(
    memberships,
    myMembership,
    isLoading,
    error,
    handlePurchaseMembership,
  );
}
