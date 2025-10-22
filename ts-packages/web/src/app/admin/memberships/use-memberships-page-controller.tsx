import { useState } from 'react';
import { useMembershipsData } from './use-memberships-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { useEffect } from 'react';
import type {
  MembershipResponse,
  CreateMembershipRequest,
  UpdateMembershipRequest,
} from '@/features/membership/dto/list-memberships-response';

const USER_TYPE_ADMIN = 98;

export class MembershipsPageController {
  constructor(
    public memberships: MembershipResponse[],
    public total: number,
    public isLoading: boolean,
    public error: Error | null,
    public isFormOpen: boolean,
    public editingMembership: MembershipResponse | null,
    public deleteConfirmMembership: MembershipResponse | null,
    public openCreateForm: () => void,
    public openEditForm: (membership: MembershipResponse) => void,
    public closeForm: () => void,
    public handleCreateMembership: (
      request: CreateMembershipRequest,
    ) => Promise<void>,
    public handleUpdateMembership: (
      request: UpdateMembershipRequest,
    ) => Promise<void>,
    public openDeleteConfirm: (membership: MembershipResponse) => void,
    public closeDeleteConfirm: () => void,
    public handleDeleteMembership: () => Promise<void>,
    public isSubmitting: boolean,
  ) {}
}

export function useMembershipsPageController() {
  const { data: user, isLoading: userLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;

  const {
    memberships,
    total,
    isLoading,
    error,
    createMembership,
    updateMembership,
    deleteMembership,
    isCreating,
    isUpdating,
    isDeleting,
  } = useMembershipsData();

  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingMembership, setEditingMembership] =
    useState<MembershipResponse | null>(null);
  const [deleteConfirmMembership, setDeleteConfirmMembership] =
    useState<MembershipResponse | null>(null);

  useEffect(() => {
    if (!userLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, userLoading, navigate]);

  const openCreateForm = () => {
    setEditingMembership(null);
    setIsFormOpen(true);
  };

  const openEditForm = (membership: MembershipResponse) => {
    setEditingMembership(membership);
    setIsFormOpen(true);
  };

  const closeForm = () => {
    setIsFormOpen(false);
    setEditingMembership(null);
  };

  const handleCreateMembership = async (request: CreateMembershipRequest) => {
    try {
      await createMembership(request);
      closeForm();
    } catch (error) {
      console.error('Failed to create membership:', error);
      throw error;
    }
  };

  const handleUpdateMembership = async (request: UpdateMembershipRequest) => {
    if (!editingMembership) return;

    try {
      await updateMembership({ id: editingMembership.id, request });
      closeForm();
    } catch (error) {
      console.error('Failed to update membership:', error);
      throw error;
    }
  };

  const openDeleteConfirm = (membership: MembershipResponse) => {
    setDeleteConfirmMembership(membership);
  };

  const closeDeleteConfirm = () => {
    setDeleteConfirmMembership(null);
  };

  const handleDeleteMembership = async () => {
    if (!deleteConfirmMembership) return;

    try {
      await deleteMembership(deleteConfirmMembership.id);
      closeDeleteConfirm();
    } catch (error) {
      console.error('Failed to delete membership:', error);
      throw error;
    }
  };

  const sortedMemberships = [...memberships].sort(
    (a, b) => a.display_order - b.display_order,
  );

  return new MembershipsPageController(
    sortedMemberships,
    total,
    isLoading,
    error,
    isFormOpen,
    editingMembership,
    deleteConfirmMembership,
    openCreateForm,
    openEditForm,
    closeForm,
    handleCreateMembership,
    handleUpdateMembership,
    openDeleteConfirm,
    closeDeleteConfirm,
    handleDeleteMembership,
    isCreating || isUpdating || isDeleting,
  );
}
