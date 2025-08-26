import { SpaceStatus } from '@/lib/api/models/spaces';
import { UserType } from '@/lib/api/models/user';
import { createContext, useContext } from 'react';

export interface SpaceContextType {
  isEdit: boolean;
  title: string;
  status: SpaceStatus;
  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;

  handleGoBack: () => void;
  handleSave: () => void;
  handleEdit: () => void;
  handleShare: () => void;
  handlePostingSpace: () => Promise<void>;
  handleUpdateTitle: (value: string) => void;
  handleDelete: () => Promise<void>;
}

export const Context = createContext<SpaceContextType | undefined>(undefined);

export function SpaceProvider({
  value,
  children,
}: {
  value: SpaceContextType;
  children: React.ReactNode;
}) {
  return <Context.Provider value={value}>{children}</Context.Provider>;
}

export function useSpaceContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}
