import { useMembershipData, MembershipData } from './use-membership-data';
import { useMembershipPageI18n, MembershipPageI18n } from './membership-page-i18n';

export interface MembershipPageController extends MembershipData {
  t: MembershipPageI18n;
  formatDate: (timestamp: number) => string;
  formatTier: (tier: string) => string;
}

export function useMembershipPageController(): MembershipPageController {
  const data = useMembershipData();
  const t = useMembershipPageI18n();

  const formatDate = (timestamp: number): string => {
    if (timestamp === Number.MAX_SAFE_INTEGER) {
      return t.unlimited;
    }
    return new Date(timestamp / 1000).toLocaleDateString();
  };

  const formatTier = (tier: string): string => {
    return tier.replace('MEMBERSHIP#', '');
  };

  return {
    ...data,
    t,
    formatDate,
    formatTier,
  };
}
