import { useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import { route } from '@/route';
import { runTeamsMigration } from '@/lib/api/ratel/admin.m3';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import type { AdminMigrationsI18n } from './migrations-page-i18n';

const USER_TYPE_ADMIN = 98;

export class AdminMigrationsController {
  constructor(
    public isAdmin: boolean,
    public isLoading: boolean,
    public isRunning: boolean,
    public lastStatus: 'idle' | 'success' | 'failed',
    public runTeams: () => Promise<void>,
  ) {}
}

export function useAdminMigrationsController(i18n: AdminMigrationsI18n) {
  const { data: user, isLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;
  const [isRunning, setIsRunning] = useState(false);
  const [lastStatus, setLastStatus] = useState<'idle' | 'success' | 'failed'>(
    'idle',
  );

  useEffect(() => {
    if (!isLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, isLoading, navigate]);

  const runTeams = useMemo(
    () => async () => {
      if (isRunning) return;
      setIsRunning(true);
      setLastStatus('idle');
      try {
        await runTeamsMigration();
        setLastStatus('success');
        showSuccessToast(i18n.success);
      } catch (error) {
        console.error('Failed to run teams migration:', error);
        setLastStatus('failed');
        showErrorToast(i18n.failed);
      } finally {
        setIsRunning(false);
      }
    },
    [isRunning, i18n],
  );

  return new AdminMigrationsController(
    Boolean(isAdmin),
    isLoading,
    isRunning,
    lastStatus,
    runTeams,
  );
}
