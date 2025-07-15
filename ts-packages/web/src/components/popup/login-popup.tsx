'use client';
import React, { useCallback, useState } from 'react';
import GoogleIcon from '@/assets/icons/google.svg';
import { LoginPopupFooter } from './login-popup-footer';
import { LoaderPopup } from './loader-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginFailurePopup } from './login-failure-popup';
import UserSetupPopup, { type UserSetupPopupProps } from './user-setup-popup';
import { logger } from '@/lib/logger';
import { useAuth, useEd25519KeyPair } from '@/lib/contexts/auth-context';
import { AuthUserInfo, EventType } from '@/lib/service/firebase-service';
import { send } from '@/lib/api/send';
import { refetchUserInfo } from '@/lib/api/hooks/users';
import { useQueryClient } from '@tanstack/react-query';
import { Col } from '../ui/col';
import { Row } from '../ui/row';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { sha3 } from '@/lib/utils';
import { useApolloClient } from '@apollo/client';
import { ratelApi } from '@/lib/api/ratel_api';
import { useNetwork } from '@/app/(social)/_hooks/use-network';
import { isWebView } from '@/lib/webview-utils';
import { TelegramIcon } from '../icons';
import { type User as TelegramUser } from '@telegram-apps/sdk-react';

interface LoginModalProps {
  id?: string;
  disableClose?: boolean;
}

interface LoginBoxProps {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
}

