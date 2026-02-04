import { useNavigate, useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useEffect } from 'react';
import { route } from '@/route';

export function useRedirectUser(path: string | undefined = undefined) {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);
  const nav = useNavigate();

  useEffect(() => {
    if (!space.isAdmin()) {
      if (path) {
        nav(path);
      } else {
        nav(route.space(space.pk));
      }
    }
  }, [nav, space, path]);
}
