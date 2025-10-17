import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';

export function useSpaceSettingsData(spacePk: string) {
  return {
    space: useSpaceById(spacePk),
  };
}