export const LoginModal = ({
  id = 'login_popup',
  disableClose = false,
}: LoginModalProps) => {
  const popup = usePopup();
  const network = useNetwork();
  const anonKeyPair = useEd25519KeyPair();
  const queryClient = useQueryClient();
  const cli = useApolloClient();

  const { login, ed25519KeyPair, telegramRaw } = useAuth();
  const [email, setEmail] = useState('');
  const [warning, setWarning] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [passwordWarning, setPasswordWarning] = useState('');

  const updateTelegramId = async () => {
    if (telegramRaw) {
      try {
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_API_URL}${ratelApi.users.updateTelegramId()}`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            credentials: 'include',
            body: JSON.stringify({
              update_telegram_id: {
                telegram_raw: telegramRaw,
              },
            }),
          },
        );
        if (!response.ok) {
          logger.error('Failed to update Telegram ID:', response.status);
        }
      } catch (error) {
        logger.error('Error updating Telegram ID:', error);
      }
    }
  };

  const openUserSetupPopup = useCallback(
    (props: UserSetupPopupProps) => {
      if (disableClose) {
        popup
          .open(<UserSetupPopup {...props} />)
          .withoutClose()
          .withoutBackdropClose();
      } else {
        popup.open(<UserSetupPopup {...props} />).withoutBackdropClose();
      }
    },
    [popup, disableClose],
  );

  const validatePassword = (pw: string) => {
    const regex =
      /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}\[\]:;<>,.?~\\/-]).{8,}$/;
    return regex.test(pw);
  };

  const handleChangePassword = async (pw: string) => {
    setPassword(pw);

    if (!validatePassword(pw)) {
      setPasswordWarning(
        'Password must contain letters, numbers, and special characters (min 8 chars).',
      );
      return;
    } else {
      setPasswordWarning('');
    }
  };

  const handleSignIn = async () => {
    const hashedPassword = sha3(password);
    const url = `/api/login?email=${encodeURIComponent(email)}&password=${hashedPassword}`;
    const info = await send(anonKeyPair, url, '');

    if (info) {
      refetchUserInfo(queryClient);
      await updateTelegramId();
      network.refetch();
    }

    popup.close();
  };

  const handleContinue = async () => {
    if (showPassword) {
      handleSignIn();
      return;
    }

    // check if email is valid
    if (!email || !email.includes('@')) {
      setWarning('Please enter a valid email address.');
      return;
    }

    const {
      data: { users },
    } = await cli.query(ratelApi.graphql.getUserByEmail(email));

    if (users.length === 0) {
      setWarning('This email is not registered.');
      return;
    }

    setWarning('');
    setShowPassword(true);
  };

  const handleGoogleSignIn = async () => {
    logger.debug('Google login button clicked');
    const loader = popup.open(
      <LoaderPopup
        title="Sign in"
        description="Signing you in..."
        logo={<GoogleIcon width="50" height="50" />}
        logoOrigin={<GoogleIcon />}
        msg="Continue with Google"
        serviceName="Google"
      />,
    );

    try {
      const user: AuthUserInfo = await login(anonKeyPair);
      logger.debug('Google login user info:', user);
      // loader.close();
      logger.debug('User principal:', user.principal);
      logger.debug('User keypair:', user.keyPair?.getPrincipal().toText());
      logger.debug(
        'edkeypair principal:',
        ed25519KeyPair?.getPrincipal().toText(),
      );
      const info = await send(user.keyPair!, '/api/login', '');

      logger.debug('User info from API:', info);

      if (!info) {
        user.event = EventType.SignUp;
      }

      if (user?.event == EventType.SignUp) {
        openUserSetupPopup({
          email: user.email ?? '',
          nickname: user.displayName ?? undefined,
          profileUrl: user.photoURL ?? undefined,
          principal: user.principal ?? undefined,
        });
      } else if (user?.event == EventType.Login) {
        refetchUserInfo(queryClient);
        network.refetch();
        await updateTelegramId();
        loader.close();
      }
    } catch (err) {
      popup.open(
        <LoginFailurePopup
          logo={<GoogleIcon />}
          logoOrigin={<GoogleIcon />}
          title="Login failed"
          description="Google authentication failed"
          msg="Try again later."
          serviceName="Google"
          onRetry={handleGoogleSignIn}
        />,
      );
      logger.debug('failed to google sign in with error: ', err);
    }
  };

  const handleTelegramSignIn = async () => {
    const loader = popup.open(
      <LoaderPopup
        title="Sign in"
        description="Signing you in..."
        logo={<TelegramIcon width="50" height="50" />}
        logoOrigin={<TelegramIcon width={24} height={24} />}
        msg="Continue with Telegram"
        serviceName="Telegram"
      />,
    );

    try {
      const info = await send(anonKeyPair, '/api/login', '');
      if (!info && telegramRaw) {
        const params = new URLSearchParams(telegramRaw);
        const userJson = params.get('user');
        if (!userJson) {
          throw new Error('Telegram user data not found');
        }
        const user: TelegramUser = JSON.parse(userJson);
        openUserSetupPopup({
          id: 'telegram_user_setup',
          email: '',
          nickname: user.username ?? '',
          username: `${user.first_name} ${user.last_name ?? ''}`.trim(),
          profileUrl: user.photo_url ?? '',
          principal: anonKeyPair.getPrincipal().toText(),
        });
      } else {
        refetchUserInfo(queryClient);
        network.refetch();
        loader.close();
      }
    } catch (err) {
      popup.open(
        <LoginFailurePopup
          logo={<TelegramIcon width={24} height={24} />}
          logoOrigin={<TelegramIcon width={24} height={24} />}
          title="Login failed"
          description="Telegram authentication failed"
          msg="Try again later."
          serviceName="Telegram"
          onRetry={handleTelegramSignIn}
        />,
      );
      logger.debug('failed to telegram sign in with error: ', err);
    }
  };

  const handleSignUp = () => {
    logger.debug('Sign up button clicked');
    popup.open(<UserSetupPopup email="" />).withoutBackdropClose();
  };

  return (
    <div
      id={id}
      className="flex flex-col w-100 max-w-100 mx-1.25 max-mobile:!w-full max-mobile:!max-w-full gap-5"
    >
      <Col className="gap-4">
        <Row className="justify-start items-center text-sm gap-1">
          <label className="text-white font-medium">New user?</label>
          <button
            className="text-primary/70 hover:text-primary"
            onClick={handleSignUp}
          >
            Create an account
          </button>
        </Row>
        <Col>
          <label className="text-sm">Email address </label>
          <Input
            type="email"
            name="username"
            autoComplete="email"
            placeholder="Enter your email address"
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                e.preventDefault();
                handleContinue();
              }
            }}
          />
          {warning !== '' && (
            <div className="text-red-500 text-xs mt-1">{warning}</div>
          )}
        </Col>

        <Col aria-hidden={!showPassword} className="aria-hidden:hidden">
          <label className="text-sm">Password</label>
          <Input
            type="password"
            placeholder="Enter your password"
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={password}
            onChange={(e) => handleChangePassword(e.target.value)}
          />
          {passwordWarning !== '' && (
            <div className="text-red-500 text-xs mt-1">{passwordWarning}</div>
          )}
        </Col>

        <Row className="justify-end items-center text-sm">
          <Button
            variant={'rounded_secondary'}
            className="text-xs py-1.5 px-4"
            onClick={handleContinue}
          >
            {showPassword ? 'Sign in' : 'Continue'}
          </Button>
        </Row>
      </Col>
      {/* FIXME: In Telegram MiniApp, google login not working for now.  */}
      {!isWebView() ? (
        <>
          <div className="rule-with-text align-center text-center font-light">
            Or
          </div>
          <div className="flex flex-col gap-2.5">
            <LoginBox
              icon={<GoogleIcon />}
              label="Continue With Google"
              onClick={handleGoogleSignIn}
            />
          </div>
        </>
      ) : (
        <></>
      )}

      {!!telegramRaw && (
        <div className="flex flex-col gap-2.5">
          <LoginBox
            icon={<TelegramIcon width={24} height={24} />}
            label="Continue With Telegram"
            onClick={handleTelegramSignIn}
          />
        </div>
      )}

      <LoginPopupFooter />
    </div>
  );
};

export const LoginBox = ({ icon, label, onClick }: LoginBoxProps) => {
  return (
    <button
      className="flex flex-row w-full rounded-[10px] bg-[#000203] px-5 py-5.5 gap-5 cursor-pointer items-center"
      onClick={onClick}
    >
      {icon}
      <div className="font-semibold text-white text-base">{label}</div>
    </button>
  );
};
