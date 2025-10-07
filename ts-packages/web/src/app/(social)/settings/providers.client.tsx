'use client';

import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { userEditProfileRequest } from '@/lib/api/models/user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import React, { createContext, useContext, useState } from 'react';

type ContextType = {
  profileUrl: string;
  handleProfileUrl: (profile: string) => void;
  nickname: string;
  handleNickname: (evt: React.FormEvent<HTMLInputElement>) => void;
  htmlContents: string;
  handleContents: (evt: React.FormEvent<HTMLTextAreaElement>) => void;
  showWalletConnect: boolean;
  handleShowWalletConnect: (walletConnect: boolean) => void;
  handleSave: () => void;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const { post } = useApiCall();
  const userinfo = useSuspenseUserInfo();
  const { data: user } = userinfo;
  const [profileUrl, setProfileUrl] = useState(user?.profile_url || '');
  const [nickname, setNickname] = useState(user?.nickname || '');
  const [htmlContents, setHtmlContents] = useState(user?.description || '');
  const [showWalletConnect, setShowWalletConnect] = useState(false);
  const handleContents = (evt: React.FormEvent<HTMLTextAreaElement>) => {
    setHtmlContents(evt.currentTarget.value);
  };

  const handleNickname = (evt: React.FormEvent<HTMLInputElement>) => {
    setNickname(evt.currentTarget.value);
  };

  const handleProfileUrl = (url: string) => {
    setProfileUrl(url);
  };

  const handleShowWalletConnect = (walletConnect: boolean) => {
    setShowWalletConnect(walletConnect);
  };

  const handleSave = async () => {
    if (checkString(nickname) || checkString(htmlContents)) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    post(
      ratelApi.users.editProfile(0), // TODO: Migrate to v3 user update endpoint
      userEditProfileRequest(nickname!, htmlContents!, profileUrl),
    );
    userinfo.refetch();
  };
  return (
    <Context.Provider
      value={{
        profileUrl,
        handleProfileUrl,
        nickname,
        handleNickname,
        htmlContents,
        handleContents,
        showWalletConnect,
        handleShowWalletConnect,
        handleSave,
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function useSettingsContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
