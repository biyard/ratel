import { PrimaryButton } from '@/components/button/primary-button';
import { Checkbox } from '@/components/checkbox/checkbox';
import FileUploader from '@/features/spaces/files/components/file-uploader';
import { Button } from '@/components/ui/button';
import { useTranslation, Trans } from 'react-i18next';
import { useState } from 'react';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  validateEmail,
  validateNickname,
  validatePassword,
} from '@/lib/valid-utils';
import { sha3 } from '@/lib/utils';
import Footer from './footer';
import LabelInput from './label-input';

const defaultImage =
  'https://metadata.ratel.foundation/ratel/default-profile.png';

function sendVerificationCode(email: string) {
  return apiFetch<void>(
    `${config.api_url}${ratelApi.users.sendVerificationCode()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        send_verification_code: {
          email,
        },
      }),
    },
  );
}

function verifyCode(email: string, code: string) {
  return apiFetch<void>(
    `${config.api_url}${ratelApi.users.sendVerificationCode()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        verify: {
          email,
          value: code,
        },
      }),
    },
  );
}

const Warning = {
  Email: 'Email',
  Code: 'Code',
  Password: 'Password',
  DisplayName: 'DisplayName',
  UserName: 'UserName',
  Agree: 'Agree',
} as const;

type Warning = (typeof Warning)[keyof typeof Warning];

interface InitialData {
  email: string;
  nickname: string;
  username: string;
  profileImage: string;
}

