import { createContext, useContext } from 'react';

type SettingsContextType = {
  profileUrl: string;
  handleProfileUrl: (profile: string) => void;
  nickname: string;
  handleNickname: (evt: React.FormEvent<HTMLInputElement>) => void;
  htmlContents: string;
  handleContents: (evt: React.FormEvent<HTMLTextAreaElement>) => void;
  showWalletConnect: boolean;
  handleShowWalletConnect: (walletConnect: boolean) => void;
  handleSave: () => Promise<boolean>;
};

export const SettingsContext = createContext<SettingsContextType | undefined>(
  undefined,
);

export function useSettingsContext() {
  const context = useContext(SettingsContext);
  if (!context)
    throw new Error(
      'SettingsContext has not been provided. Please wrap your component with SettingsProvider.',
    );

  return context;
}
