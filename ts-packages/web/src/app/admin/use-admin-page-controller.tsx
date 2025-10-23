import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { useEffect } from 'react';

const USER_TYPE_ADMIN = 98;

export class AdminPageController {
  constructor(
    public isAdmin: boolean,
    public isLoading: boolean,
  ) {}
}

export function useAdminPageController() {
  const { data: user, isLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;

  useEffect(() => {
    if (!isLoading && !isAdmin) {
      // Redirect non-admin users to home
      navigate(route.home());
    }
  }, [isAdmin, isLoading, navigate]);

  return new AdminPageController(isAdmin, isLoading);
}
