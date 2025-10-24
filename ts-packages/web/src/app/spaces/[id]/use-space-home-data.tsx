import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';

export function useSpaceHomeData(spacePk: string) {
  return {
    space: useSpaceById(spacePk),
    user: useSuspenseUserInfo(),
  };
}
