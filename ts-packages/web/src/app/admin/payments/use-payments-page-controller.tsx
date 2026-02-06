import { useEffect, useCallback } from 'react';
import usePaymentsData from './use-payments-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { type AdminPaymentResponse } from '@/features/admin/types/admin-user';
import { UserType } from '@/lib/api/ratel/users.v3';

export class PaymentsPageController {
  constructor(
    public payments: AdminPaymentResponse[],
    public isLoading: boolean,
    public error: Error | null,
    public isAdmin: boolean,
    public isCheckingAdmin: boolean,
    public hasNextPage: boolean,
    public isFetchingNextPage: boolean,
    public fetchNextPage: () => void,
  ) {}
}

export function usePaymentsPageController(): PaymentsPageController {
  const {
    data,
    isLoading,
    error,
    hasNextPage,
    isFetchingNextPage,
    fetchNextPage,
  } = usePaymentsData();
  const { data: userInfo, isLoading: isUserLoading } = useUserInfo();
  const navigate = useNavigate();

  const isAdmin = userInfo?.user_type === UserType.Admin;
  const isCheckingAdmin = isUserLoading;

  useEffect(() => {
    if (!isUserLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isUserLoading, isAdmin, navigate]);

  const payments = data?.pages.flatMap((page) => page.items) ?? [];

  const handleFetchNextPage = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [hasNextPage, isFetchingNextPage, fetchNextPage]);

  return new PaymentsPageController(
    payments,
    isLoading,
    error,
    isAdmin,
    isCheckingAdmin,
    hasNextPage ?? false,
    isFetchingNextPage,
    handleFetchNextPage,
  );
}
