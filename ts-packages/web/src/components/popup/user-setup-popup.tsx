'use client';

import { useState, useEffect } from 'react';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginPopupFooter } from './login-popup-footer';
import { PrimaryButton } from '../button/primary-button';
import { Checkbox } from '../checkbox/checkbox';
import { ConfirmPopup } from './confirm-popup';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { logger } from '@/lib/logger';
import { useAuth } from '@/lib/contexts/auth-context';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { Row } from '../ui/row';
import { Button } from '../ui/button';
import { sha3 } from '@/lib/utils';
import FileUploader from '../../features/spaces/files/components/file-uploader';
import { useTranslation, Trans } from 'react-i18next';
import { OAuthProvider } from '@/types/oauth-provider';
import { ratelSdk } from '@/lib/api/ratel';
import { useUserInfo } from '@/hooks/use-user-info';
import { signup } from '@/lib/api/ratel/auth.v3';

export interface UserSetupPopupProps {
  id?: string;
  nickname?: string;
  username?: string;
  profileUrl?: string;
  email: string;
  principal?: string;
  idToken?: string;
  accessToken?: string;
  provider?: OAuthProvider;
}

interface LabeledInputProps {
  labelName: string;
  placeholder: string;
  value: string;
  onInput: (text: string) => void;
  warning?: string;
}

