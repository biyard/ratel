import { useCallback, useState } from 'react';
import GoogleIcon from '@/assets/icons/google.svg?react';
import { LoginPopupFooter } from './login-popup-footer';
import { LoaderPopup } from './loader-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginFailurePopup } from './login-failure-popup';
import { ForgotPasswordPopup } from './forgot-password-popup';
import UserSetupPopup, { type UserSetupPopupProps } from './user-setup-popup';
import { logger } from '@/lib/logger';
import { useAuth, useEd25519KeyPair } from '@/lib/contexts/auth-context';
import { AuthUserInfo } from '@/lib/service/firebase-service';
import { send } from '@/lib/api/send';
import { Col } from '../ui/col';
import { Row } from '../ui/row';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { sha3 } from '@/lib/utils';
import { isWebView } from '@/lib/webview-utils';
import { TelegramIcon } from '../icons';
import { type User as TelegramUser } from '@tma.js/sdk-react';
import { getQueryClient } from '@/providers/getQueryClient';
import { useTranslation } from 'react-i18next';
import { feedKeys } from '@/constants';
import { FeedStatus } from '@/features/posts/types/post';
import { useApiCall } from '@/lib/api/use-send';
import { OAuthProvider } from '@/types/oauth-provider';
import { ratelSdk } from '@/lib/api/ratel';
import { useNetwork } from '@/app/(social)/my-network/_hook/use-network';
import { refetchUserInfo } from '@/hooks/use-user-info';

interface LoginModalProps {
  id?: string;
  disableClose?: boolean;
}

interface LoginBoxProps {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
}
//FIXME: In Telegram MiniApp, google login not working for now.
//Please use `/login` route instead of LoginPopup in Telegram MiniApp.

