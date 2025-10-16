import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';

export function useSpaceHomeData(spacePk: string) {
  return {
    space: useSpaceById(spacePk),
    user: useSuspenseUserInfo(),
  };
}
