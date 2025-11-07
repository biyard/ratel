import { useState, useEffect } from 'react';
import { useAttributeCodesData } from './use-attribute-codes-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import type { AttributeCodeResponse } from '@/features/did/dto/attribute-code-response';
import type { CreateAttributeCodeRequest } from '@/features/did/dto/create-attribute-code-request';

const USER_TYPE_ADMIN = 98;

export class AttributeCodesPageController {
  constructor(
    public attributeCodes: AttributeCodeResponse[],
    public total: number,
    public isLoading: boolean,
    public error: Error | null,
    public isFormOpen: boolean,
    public deleteConfirmCode: AttributeCodeResponse | null,
    public openCreateForm: () => void,
    public closeForm: () => void,
    public handleCreateAttributeCode: (
      request: CreateAttributeCodeRequest,
    ) => Promise<void>,
    public openDeleteConfirm: (code: AttributeCodeResponse) => void,
    public closeDeleteConfirm: () => void,
    public handleDeleteAttributeCode: () => Promise<void>,
    public isSubmitting: boolean,
  ) {}
}

export function useAttributeCodesPageController() {
  const { data: user, isLoading: userLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;

  const {
    attributeCodes,
    total,
    isLoading,
    error,
    createAttributeCode,
    deleteAttributeCode,
    isCreating,
    isDeleting,
  } = useAttributeCodesData();

  const [isFormOpen, setIsFormOpen] = useState(false);
  const [deleteConfirmCode, setDeleteConfirmCode] =
    useState<AttributeCodeResponse | null>(null);

  useEffect(() => {
    if (!userLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, userLoading, navigate]);

  const openCreateForm = () => {
    setIsFormOpen(true);
  };

  const closeForm = () => {
    setIsFormOpen(false);
  };

  const handleCreateAttributeCode = async (
    request: CreateAttributeCodeRequest,
  ) => {
    try {
      await createAttributeCode(request);
      closeForm();
    } catch (error) {
      console.error('Failed to create attribute code:', error);
      throw error;
    }
  };

  const openDeleteConfirm = (code: AttributeCodeResponse) => {
    setDeleteConfirmCode(code);
  };

  const closeDeleteConfirm = () => {
    setDeleteConfirmCode(null);
  };

  const handleDeleteAttributeCode = async () => {
    if (!deleteConfirmCode) return;

    try {
      await deleteAttributeCode(deleteConfirmCode.pk);
      closeDeleteConfirm();
    } catch (error) {
      console.error('Failed to delete attribute code:', error);
      throw error;
    }
  };

  const sortedCodes = [...attributeCodes].sort(
    (a, b) => b.created_at - a.created_at,
  );

  return new AttributeCodesPageController(
    sortedCodes,
    total,
    isLoading,
    error,
    isFormOpen,
    deleteConfirmCode,
    openCreateForm,
    closeForm,
    handleCreateAttributeCode,
    openDeleteConfirm,
    closeDeleteConfirm,
    handleDeleteAttributeCode,
    isCreating || isDeleting,
  );
}
