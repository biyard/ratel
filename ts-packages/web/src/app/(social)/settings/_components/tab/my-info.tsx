'use client';

import FileUploader from '@/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import { ratelApi } from '@/lib/api/ratel_api';
import { checkString } from '@/lib/string-filter-utils';
import { route } from '@/route';
import React from 'react';
import Image from 'next/image';
import { logger } from '@/lib/logger';
import { useApiCall } from '@/lib/api/use-send';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useSettingsContext } from '../../providers.client';
import WalletSummary from '../wallet-summary';
import { useRouter } from 'next/navigation';

export default function MyInfo() {
  const { post } = useApiCall();
  const userInfo = useSuspenseUserInfo();
  const { data: user } = userInfo;
  const router = useRouter();

  const {
    profileUrl,
    handleProfileUrl,
    nickname,
    handleNickname,
    htmlContents,
    handleContents,
    showWalletConnect,
    handleShowWalletConnect,
    handleSave,
  } = useSettingsContext();

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
              onClick={() => handleShowWalletConnect(!showWalletConnect)}
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

              userInfo.refetch();
              handleShowWalletConnect(false);
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
            onClick={async () => {
              await handleSave();
              router.push(route.home());
            }}
          >
            Save
          </Button>
        </Row>
      </Col>
    </div>
  );
}
