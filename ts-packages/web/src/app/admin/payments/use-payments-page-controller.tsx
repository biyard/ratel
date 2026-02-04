import { useState, useEffect } from 'react';
import { usePaymentsData } from './use-payments-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import type { AdminPaymentDetail } from '@/features/admin/types/admin-user';

const USER_TYPE_ADMIN = 98;
const PAGE_SIZE = 10;

export class PaymentsPageController {
  constructor(
    public payments: AdminPaymentDetail[],
    public totalCount: number,
    public isLoading: boolean,
    public error: Error | null,
    public currentPage: number,
    public totalPages: number,
    public isAdmin: boolean,
    public isCheckingAdmin: boolean,
    private setCurrentPage: (page: number) => void,
  ) {}

  get pageSize(): number {
    return PAGE_SIZE;
  }

  goToPage(page: number): void {
    if (page >= 0 && page < this.totalPages) {
      this.setCurrentPage(page);
    }
  }

  goToNextPage(): void {
    this.goToPage(this.currentPage + 1);
  }

  goToPreviousPage(): void {
    this.goToPage(this.currentPage - 1);
  }

  get hasNextPage(): boolean {
    return this.currentPage < this.totalPages - 1;
  }

  get hasPreviousPage(): boolean {
    return this.currentPage > 0;
  }
}

export function usePaymentsPageController(): PaymentsPageController {
  const [currentPage, setCurrentPage] = useState(0);
  const { data, isLoading, error } = usePaymentsData(currentPage);
  const { data: userInfo, isLoading: isUserLoading } = useUserInfo();
  const navigate = useNavigate();

  const isAdmin = userInfo?.user_type === USER_TYPE_ADMIN;
  const isCheckingAdmin = isUserLoading;

  useEffect(() => {
    if (!isUserLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isUserLoading, isAdmin, navigate]);

  return new PaymentsPageController(
    data?.payments ?? [],
    data?.totalCount ?? 0,
    isLoading,
    error,
    currentPage,
    data?.totalPages ?? 0,
    isAdmin,
    isCheckingAdmin,
    setCurrentPage,
  );
}
