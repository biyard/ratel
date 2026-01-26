import { useQuery } from '@tanstack/react-query';
import {
  getUserMembership,
  getPurchaseHistory,
  UserMembershipResponse,
  ListPurchaseHistoryResponse,
} from '@/lib/api/ratel/me.v3';

export interface MembershipData {
  membership: UserMembershipResponse | undefined;
  purchaseHistory: ListPurchaseHistoryResponse | undefined;
  isLoadingMembership: boolean;
  isLoadingHistory: boolean;
}

export function useMembershipData(): MembershipData {
  const { data: membership, isLoading: isLoadingMembership } = useQuery({
    queryKey: ['user-membership'],
    queryFn: getUserMembership,
  });

  const { data: purchaseHistory, isLoading: isLoadingHistory } = useQuery({
    queryKey: ['purchase-history'],
    queryFn: () => getPurchaseHistory(),
  });

  return {
    membership,
    purchaseHistory,
    isLoadingMembership,
    isLoadingHistory,
  };
}