const UserSetupPopup = ({
  id = 'user_setup_popup',
  email = '',
  profileUrl = 'https://metadata.ratel.foundation/ratel/default-profile.png',
  username = '',
  nickname = '',
  provider,
  accessToken,
}: UserSetupPopupProps) => {
  const { t } = useTranslation('Signup');
  const { post } = useApiCall();

  const popup = usePopup();
  const [displayName, setDisplayName] = useState(nickname);
  const [userName, setUserName] = useState(username);
  const [agreed, setAgreed] = useState(false);
  const [announcementAgreed, setAnnouncementAgree] = useState(false);
  const [isUserNameValid, setIsUserNameValid] = useState(false);
  const [warning, setWarning] = useState('');
  const [warningDisplayName, setWarningDisplayname] = useState('');
  const [isValidDisplayName, setIsValidDisplayName] = useState(true);
  const [emailState, setEmailState] = useState(email);
  const [emailWarning, setEmailWarning] = useState('');
  const [sentCode, setSentCode] = useState(false);
  const [isValidEmail, setIsValidEmail] = useState(email !== '');
  const [profileUrlState, setProfileUrlState] = useState(profileUrl);
  const [termsError, setTermsError] = useState('');

  const query = useUserInfo();
  const auth = useAuth();

  const isValidUsername = (username: string) => /^[a-z0-9_-]+$/.test(username);

  const isValidEmailFormat = (email: string) => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  };

  const handleProfileUrl = (url: string) => {
    setProfileUrlState(url);
  };

  const handleSubmit = async () => {
    // Clear previous errors
    setTermsError('');

    // Validate all required fields
    if (!displayName.trim()) {
      setWarningDisplayname(t('display_name_required'));
      setIsValidDisplayName(false);
      return;
    }

    if (!userName.trim()) {
      setWarning(t('username_required'));
      setIsUserNameValid(false);
      return;
    }

    if (checkString(displayName) || checkString(userName)) {
      showErrorToast(t('remove_test_keyword'));
      return;
    }

    if (!agreed) {
      setTermsError(t('terms_required'));
      return;
    }

    if (!isUserNameValid) return;

    if (announcementAgreed) {
      try {
        await post(ratelApi.subscription.subscribe(), {
          subscribe: {
            email: emailState,
          },
        });
      } catch (err) {
        logger.error('failed to subscription with error: ', err);
      }
    }

    try {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const req: any = {
        display_name: displayName,
        username: userName,
        profile_url: profileUrlState,
        description: '',
        term_agreed: agreed,
        informed_agreed: announcementAgreed,
      };
      if (emailState !== '' && password !== '') {
        // NOTE: Signup with email and password
        req.email = emailState;
        req.password = sha3(password);
        req.code = authCode;
      } else if (provider && accessToken) {
        req.provider = provider;
        req.access_token = accessToken;
      } else if (auth.telegramRaw) {
        // NOTE: First signup for telegram
        // FIXME: Update email and password for telegram user
        //        But, v3 does not support now.
        //        Consider just update email and password in my profile
        req.telegram_raw = auth.telegramRaw;
        // FIXME: EVM address must be verified by signature in server-side
        /* req.evm_address = auth.evmWallet?.address; */
      }
      const res = await signup(req);
      if (res) {
        query.refetch();
        popup.open(<ConfirmPopup />);
      }
    } catch (err) {
      logger.error('failed to signup with error: ', err);
    }
  };
  const [password, setPassword] = useState('');
  const [isValid, setIsValid] = useState(true);
  const [authCode, setAuthCode] = useState('');

  const validatePassword = (pw: string) => {
    const regex =
      /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}\[\]:;<>,.?~\\/-]).{8,}$/;
    return regex.test(pw);
  };

  const handleSendCode = async () => {
    if (!isValidEmailFormat(emailState)) {
      setEmailWarning(t('invalid_email_format'));
      return;
    }

    setEmailWarning('');
    logger.debug('Sending verification code to email:', emailState);
    try {
      await ratelSdk.auth.sendVerificationCode(emailState);
      setSentCode(true);
    } catch (err) {
      showErrorToast(t('failed_send_code'));
      logger.error('failed to send verification code with error: ', err);
    }
  };

  const handleVerify = async () => {
    logger.debug('Sending verification code to email:', emailState);
    try {
      await ratelSdk.auth.verifyCode(emailState, authCode);

      setIsValidEmail(true);
    } catch (err) {
      showErrorToast(t('failed_verify_code'));
      logger.error('failed to verify code with error: ', err);
    }
  };

  useEffect(() => {
    setIsUserNameValid(isValidUsername(userName));
  }, [userName]);

  return (
    <div
      id={id}
      className="overflow-y-scroll w-full max-h-screen h-150 tablet:h-full mt-8.75 scrollbar-hide"
    >
      <div className="flex flex-col gap-4 w-full max-w-100 tablet:gap-8.75 max-mobile:overflow-y-scroll">
        <FileUploader onUploadSuccess={handleProfileUrl}>
          <div className="flex relative justify-center items-center mx-auto group size-40 max-mobile:size-20">
            <img
              src={profileUrlState}
              alt="Team Logo"
              className="object-cover relative w-40 h-40 rounded-full cursor-pointer group max-mobile:size-20"
            />

            <div className="flex absolute inset-0 justify-center items-center w-40 h-40 font-semibold text-center text-white rounded-full opacity-0 transition-opacity duration-300 group-hover:opacity-100 bg-component-bg/50">
              {t('clicked_image')}
            </div>
          </div>
        </FileUploader>

        <div className="flex flex-col justify-start items-start w-full gap-1.25">
          <div className="flex flex-col w-full gap-[5px]">
            <div className="flex flex-row items-start">
              <span className="font-bold text-c-cg-30 text-base/7">
                {t('email')}
              </span>
            </div>

            <Row>
              <input
                type="email"
                className="px-5 w-full h-11 text-base font-medium placeholder-gray-500 rounded-lg border outline-none bg-input-box-bg border-input-box-border"
                disabled={email !== '' || isValidEmail}
                name="email"
                aria-label="email"
                autoComplete="email"
                placeholder={t('email')}
                value={emailState}
                onChange={(e) => {
                  const value = e.target.value;
                  setEmailState(value);
                  if (value && !isValidEmailFormat(value)) {
                    setEmailWarning(t('invalid_email_format'));
                  } else {
                    setEmailWarning('');
                  }
                }}
              />
              {email === '' && (
                <Button
                  variant={'rounded_secondary'}
                  className="rounded-sm border border-transparent light:border-neutral-300 light:text-text-primary"
                  onClick={handleSendCode}
                >
                  {t('send')}
                </Button>
              )}
            </Row>
            {emailWarning && (
              <p className="mt-1 text-sm text-red-500">{emailWarning}</p>
            )}

            <Row
              className="aria-hidden:hidden"
              aria-hidden={!sentCode || isValidEmail}
            >
              <input
                id="otp"
                name="otp"
                className="px-5 w-full h-11 text-base font-medium placeholder-gray-500 rounded-lg border outline-none bg-input-box-bg border-input-box-border text-text-primary"
                value={authCode}
                placeholder={t('verification_code')}
                onChange={(e) => {
                  setAuthCode(e.target.value);
                }}
              />
              <Button
                variant={'rounded_secondary'}
                className="rounded-sm border border-transparent light:border-neutral-300 light:text-text-primary"
                onClick={handleVerify}
              >
                {t('verify')}
              </Button>
            </Row>
          </div>
          {email === '' && (
            <div className="flex flex-col w-full gap-[5px]">
              <div className="flex flex-row items-start">
                <span className="font-bold text-c-cg-30 text-base/7">
                  {t('password')}
                </span>
              </div>
              <input
                className="px-5 w-full h-11 text-base font-medium placeholder-gray-500 rounded-lg border outline-none bg-input-box-bg border-input-box-border text-text-primary"
                type="password"
                name="password"
                aria-label="password"
                placeholder={t('password')}
                value={password}
                onChange={(e) => {
                  const val = e.target.value;
                  setPassword(val);
                  setIsValid(validatePassword(val));
                }}
              />

              {!isValid && password.length > 7 && (
                <p className="mt-1 text-sm text-red-500">
                  {t('invalid_password_format')}
                </p>
              )}
            </div>
          )}

          <div className="flex flex-col gap-5 w-full mt-2.25">
            <LabeledInput
              labelName={t('display_name')}
              placeholder={t('display_name')}
              value={displayName}
              onInput={(value: string) => {
                setDisplayName(value);

                if (!value.trim()) {
                  setWarningDisplayname('');
                  setIsValidDisplayName(false);
                  return;
                }

                if (checkString(value)) {
                  setWarningDisplayname(t('display_name_warning'));
                  setIsValidDisplayName(false);
                  return;
                } else {
                  setWarningDisplayname('');
                  setIsValidDisplayName(true);
                }
              }}
              warning={warningDisplayName}
            />

            <LabeledInput
              labelName={t('user_name')}
              placeholder={t('user_name')}
              value={userName}
              onInput={async (value: string) => {
                setUserName(value);
                if (value.length === 0) {
                  setWarning('');
                  setIsUserNameValid(false);
                  return;
                }

                if (!isValidUsername(value)) {
                  setWarning(t('invalid_username_format'));
                  setIsUserNameValid(false);
                  return;
                } else if (checkString(value)) {
                  setWarning(t('user_name_warning'));
                  setIsUserNameValid(false);
                  return;
                } else {
                  setWarning('');
                  setIsUserNameValid(true);
                }

                setWarning('');
                setIsUserNameValid(true);

                // const users = await get(
                //   ratelApi.users.getUserByUsername(value),
                // );

                // if (users.length > 0) {
                //   setWarning(t('already_exists_user'));
                //   setIsUserNameValid(false);
                // } else {
                // }
              }}
              warning={warning}
            />
          </div>

          <div className="flex flex-col items-start mt-5 mb-5 gap-2.25">
            <Checkbox
              id="agree_checkbox"
              value={agreed}
              onChange={(checked) => {
                setAgreed(checked);
                if (checked) {
                  setTermsError('');
                }
              }}
            >
              <span className="text-sm text-gray-400">
                <Trans
                  i18nKey="agree_tos"
                  ns="Signup"
                  components={{
                    req: <strong />,
                    b: <strong />,
                  }}
                />
              </span>
            </Checkbox>
            {termsError && (
              <p className="-mt-1 text-sm text-red-500">{termsError}</p>
            )}

            <Checkbox
              id="announcement_checkbox"
              value={announcementAgreed}
              onChange={setAnnouncementAgree}
            >
              <span className="text-sm text-gray-400">{t('agree_news')}</span>
            </Checkbox>
          </div>

          <PrimaryButton
            disabled={
              !agreed ||
              !isUserNameValid ||
              !isValidDisplayName ||
              checkString(displayName) ||
              checkString(userName)
            }
            onClick={handleSubmit}
          >
            {t('finish_signup')}
          </PrimaryButton>
        </div>

        <LoginPopupFooter />
      </div>
    </div>
  );
};

const LabeledInput = ({
  labelName,
  placeholder,
  value,
  onInput,
  warning = '',
}: LabeledInputProps) => (
  <div className="flex flex-col items-start w-full gap-[5px]">
    <div className="font-bold text-c-cg-30 text-base/7">{labelName}</div>
    <input
      type="text"
      className="px-5 w-full text-base font-medium placeholder-gray-500 rounded-lg border outline-none bg-input-box-bg border-input-box-border text-text-primary"
      style={{ height: 50 }}
      placeholder={placeholder}
      aria-label={labelName}
      value={value}
      onChange={(e) => onInput(e.target.value)}
    />
    {warning !== '' && (
      <span className="text-sm text-c-p-50 light:text-red-600">{warning}</span>
    )}
  </div>
);

export default UserSetupPopup;