export const LoginModal = ({
  id = 'login_popup',
  disableClose = false,
}: LoginModalProps) => {
  const { t } = useTranslation('SignIn');
  const { t: signupTranslate } = useTranslation('Signup');
  const popup = usePopup();
  const network = useNetwork();
  const anonKeyPair = useEd25519KeyPair();
  const queryClient = getQueryClient();

  const { login, ed25519KeyPair, telegramRaw } = useAuth();
  const [email, setEmail] = useState('');
  const [warning, setWarning] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [passwordWarning, setPasswordWarning] = useState('');
  const [loginError, setLoginError] = useState('');
  const { post } = useApiCall();

  const updateTelegramId = async () => {
    if (telegramRaw) {
      logger.error('Updating Telegram ID is not implemented yet.');
      // try {
      //   const response = await fetch(
      //     `${process.env.NEXT_PUBLIC_API_URL}${ratelApi.users.updateTelegramId()}`,
      //     {
      //       method: 'POST',
      //       headers: {
      //         'Content-Type': 'application/json',
      //       },
      //       credentials: 'include',
      //       body: JSON.stringify({
      //         telegram_raw: telegramRaw,
      //       }),
      //     },
      //   );
      //   if (!response.ok) {
      //     logger.error('Failed to update Telegram ID:', response.status);
      //   }
      // } catch (error) {
      //   logger.error('Error updating Telegram ID:', error);
      // }
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
      /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}[\]:;<>,.?~\\/-]).{8,}$/;
    return regex.test(pw);
  };

  const handleChangePassword = async (pw: string) => {
    setPassword(pw);
    setLoginError('');

    if (!validatePassword(pw)) {
      setPasswordWarning(t('invalid_password_format'));
      return;
    } else {
      setPasswordWarning('');
    }
  };

  const handleSignIn = async () => {
    setLoginError('');
    const hashedPassword = sha3(password);
    const info = await post('/v3/auth/login', {
      email,
      password: hashedPassword,
    });

    if (info) {
      console.log('Sign in user info:', info);
      refetchUserInfo(queryClient);
      // TODO: Update to use v3 feed query keys without userId
      await queryClient.invalidateQueries({
        queryKey: feedKeys.list({
          status: FeedStatus.Published,
        }),
      });
      await updateTelegramId();
      network.refetch();
      popup.close();
      window.location.href = '/';
    } else {
      setLoginError(t('invalid_credentials'));
    }
  };

  const handleContinue = async () => {
    if (showPassword) {
      handleSignIn();
      return;
    }

    // check if email is valid
    if (!email || !email.includes('@')) {
      setWarning(t('invalid_email_format'));
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
        description={signupTranslate('signing_in')}
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
      try {
        await ratelSdk.auth.loginWithOAuth(
          OAuthProvider.Google,
          user.accessToken,
        );

        refetchUserInfo(queryClient);
        // TODO: Update to use v3 feed query keys without userId
        await queryClient.invalidateQueries({
          queryKey: feedKeys.list({
            status: FeedStatus.Published,
          }),
        });
        network.refetch();
        await updateTelegramId();
        loader.close();
      } catch (e) {
        logger.debug('Ratel login with Google failed:', e);
        openUserSetupPopup({
          email: user.email ?? '',
          nickname: user.displayName ?? undefined,
          profileUrl: user.photoURL ?? undefined,
          principal: user.principal ?? undefined,
          idToken: user.idToken,
          accessToken: user.accessToken,
          provider: OAuthProvider.Google,
        });
      }
    } catch (err) {
      popup.open(
        <LoginFailurePopup
          logo={<GoogleIcon />}
          logoOrigin={<GoogleIcon />}
          title={signupTranslate('login_failed')}
          description={signupTranslate('google_login_failed_description')}
          msg={signupTranslate('login_failed_msg')}
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
        description={signupTranslate('signing_in')}
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
        // TODO: Update to use v3 feed query keys without userId
        await queryClient.invalidateQueries({
          queryKey: feedKeys.list({
            status: FeedStatus.Published,
          }),
        });

        network.refetch();
        loader.close();
      }
    } catch (err) {
      popup.open(
        <LoginFailurePopup
          logo={<TelegramIcon width={24} height={24} />}
          logoOrigin={<TelegramIcon width={24} height={24} />}
          title={signupTranslate('login_failed')}
          description={signupTranslate('telegram_login_failed_description')}
          msg={signupTranslate('login_failed_msg')}
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

  const handleForgotPassword = () => {
    logger.debug('Forgot password button clicked');
    popup.open(<ForgotPasswordPopup />).withoutBackdropClose();
  };

  return (
    <div
      id={id}
      className="flex flex-col w-100 max-w-100 mx-1.25 max-mobile:!w-full max-mobile:!max-w-full gap-5"
    >
      <Col className="gap-4">
        <Row className="justify-start items-center text-sm gap-1">
          <label className="text-text-primary font-medium">
            {t('new_user')}
          </label>
          <button
            className="text-primary/70 light:text-primary hover:text-primary"
            onClick={handleSignUp}
          >
            {t('create_account')}
          </button>
        </Row>
        <Col>
          <label className="text-sm">{t('email_address')} </label>
          <Input
            type="email"
            name="username"
            autoComplete="email"
            placeholder={t('email_address_hint')}
            className="w-full bg-input-box-bg border border-input-box-border rounded-[10px] px-5 py-5.5 text-text-primary font-light"
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
          <label className="text-sm">{t('password')}</label>
          <Input
            type="password"
            placeholder={t('password_hint')}
            className="w-full rounded-[10px] px-5 py-5.5 font-light"
            value={password}
            onChange={(e) => handleChangePassword(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                e.preventDefault();
                handleSignIn();
              }
            }}
          />
          {passwordWarning !== '' && (
            <div className="text-red-500 text-xs mt-1">{passwordWarning}</div>
          )}
          {loginError !== '' && (
            <div className="text-red-500 text-xs mt-1">{loginError}</div>
          )}
          <Row className="justify-start mt-1">
            <button
              className="text-primary/70 hover:text-primary text-xs"
              onClick={handleForgotPassword}
            >
              Forgot your password?
            </button>
          </Row>
        </Col>

        <Row className="justify-end items-center text-sm">
          <Button
            variant={'rounded_secondary'}
            className="text-xs py-1.5 px-4 light:bg-neutral-600"
            onClick={handleContinue}
          >
            {showPassword ? t('sign_in') : t('continue')}
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
