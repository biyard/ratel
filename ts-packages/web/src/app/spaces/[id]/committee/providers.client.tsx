'use client';
import React, {
  createContext,
  Dispatch,
  SetStateAction,
  useContext,
  useState,
} from 'react';

const noop: Dispatch<SetStateAction<boolean>> = () => {};

export const CommitteeSpaceByIdContext = createContext({
  expand: false,
  setExpand: noop,
  close: true,
  setClose: noop,
  spaceId: 0,
});

export default function ClientProviders({
  children,
  spaceId,
}: {
  children: React.ReactNode;
  spaceId: number;
}) {
  const [expand, setExpand] = useState(false);
  const [close, setClose] = useState(true);

  return (
    <CommitteeSpaceByIdContext.Provider
      value={{ expand, setExpand, close, setClose, spaceId }}
    >
      {children}
    </CommitteeSpaceByIdContext.Provider>
  );
}

export function useCommitteeSpaceByIdContext() {
  return useContext(CommitteeSpaceByIdContext);
}
