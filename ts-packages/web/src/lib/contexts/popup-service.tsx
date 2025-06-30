'use client';
import React, {
  createContext,
  useContext,
  useRef,
  useState,
  ReactNode,
} from 'react';

type PopupConfig = {
  id?: string;
  title?: string;
  content: ReactNode;
  closable?: boolean;
  backdropClosable?: boolean;
};

type PopupServiceType = {
  popup: PopupConfig | null;
  open: (content: ReactNode) => PopupServiceType;
  withId: (id: string) => PopupServiceType;
  withTitle: (title: string) => PopupServiceType;
  withoutClose: () => PopupServiceType;
  withoutBackdropClose: () => PopupServiceType;
  close: () => void;
};

export const PopupServiceContext = createContext<PopupServiceType | null>(null);

export const PopupProvider = ({ children }: { children: ReactNode }) => {
  const [popup, setPopup] = useState<PopupConfig | null>(null);
  const popupRef = useRef<PopupConfig | null>(null);

  const service: PopupServiceType = {
    popup,

    open: (content) => {
      popupRef.current = { content };
      setPopup(popupRef.current);
      return service;
    },

    withId: (id) => {
      if (popupRef.current) {
        popupRef.current = { ...popupRef.current, id };
        setPopup({ ...popupRef.current });
      }
      return service;
    },

    withTitle: (title) => {
      if (popupRef.current) {
        popupRef.current = { ...popupRef.current, title };
        setPopup({ ...popupRef.current });
      }
      return service;
    },

    withoutClose: () => {
      if (popupRef.current) {
        popupRef.current = { ...popupRef.current, closable: false };
        setPopup({ ...popupRef.current });
      }
      return service;
    },
    withoutBackdropClose: () => {
      if (popupRef.current) {
        popupRef.current = { ...popupRef.current, backdropClosable: false };
        setPopup({ ...popupRef.current });
      }
      return service;
    },
    close: () => {
      popupRef.current = null;
      setPopup(null);
    },
  };

  return (
    <PopupServiceContext.Provider value={service}>
      {children}
    </PopupServiceContext.Provider>
  );
};

export const usePopup = () => {
  const ctx = useContext(PopupServiceContext);
  if (!ctx) throw new Error('PopupProvider is missing');
  return ctx;
};
