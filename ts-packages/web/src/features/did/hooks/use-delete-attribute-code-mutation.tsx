import { QK_ATTRIBUTE_CODES } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { DeleteAttributeCodeResponse } from '../dto/delete-attribute-code-response';

/**
 * Delete an attribute code (Admin only)
 */
export function useDeleteAttributeCodeMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (codePk: string): Promise<DeleteAttributeCodeResponse> =>
      call('DELETE', `/m3/attribute-codes/${encodeURIComponent(codePk)}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_ATTRIBUTE_CODES] });
    },
  });
}
