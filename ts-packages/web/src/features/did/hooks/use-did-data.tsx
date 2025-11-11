import { QK_GET_DID } from '@/constants';
import { getDid } from '@/lib/api/ratel/me.v3';
import { DidDocument } from '@/features/did/types/did-document';
import {
  QueryClient,
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useDidData(): UseQueryResult<DidDocument | null> {
  const query = useQuery({
    queryKey: [QK_GET_DID],
    queryFn: async () => {
      try {
        const did = await getDid();
        return did;
      } catch {
        return null;
      }
    },
  });

  return query;
}

export function useSuspenseDidData(): UseSuspenseQueryResult<DidDocument | null> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_DID],
    queryFn: async () => {
      try {
        const did = await getDid();
        return did;
      } catch {
        return null;
      }
    },
  });

  return query;
}

export function invalidateDidData(queryClient: QueryClient) {
  queryClient.invalidateQueries({
    queryKey: [QK_GET_DID],
  });
}

export function removeDidData(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: [QK_GET_DID] });
}

export function refetchDidData(queryClient: QueryClient) {
  queryClient.refetchQueries({ queryKey: [QK_GET_DID] });
}
