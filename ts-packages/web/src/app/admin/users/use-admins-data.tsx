import { useState, useEffect } from 'react';
import { listAdmins } from '@/lib/api/ratel/admin.m3';
import type { AdminUser } from '@/features/admin/types/admin-user';

export function useAdminsData() {
  const [admins, setAdmins] = useState<AdminUser[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchAdmins = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await listAdmins();
      setAdmins(response.items);
    } catch (err) {
      setError(err as Error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchAdmins();
  }, []);

  return {
    admins,
    isLoading,
    error,
    refetch: fetchAdmins,
  };
}
