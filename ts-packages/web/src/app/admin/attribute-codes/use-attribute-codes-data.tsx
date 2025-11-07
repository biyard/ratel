import { useListAttributeCodes } from '@/features/did/hooks/use-list-attribute-codes';
import { useCreateAttributeCodeMutation } from '@/features/did/hooks/use-create-attribute-code-mutation';
import { useDeleteAttributeCodeMutation } from '@/features/did/hooks/use-delete-attribute-code-mutation';

export function useAttributeCodesData() {
  const { data, isLoading, error, refetch } = useListAttributeCodes();

  const createMutation = useCreateAttributeCodeMutation();
  const deleteMutation = useDeleteAttributeCodeMutation();

  return {
    attributeCodes: data?.items || [],
    total: data?.items.length || 0,
    isLoading,
    error,
    refetch,
    createAttributeCode: createMutation.mutateAsync,
    deleteAttributeCode: deleteMutation.mutateAsync,
    isCreating: createMutation.isPending,
    isDeleting: deleteMutation.isPending,
  };
}
