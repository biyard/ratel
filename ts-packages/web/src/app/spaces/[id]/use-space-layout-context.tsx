import { useContext } from 'react';
import { Context } from './space-by-id-layout';
import { SpaceHomeController } from './use-space-home-controller';

export function useSpaceLayoutContext(): SpaceHomeController {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
