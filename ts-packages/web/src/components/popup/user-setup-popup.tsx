'use client';

import React, { useState, useEffect } from 'react';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginPopupFooter } from './login-popup-footer';
import { PrimaryButton } from '../button/primary-button';
import { Checkbox } from '../checkbox/checkbox';
import { ConfirmPopup } from './confirm-popup';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { logger } from '@/lib/logger';
import { useApolloClient } from '@apollo/client';
import { useUserInfo } from '@/lib/api/hooks/users';
import { useAuth } from '@/lib/contexts/auth-context';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { Row } from '../ui/row';
import { Button } from '../ui/button';
import { sha3 } from '@/lib/utils';
import FileUploader from '../file-uploader';
import Image from 'next/image';
import { emailSignupRequest } from '@/lib/api/models/users/email-signup-request';
import { signupRequest } from '@/lib/api/models/users/signup-request';
import { useTranslations } from 'next-intl';

export interface UserSetupPopupProps {
  id?: string;
  nickname?: string;
  username?: string;
  profileUrl?: string;
  email: string;
  principal?: string;
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
}: UserSetupPopupProps) => {
  const t = useTranslations('Signup');
  const { post } = useApiCall();
  const client = useApolloClient();

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
  const [sentCode, setSentCode] = useState(false);
  const [isValidEmail, setIsValidEmail] = useState(email !== '');
  const [profileUrlState, setProfileUrlState] = useState(profileUrl);

  const query = useUserInfo();
  const auth = useAuth();

  const isValidUsername = (username: string) => /^[a-z0-9_-]+$/.test(username);

  const handleProfileUrl = (url: string) => {
    setProfileUrlState(url);
  };

  const handleSubmit = async () => {
    if (checkString(displayName) || checkString(userName)) {
      showErrorToast(t('remove_test_keyword'));
      return;
    }
    if (!agreed || !isUserNameValid) return;

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
      let req;
      if (email === '') {
        req = emailSignupRequest(
          displayName,
          emailState,
          profileUrlState,
          agreed,
          announcementAgreed,
          userName,
          sha3(password),
          auth.telegramRaw,
        );
      } else {
        req = signupRequest(
          displayName,
          emailState,
          profileUrlState,
          agreed,
          announcementAgreed,
          userName,
          auth.evmWallet!.address,
          auth.telegramRaw,
        );
      }

      if (await post(ratelApi.users.signup(), req)) {
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
    logger.debug('Sending verification code to email:', emailState);
    await post(ratelApi.users.sendVerificationCode(), {
      send_verification_code: {
        email: emailState,
      },
    });
    setSentCode(true);
  };

  const handleVerify = async () => {
    logger.debug('Sending verification code to email:', emailState);
    await post(ratelApi.users.sendVerificationCode(), {
      verify: {
        email: emailState,
        value: authCode,
      },
    });
    setIsValidEmail(true);
  };

  useEffect(() => {
    setIsUserNameValid(isValidUsername(userName));
  }, [userName]);

  return (
    <div
      id={id}
      className="h-150 tablet:h-full max-h-screen overflow-y-scroll w-full mt-8.75 scrollbar-hide"
    >
      <div className="flex flex-col max-w-100 w-full gap-4 tablet:gap-8.75 max-mobile:overflow-y-scroll">
        <FileUploader onUploadSuccess={handleProfileUrl}>
          <div className="group relative flex items-center justify-center size-40 max-mobile:size-20 mx-auto">
            <Image
              src={profileUrlState}
              width={160}
              height={160}
              alt="Team Logo"
              className="w-40 h-40 rounded-full object-cover cursor-pointer relative group max-mobile:size-20"
            />

            <div className="absolute w-40 h-40 inset-0 bg-component-bg/50 flex items-center justify-center text-center rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300 text-white font-semibold">
              {t('clicked_image')}
            </div>
          </div>
        </FileUploader>

        <div className="flex flex-col items-start justify-start w-full gap-1.25">
          <div className="w-full flex flex-col gap-[5px]">
            <div className="flex flex-row items-start">
              <span className="text-c-cg-30 font-bold text-base/7">
                {t('email')}
              </span>
            </div>

            <Row>
              <input
                type="email"
                className="bg-input-box-bg border border-input-box-border w-full outline-none px-5 h-11 text-base placeholder-gray-500 font-medium rounded-lg"
                disabled={email !== '' || isValidEmail}
                name="email"
                aria-label="email"
                autoComplete="email"
                placeholder={t('email')}
                value={emailState}
                onChange={(e) => {
                  setEmailState(e.target.value);
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

            <Row
              className="aria-hidden:hidden"
              aria-hidden={!sentCode || isValidEmail}
            >
              <input
                className="bg-input-box-bg border border-input-box-border w-full outline-none px-5 h-11 text-text-primary text-base placeholder-gray-500 font-medium rounded-lg"
                value={authCode}
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
            <div className="w-full flex flex-col gap-[5px]">
              <div className="flex flex-row items-start">
                <span className="text-c-cg-30 font-bold text-base/7">
                  {t('password')}
                </span>
              </div>
              <input
                className="bg-input-box-bg border border-input-box-border w-full outline-none px-5 h-11 text-text-primary text-base placeholder-gray-500 font-medium rounded-lg"
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
                <p className="text-red-500 text-sm mt-1">
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
                  setIsUserNameValid(true);
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
                const {
                  data: { users },
                } = await client.query(
                  ratelApi.graphql.getUserByUsername(value),
                );

                if (users.length > 0) {
                  setWarning(t('already_exists_user'));
                  setIsUserNameValid(false);
                } else {
                  setWarning('');
                  setIsUserNameValid(true);
                }
              }}
              warning={warning}
            />
          </div>

          <div className="flex flex-col gap-2.25 items-start mb-5 mt-5">
            <Checkbox id="agree_checkbox" onChange={setAgreed}>
              <span className="text-sm text-gray-400">
                {t.rich('agree_tos', {
                  req: (chunks) => <strong>{chunks}</strong>,
                  b: (chunks) => <strong>{chunks}</strong>,
                })}
              </span>
            </Checkbox>

            <Checkbox
              id="announcement_checkbox"
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
  <div className="w-full flex flex-col items-start gap-[5px]">
    <div className="text-c-cg-30 font-bold text-base/7">{labelName}</div>
    <input
      type="text"
      className="bg-input-box-bg border border-input-box-border w-full outline-none px-5 text-text-primary text-base placeholder-gray-500 font-medium rounded-lg"
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
