'use client';
import React, { useState } from 'react';
import GoogleIcon from '@/assets/icons/google.svg';
import { LoginPopupFooter } from './login-popup-footer';
import { LoaderPopup } from './loader-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginFailurePopup } from './login-failure-popup';
import UserSetupPopup from './user-setup-popup';
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

interface LoginModalProps {
  id?: string;
}

interface LoginBoxProps {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
}

export const LoginModal = ({ id = 'login_popup' }: LoginModalProps) => {
  const popup = usePopup();
  const network = useNetwork();
  const anonKeyPair = useEd25519KeyPair();
  const queryClient = useQueryClient();
  const cli = useApolloClient();

  const { login, ed25519KeyPair } = useAuth();
  const [email, setEmail] = useState('');
  const [warning, setWarning] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [passwordWarning, setPasswordWarning] = useState('');

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
        popup
          .open(
            <UserSetupPopup
              email={user.email ?? ''}
              nickname={user.displayName ?? ''}
              profileUrl={user.photoURL ?? ''}
              principal={user.principal ?? ''}
            />,
          )
          .withoutBackdropClose();
      } else if (user?.event == EventType.Login) {
        refetchUserInfo(queryClient);
        network.refetch();
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
        />,
      );
      logger.debug('failed to google sign in with error: ', err);
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
