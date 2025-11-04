'use client';

import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { updateUserProfile } from '@/lib/api/ratel/me.v3';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useState } from 'react';
import { SettingsContext } from './settings-context';

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
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

  const handleSave = async (): Promise<boolean> => {
    if (checkString(nickname) || checkString(htmlContents)) {
      showErrorToast('Please remove the test keyword');
      return false;
    }

    // Validate nickname (display name)
    const trimmedNickname = nickname.trim();
    if (trimmedNickname.length < 1 || trimmedNickname.length > 30) {
      showErrorToast('Display name must be between 1 and 30 characters');
      return false;
    }

    // Check word count (max 2 words)
    const wordCount = trimmedNickname
      .split(/\s+/)
      .filter((w) => w.length > 0).length;
    if (wordCount > 2) {
      showErrorToast('Display name must be at most 2 words');
      return false;
    }

    // Validate description (min 10 characters)
    if (htmlContents && htmlContents.trim().length < 10) {
      showErrorToast('Description must be at least 10 characters');
      return false;
    }

    try {
      await updateUserProfile(trimmedNickname, profileUrl, htmlContents!);

      // Refetch user info to get updated data
      await userinfo.refetch();

      // Show success message
      showSuccessToast('Profile updated successfully!');
      return true;
    } catch (error) {
      showErrorToast('Failed to update profile');
      console.error('Failed to update user profile:', error);
      return false;
    }
  };
  return (
    <SettingsContext.Provider
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
    </SettingsContext.Provider>
  );
}