interface SignupFormProps {
  initialData?: InitialData;
  updateUserInfo: ({
    email,
    nickname,
    username,
    profileImage,
    password,
    agreed,
    announcementAgreed,
  }: {
    email: string;
    nickname: string;
    username: string;
    profileImage: string;
    password: string;
    agreed: boolean;
    announcementAgreed: boolean;
  }) => Promise<void>;
}
export default function SignupForm({
  initialData,
  updateUserInfo,
}: SignupFormProps) {
  const { t } = useTranslation('Signup');

  const {
    email: initialEmail,
    nickname: initialNickname,
    username: initialUsername,
    profileImage: initialProfileImage,
  } = initialData || {};
  const [errors, setErrors] = useState<Partial<Record<Warning, string>>>({});

  // Internal States
  const [code, setCode] = useState('');
  const [isValidEmail, setIsValidEmail] = useState(!!initialEmail);
  const [showCodeInput, setShowCodeInput] = useState(false);
  const [sendButtonClicked, setSendButtonClicked] = useState(false);

  const [profileImage, setProfileImage] = useState(initialProfileImage);
  const [email, setEmail] = useState(initialEmail || '');
  const [username, setUsername] = useState(initialUsername || '');
  const [nickname, setNickname] = useState(initialNickname || '');
  const [password, setPassword] = useState('');

  const [agreed, setAgreed] = useState(false);
  const [announcementAgreed, setAnnouncementAgreed] = useState(false);

  const insertError = (key: Warning, message: string) => {
    setErrors((prev) => ({ ...prev, [key]: message }));
  };
  const popError = (key: Warning) => {
    setErrors((prev) => {
      const newErrors = { ...prev };
      delete newErrors[key];
      return newErrors;
    });
  };
  const handleSendCode = async () => {
    if (email) {
      try {
        await sendVerificationCode(email);
        setSendButtonClicked(true);
        setShowCodeInput(true);
      } catch (error) {
        console.error(error);
        insertError(Warning.Email, t('Email.error.unregistered'));
      }
    }
  };

  const handleVerifyCode = async () => {
    if (!email || initialEmail) return;
    try {
      await verifyCode(email, code);
      setIsValidEmail(true);
      setShowCodeInput(false);
    } catch (error) {
      console.error(error);
      insertError(Warning.Code, t('Code.error.invalid'));
      setIsValidEmail(false);
      setCode('');
    }
  };

  const handleUpdateEmail = (value: string) => {
    if (validateEmail(value)) {
      setShowCodeInput(true);
      popError(Warning.Email);
    } else {
      insertError(Warning.Email, t('Email.error.invalid'));
    }
    //Reset State if email changed
    setShowCodeInput(false);
    setSendButtonClicked(false);
    setEmail(value);
  };

  const handleUpdateNickname = (value: string) => {
    setNickname(value);
    if (!validateNickname(value)) {
      insertError(Warning.DisplayName, t('Nickname.error.invalid'));
    } else {
      popError(Warning.DisplayName);
    }
  };

  const handleUpdateUsername = (value: string) => {
    setUsername(value);
  };

  const handleProfileImage = (url: string) => {
    setProfileImage(url);
  };

  const handlePassword = (password: string) => {
    setPassword(password);
    if (!validatePassword(password)) {
      insertError(Warning.Password, t('Password.error.invalid'));
    } else {
      popError(Warning.Password);
    }
  };

  const handleSubmit = async () => {
    if (!isValidEmail) {
      insertError(Warning.Email, t('Email.error.unregistered'));
      return;
    }
    if (!nickname) {
      insertError(Warning.DisplayName, t('Nickname.error.invalid'));
      return;
    }
    if (!username) {
      insertError(
        Warning.UserName,
        t('CommonError.required', { fieldName: t('Username.label') }),
      );
      return;
    }
    if (!initialEmail && !password) {
      insertError(Warning.Password, t('Password.error.invalid'));
      return;
    }
    if (!agreed) {
      insertError(Warning.Agree, t('Term.error.required'));
      return;
    }

    if (!initialEmail && password && !validatePassword(password)) {
      insertError(Warning.Password, t('Password.error.invalid'));
      return;
    }

    updateUserInfo({
      email,
      nickname,
      username,
      profileImage: profileImage || defaultImage,
      password: password ? sha3(password) : '',
      agreed,
      announcementAgreed,
    });
  };

  return (
    <div className="flex flex-col gap-5 w-full">
      <FileUploader onUploadSuccess={handleProfileImage}>
        <div className="group relative flex items-center justify-center size-40 max-mobile:size-20 mx-auto">
          <img
            src={profileImage || defaultImage}
            alt="logo"
            className="w-40 h-40 rounded-full object-cover cursor-pointer relative group max-mobile:size-20"
          />

          <div className="absolute w-40 h-40 inset-0 bg-component-bg/50 flex items-center justify-center text-center rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300 text-white font-semibold">
            {t('clicked_image')}
          </div>
        </div>
      </FileUploader>
      <div className="flex flex-col gap-3.75">
        <LabelInput
          id="input-email"
          type="email"
          label={t('Email.label')}
          placeholder={t('Email.placeholder')}
          value={email}
          disabled={isValidEmail || !!initialEmail}
          onChange={handleUpdateEmail}
          errorMessage={errors[Warning.Email]}
        >
          {!isValidEmail && (
            <Button
              className="w-20"
              variant="rounded_secondary"
              size="sm"
              onClick={handleSendCode}
              disabled={
                errors[Warning.Email] !== undefined ||
                !email ||
                sendButtonClicked
              }
            >
              {t('Email.send_button_label')}
            </Button>
          )}
        </LabelInput>

        {!isValidEmail && showCodeInput && (
          <LabelInput
            id="input-code"
            placeholder={t('Code.placeholder')}
            value={code}
            onChange={setCode}
            errorMessage={errors[Warning.Code]}
          >
            <Button
              className="w-20"
              variant="rounded_secondary"
              size="sm"
              disabled={code.length === 0}
              onClick={handleVerifyCode}
            >
              {t('Code.verify_button_label')}
            </Button>
          </LabelInput>
        )}
        {!initialEmail && (
          <LabelInput
            id="input-password"
            type="password"
            label={t('Password.label')}
            placeholder={t('Password.placeholder')}
            value={password}
            errorMessage={errors[Warning.Password]}
            onChange={handlePassword}
          />
        )}

        <LabelInput
          id="input-nickname"
          label={t('Nickname.label')}
          placeholder={t('Nickname.placeholder')}
          value={nickname}
          onChange={handleUpdateNickname}
          errorMessage={errors[Warning.DisplayName]}
        />

        <LabelInput
          id="input-username"
          label={t('Username.label')}
          placeholder={t('Username.placeholder')}
          value={username}
          onChange={handleUpdateUsername}
          errorMessage={errors[Warning.UserName]}
        />

        <Checkbox id="agree_checkbox" onChange={setAgreed}>
          <div className="flex flex-col gap-1">
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
            {errors[Warning.Agree] && (
              <span className="text-sm text-red-500">
                {errors[Warning.Agree]}
              </span>
            )}
          </div>
        </Checkbox>

        <Checkbox id="announcement_checkbox" onChange={setAnnouncementAgreed}>
          <span className="text-sm text-gray-400">{t('agree_news')}</span>
        </Checkbox>
      </div>
      <PrimaryButton disabled={false} onClick={handleSubmit}>
        {t('finish_signup')}
      </PrimaryButton>

      <Footer />
    </div>
  );
}
