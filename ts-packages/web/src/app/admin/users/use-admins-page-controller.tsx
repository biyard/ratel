import { useState } from 'react';
import { useAdminsData } from './use-admins-data';
import { promoteToAdmin, demoteAdmin } from '@/lib/api/ratel/admin.m3';
import type { AdminUser } from '@/features/admin/types/admin-user';

export function useAdminsPageController() {
  const { admins, isLoading, error, refetch } = useAdminsData();
  const [isPromoteDialogOpen, setIsPromoteDialogOpen] = useState(false);
  const [isDemoteDialogOpen, setIsDemoteDialogOpen] = useState(false);
  const [selectedAdmin, setSelectedAdmin] = useState<AdminUser | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  const openPromoteDialog = () => {
    setActionError(null);
    setIsPromoteDialogOpen(true);
  };

  const closePromoteDialog = () => {
    setIsPromoteDialogOpen(false);
    setActionError(null);
  };

  const openDemoteDialog = (admin: AdminUser) => {
    setSelectedAdmin(admin);
    setActionError(null);
    setIsDemoteDialogOpen(true);
  };

  const closeDemoteDialog = () => {
    setIsDemoteDialogOpen(false);
    setSelectedAdmin(null);
    setActionError(null);
  };

  const handlePromoteToAdmin = async (email: string) => {
    try {
      setIsSubmitting(true);
      setActionError(null);
      await promoteToAdmin(email);
      await refetch();
      closePromoteDialog();
    } catch (err) {
      setActionError((err as Error).message || 'Failed to promote user to admin');
      throw err;
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDemoteAdmin = async () => {
    if (!selectedAdmin) return;

    try {
      setIsSubmitting(true);
      setActionError(null);
      await demoteAdmin(selectedAdmin.user_id);
      await refetch();
      closeDemoteDialog();
    } catch (err) {
      setActionError((err as Error).message || 'Failed to demote admin');
      throw err;
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    admins,
    isLoading,
    error,
    isPromoteDialogOpen,
    isDemoteDialogOpen,
    selectedAdmin,
    isSubmitting,
    actionError,
    openPromoteDialog,
    closePromoteDialog,
    openDemoteDialog,
    closeDemoteDialog,
    handlePromoteToAdmin,
    handleDemoteAdmin,
  };
}
