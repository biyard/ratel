import { createContext, useContext } from 'react';
import { SpaceHeaderContextType } from './provider';

export const SpaceHeaderContext = createContext<SpaceHeaderContextType | null>(
  null,
);

export const useSpaceHeaderContext = (): SpaceHeaderContextType => {
  const context = useContext(SpaceHeaderContext);
  if (!context) {
    throw new Error(
      'useSpaceHeaderContext must be used within a SpaceHeaderProvider',
    );
  }
  return context;
};
