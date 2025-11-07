import { QK_ATTRIBUTE_CODES } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { CreateAttributeCodeRequest } from '../dto/create-attribute-code-request';
import { call } from '@/lib/api/ratel/call';

/**
 * Create a new attribute code (Admin only)
 */
export function useCreateAttributeCodeMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: CreateAttributeCodeRequest) =>
      call('POST', '/m3/attribute-codes', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_ATTRIBUTE_CODES] });
    },
  });
}
