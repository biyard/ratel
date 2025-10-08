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
import { logger } from '@/lib/logger';
import { useApiCall } from '@/lib/api/use-send';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useSettingsContext } from '../../providers.client';
import WalletSummary from '../wallet-summary';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';

export default function MyInfo() {
  const { t } = useTranslation('Settings');
  const { post } = useApiCall();
  const userInfo = useSuspenseUserInfo();
  const { data: user } = userInfo;
  const navigate = useNavigate();

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
          <img
            src={profileUrl}
            alt="Team Logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer"
          />
        ) : (
          <button className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-text-primary">
            {t('upload_logo')}
          </button>
        )}
      </FileUploader>

      <Col className="w-full gap-2.5">
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('username')}
          </label>
          <Input
            type="text"
            disabled
            className="text-text-primary"
            value={`@${user?.username}`}
          />
        </Row>
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('evm_address')}
          </label>
          <Row>
            <Input
              type="text"
              className="text-text-primary"
              disabled
              value={`${user?.evm_address}`}
            />
            <Button
              variant={'rounded_secondary'}
              className="py-0 rounded-sm bg-enable-button-bg text-enable-button-white-text hover:bg-enable-button-bg/80"
              onClick={() => handleShowWalletConnect(!showWalletConnect)}
            >
              {showWalletConnect ? t('hide') : t('change')}
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
          <label className="w-40 font-bold text-text-primary">
            {t('display_name')}
          </label>
          <Input
            type="text"
            placeholder={t('display_name')}
            className="text-text-primary"
            value={nickname}
            onInput={handleNickname}
          />
        </Row>
        <Col>
          <label className="w-40 font-bold text-text-primary">
            {t('description')}
          </label>
          <Textarea
            placeholder={t('description_hint')}
            className="text-text-primary"
            value={htmlContents}
            onChange={handleContents}
          />
        </Col>
        <Row className="justify-end py-5">
          <Button
            className={
              checkString(nickname) || checkString(htmlContents)
                ? 'cursor-not-allowed bg-disable-button-bg text-disable-button-white-text'
                : 'cursor-pointer bg-enable-button-bg text-enable-button-white-text'
            }
            variant={'rounded_primary'}
            onClick={async () => {
              await handleSave();
              navigate(route.home());
            }}
          >
            {t('save')}
          </Button>
        </Row>
      </Col>
    </div>
  );
}
