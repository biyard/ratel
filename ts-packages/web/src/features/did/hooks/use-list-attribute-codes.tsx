import { QK_ATTRIBUTE_CODES } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';
import { useQuery } from '@tanstack/react-query';
import { AttributeCodeResponse } from '../dto/attribute-code-response';

/**
 * List all attribute codes (Admin only)
 */
export function useListAttributeCodes() {
  return useQuery({
    queryKey: [QK_ATTRIBUTE_CODES],
    queryFn: async (): Promise<ListResponse<AttributeCodeResponse>> => {
      try {
        const ret: ListResponse<unknown> = await call(
          'GET',
          '/m3/attribute-codes',
        );

        return {
          items: ret.items.map((item) => new AttributeCodeResponse(item)),
          bookmark: ret.bookmark,
        };
      } catch (e) {
        logger.error('Failed to fetch attribute codes', e);
        throw new Error(String(e));
      }
    },
  });
}
