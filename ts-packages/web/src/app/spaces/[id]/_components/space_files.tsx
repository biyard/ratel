'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { FileInfo } from '@/lib/api/models/feeds';
import React from 'react';

import { replacePdfLinks, UrlReplacement } from '@/lib/pdf-utils';
import { useRedeemCode } from '@/lib/api/hooks/redeem-codes';
import { useParams } from 'next/navigation';
import { usePopup } from '@/lib/contexts/popup-service';
import { NftSelectModal } from './nft-select-modal';
import { Badge } from '@/lib/api/models/badge';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { useUserBadge } from '@/lib/api/hooks/user-badges';
import { checkString } from '@/lib/string-filter-utils';
import SpaceFile from './space_file';

export interface SpaceFilesProps {
  files: FileInfo[];
  badges: Badge[];
}

export default function SpaceFiles({ files, badges }: SpaceFilesProps) {
  const popup = usePopup();
  const params = useParams();
  const spaceId = Number(params.id);

  const redeem = useRedeemCode(spaceId);
  const { post } = useApiCall();

  const userBadges = useUserBadge(spaceId, 1, 20);

  const badgeList = userBadges?.data?.items ?? [];

  const handlePdfDownload = async (file: FileInfo) => {
    const redeemId = redeem.data.id;
    const currentUrl = window.location.origin;
    const urlReplacements = redeem.data.codes.map((code, index) => {
      return {
        original: `https://redeem-codes/${index}`,
        replace: `${currentUrl}/redeems/${redeemId}?code=${code}`,
      } as UrlReplacement;
    });
    await replacePdfLinks({
      url: file.url ?? '',
      urlReplacements,
      fileName: file.name,
    });
  };
  const handleFileDownload = async (file: FileInfo) => {
    if (badgeList.length != 0) {
      await handlePdfDownload(file);
      return;
    }

    popup
      .open(
        <NftSelectModal
          badges={badges}
          handleMintNfts={async (ids: number[], contract: string) => {
            const res = await post(ratelApi.spaces.claimBadge(spaceId), {
              claim: {
                ids: ids,
                evm_address: contract,
              },
            });
            if (res) {
              userBadges.refetch();
              await handlePdfDownload(file);
              popup.close();
              return true;
            } else {
              popup.close();
              return false;
            }
          }}
        />,
      )
      .withTitle('Select Nft');
  };
  return (
    <BlackBox>
      <div className="flex flex-col w-full gap-5">
        <div className="font-bold text-white text-[15px]/[20px]">
          Attached Files
        </div>

        <div className="grid grid-cols-2 max-tablet:grid-cols-1 gap-2.5">
          {files
            ?.filter((file) => !checkString(file.name))
            .map((file, index) => (
              <SpaceFile
                file={file}
                key={'file ' + index}
                onClick={() => handleFileDownload(file)}
              />
            ))}
        </div>
      </div>
    </BlackBox>
  );
}
