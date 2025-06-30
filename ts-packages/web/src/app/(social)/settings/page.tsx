'use client';

import FileUploader from '@/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { userEditProfileRequest } from '@/lib/api/models/user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { route } from '@/route';
import { useRouter } from 'next/navigation';
import React, { useState } from 'react';
import WalletSummary from './_components/wallet-summary';
import Image from 'next/image';
import { logger } from '@/lib/logger';

export default function MyProfilePage() {
  const { post } = useApiCall();
  const userinfo = useSuspenseUserInfo();
  const { data: user } = userinfo;
  const router = useRouter();

  const [profileUrl, setProfileUrl] = useState(user?.profile_url || '');
  const [nickname, setNickname] = useState(user?.nickname);
  const [htmlContents, setHtmlContents] = useState(user?.html_contents);
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

  const handleSave = async () => {
    if (checkString(nickname) || checkString(htmlContents)) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    post(
      ratelApi.users.editProfile(user!.id),
      userEditProfileRequest(nickname!, htmlContents!, profileUrl),
    );
    userinfo.refetch();
    router.push(route.home());
  };

  return (
    <div className="w-full max-tablet:w-full flex flex-col gap-10 items-center">
      <FileUploader onUploadSuccess={handleProfileUrl}>
        {profileUrl ? (
          <Image
            src={profileUrl}
            width={40}
            height={80}
            alt="Team Logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer"
          />
        ) : (
          <button className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50">
            Upload logo
          </button>
        )}
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold">Username</label>
          <Input type="text" disabled value={`@${user?.username}`} />
        </Row>
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold">EVM Address</label>
          <Row>
            <Input type="text" disabled value={`${user?.evm_address}`} />
            <Button
              variant={'rounded_secondary'}
              className="py-0 rounded-sm"
              onClick={() => setShowWalletConnect(!showWalletConnect)}
            >
              {showWalletConnect ? 'Hide' : 'Change'}
            </Button>
          </Row>
        </Row>
        <Row
          className="w-full aria-hidden:hidden"
          aria-hidden={!showWalletConnect}
        >
          <WalletSummary
            onUpdate={async (address) => {
              logger.debug('Updating wallet address...', address);

              await post(ratelApi.users.updateEvmAddress(), {
                update_evm_address: {
                  evm_address: address,
                },
              });

              userinfo.refetch();
              setShowWalletConnect(false);
            }}
          />
        </Row>

        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold">Display name</label>
          <Input
            type="text"
            placeholder="Team display name"
            value={nickname}
            onInput={handleNickname}
          />
        </Row>
        <Col>
          <label className="w-40 font-bold">Description</label>
          <Textarea
            placeholder="Please type description of your team."
            value={htmlContents}
            onChange={handleContents}
          />
        </Col>
        <Row className="justify-end py-5">
          <Button
            className={
              checkString(nickname) || checkString(htmlContents)
                ? 'cursor-not-allowed bg-neutral-600'
                : 'cursor-pointer bg-primary'
            }
            variant={'rounded_primary'}
            onClick={handleSave}
          >
            Save
          </Button>
        </Row>
      </Col>
    </div>
  );
}
