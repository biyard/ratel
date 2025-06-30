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

interface UserSetupPopupProps {
  id?: string;
  nickname?: string;
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
  children?: React.ReactNode;
}

const UserSetupPopup = ({
  id = 'user_setup_popup',
  email = '',
  profileUrl = 'https://metadata.ratel.foundation/ratel/default-profile.png',
}: UserSetupPopupProps) => {
  const { post } = useApiCall();
  const client = useApolloClient();

  const popup = usePopup();
  const [displayName, setDisplayName] = useState('');
  const [userName, setUserName] = useState('');
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
      showErrorToast('Please remove the test keyword');
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
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      let req: any = {
        signup: {
          nickname: displayName,
          email: emailState,
          profile_url: profileUrlState,
          term_agreed: agreed,
          informed_agreed: announcementAgreed,
          username: userName,
          evm_address: auth.evmWallet!.address,
        },
      };

      if (email === '') {
        req = {
          email_signup: {
            nickname: displayName,
            email: emailState,
            profile_url: profileUrlState,
            term_agreed: agreed,
            informed_agreed: announcementAgreed,
            username: userName,
            password: sha3(password),
          },
        };
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
      className="flex flex-col w-100 max-w-100 mx-4.25 max-mobile:!w-full max-mobile:!max-w-full gap-8.75 mt-8.75"
    >
      <FileUploader onUploadSuccess={handleProfileUrl}>
        <div className="group relative flex items-center justify-center w-40 h-40 mx-auto">
          <Image
            src={profileUrlState}
            width={160}
            height={160}
            alt="Team Logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer relative group"
          />

          <div className="absolute w-40 h-40 inset-0 bg-component-bg/50 flex items-center justify-center text-center rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300 text-white font-semibold">
            Click to change profile image
          </div>
        </div>
      </FileUploader>

      <div className="flex flex-col items-start justify-start w-full gap-1.25">
        <div className="w-full flex flex-col gap-[5px]">
          <div className="flex flex-row items-start">
            <span className="text-c-cg-30 font-bold text-base/7">
              {'Email'}
            </span>
          </div>

          <Row>
            <input
              className="w-full outline-none px-5 h-11 text-white text-base placeholder-gray-500 font-medium border rounded-lg border-gray-600"
              disabled={email !== '' || isValidEmail}
              name="username"
              autoComplete="email"
              value={emailState}
              onChange={(e) => {
                setEmailState(e.target.value);
              }}
            />
            {email === '' && (
              <Button
                variant={'rounded_secondary'}
                className="rounded-sm"
                onClick={handleSendCode}
              >
                Send
              </Button>
            )}
          </Row>

          <Row
            className="aria-hidden:hidden"
            aria-hidden={!sentCode || isValidEmail}
          >
            <input
              className="w-full outline-none px-5 h-11 text-white text-base placeholder-gray-500 font-medium border rounded-lg border-gray-600"
              value={authCode}
              onChange={(e) => {
                setAuthCode(e.target.value);
              }}
            />
            <Button
              variant={'rounded_secondary'}
              className="rounded-sm"
              onClick={handleVerify}
            >
              Verify
            </Button>
          </Row>
        </div>
        {email === '' && (
          <div className="w-full flex flex-col gap-[5px]">
            <div className="flex flex-row items-start">
              <span className="text-c-cg-30 font-bold text-base/7">
                Password
              </span>
            </div>
            <input
              className="w-full outline-none px-5 h-11 text-white text-base placeholder-gray-500 font-medium border rounded-lg border-gray-600"
              type="password"
              value={password}
              onChange={(e) => {
                const val = e.target.value;
                setPassword(val);
                setIsValid(validatePassword(val));
              }}
            />

            {!isValid && password.length > 7 && (
              <p className="text-red-500 text-sm mt-1">
                Password must contain letters, numbers, and special characters
                (min 8 chars).
              </p>
            )}
          </div>
        )}

        <div className="flex flex-col gap-5 w-full mt-2.25">
          <LabeledInput
            labelName={'Display Name'}
            placeholder={'Display Name'}
            value={displayName}
            onInput={(value: string) => {
              setDisplayName(value);

              if (checkString(value)) {
                setWarningDisplayname(
                  'Please remove the test keyword from your display name.',
                );
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
            labelName={'User Name'}
            placeholder={'User Name'}
            value={userName}
            onInput={async (value: string) => {
              setUserName(value);
              if (value.length === 0) {
                setWarning('');
                setIsUserNameValid(true);
                return;
              }

              if (!isValidUsername(value)) {
                setWarning(
                  'Only numbers, lowercase letters, -, _ and more than one character can be entered.',
                );
                setIsUserNameValid(false);
                return;
              } else if (checkString(value)) {
                setWarning(
                  'Please remove the test keyword from your username.',
                );
                setIsUserNameValid(false);
                return;
              } else {
                setWarning('');
                setIsUserNameValid(true);
              }
              const {
                data: { users },
              } = await client.query(ratelApi.graphql.getUserByUsername(value));

              if (users.length > 0) {
                setWarning('This username is already taken.');
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
              <strong>[Required]</strong> I have read and accept the{' '}
              <strong>Terms of Service</strong>
            </span>
          </Checkbox>

          <Checkbox id="announcement_checkbox" onChange={setAnnouncementAgree}>
            <span className="text-sm text-gray-400">
              I want to receive announcements and news from Ratel.
            </span>
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
          {'Finished Sign-up'}
        </PrimaryButton>
      </div>

      <LoginPopupFooter />
    </div>
  );
};

const LabeledInput = ({
  labelName,
  placeholder,
  value,
  onInput,
  children,
  warning = '',
}: LabeledInputProps) => (
  <div className="w-full flex flex-col items-start gap-[5px]">
    <div className="text-c-cg-30 font-bold text-base/7">{labelName}</div>
    <input
      type="text"
      className="w-full outline-none px-5 text-white text-base placeholder-gray-500 font-medium border rounded-lg border-gray-600"
      style={{ height: 50 }}
      placeholder={placeholder}
      value={value}
      onChange={(e) => onInput(e.target.value)}
    >
      {children}
    </input>
    {warning !== '' && <span className="text-sm text-c-p-50">{warning}</span>}
  </div>
);

export default UserSetupPopup;
