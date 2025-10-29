import FileUploader from '@/features/spaces/files/components/file-uploader';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Textarea } from '@/components/ui/textarea';
import { checkString } from '@/lib/string-filter-utils';
import { route } from '@/route';
import { logger } from '@/lib/logger';
import { useSettingsContext } from '../../../providers.client';
import WalletSummary from '../../wallet-summary';
import { showErrorToast } from '@/lib/toast';
import { updateUserEvmAddress } from '@/lib/api/ratel/me.v3';
import { useController } from './use-controller';

export default function MyInfo() {
  const ctrl = useController();
  const { user, navigate, t } = ctrl;
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
    <div className="flex flex-col gap-10 items-center w-full max-tablet:w-full">
      <FileUploader onUploadSuccess={handleProfileUrl}>
        {profileUrl ? (
          <img
            src={profileUrl}
            alt="Team Logo"
            data-pw="profile-image"
            className="object-cover w-40 h-40 rounded-full cursor-pointer"
          />
        ) : (
          <button
            data-pw="upload-profile-button"
            className="flex justify-center items-center w-40 h-40 text-sm font-semibold rounded-full bg-c-wg-80 text-text-primary"
          >
            {t('upload_logo')}
          </button>
        )}
      </FileUploader>

      <Col className="gap-2.5 w-full">
        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('username')}
          </label>
          <Input
            type="text"
            disabled
            data-pw="username-input"
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
              data-pw="evm-address-input"
              value={`${user?.evm_address}`}
            />
            <Button
              variant={'rounded_secondary'}
              data-pw="toggle-wallet-button"
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
              try {
                await updateUserEvmAddress(address);

                await ctrl.userInfo.refetch();
                handleShowWalletConnect(false);
              } catch (error) {
                showErrorToast('Failed to update EVM address');
                console.error('Failed to update EVM address:', error);
              }
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
            data-pw="display-name-input"
            className="text-text-primary"
            value={nickname}
            onInput={handleNickname}
            maxLength={30}
          />
        </Row>

        <Row className="max-tablet:flex-col">
          <label className="w-40 font-bold text-text-primary">
            {t('identity_verification')}
          </label>
          <Row className="gap-2 items-center">
            <Button
              variant={'rounded_secondary'}
              data-pw="identity-verification-button"
              className="py-2 px-4 rounded-sm disabled:cursor-not-allowed bg-enable-button-bg text-enable-button-white-text hover:bg-enable-button-bg/80 disabled:bg-disable-button-bg disabled:text-disable-button-white-text"
              onClick={ctrl.handleIdentityVerification}
              disabled={ctrl.identityVerification.verifying}
            >
              {ctrl.identityVerification.verifying
                ? t('verifying')
                : t('verify_identity')}
            </Button>
          </Row>
        </Row>
        <Col>
          <label className="w-40 font-bold text-text-primary">
            {t('description')}
          </label>
          <Textarea
            placeholder={t('description_hint')}
            data-pw="description-textarea"
            className="text-text-primary"
            value={htmlContents}
            onChange={handleContents}
          />
        </Col>
        <Row className="justify-end py-5">
          <Button
            data-pw="save-profile-button"
            className={
              checkString(nickname) || checkString(htmlContents)
                ? 'cursor-not-allowed bg-disable-button-bg text-disable-button-white-text'
                : 'cursor-pointer bg-enable-button-bg text-enable-button-white-text'
            }
            variant={'rounded_primary'}
            onClick={async () => {
              const success = await handleSave();
              if (success) {
                navigate(route.home());
              }
            }}
          >
            {t('save')}
          </Button>
        </Row>
      </Col>
    </div>
  );
}
