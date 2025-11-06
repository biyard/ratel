import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { route } from '@/route';
import { useState } from 'react';
import { NavLink } from 'react-router';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginModal } from '@/components/popup/login-popup';
import { useTranslation } from 'react-i18next';
import Footer from './footer';

interface ResetPasswordFormProps {
  onResetPassword: (
    email: string,
    code: string,
    password: string,
  ) => Promise<void>;
}

export default function ResetPasswordForm({
  onResetPassword,
}: ResetPasswordFormProps) {
  const { t } = useTranslation('SignIn');
  const popup = usePopup();
  const [email, setEmail] = useState('');
  const [code, setCode] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async () => {
    setError('');

    if (!email.trim() || !code.trim() || !password.trim()) {
      setError(t('all_fields_required'));
      return;
    }

    if (password !== confirmPassword) {
      setError(t('passwords_do_not_match'));
      return;
    }

    if (password.length < 8) {
      setError(t('password_min_length'));
      return;
    }

    try {
      setLoading(true);
      await onResetPassword(email, code, password);
    } catch (err) {
      console.error('Failed to reset password:', err);
      setError(t('reset_password_failed'));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col w-full gap-5">
      <div className="flex flex-col gap-2">
        <h1 className="text-2xl font-bold text-white">
          {t('reset_password_title')}
        </h1>
        <p className="text-sm text-gray-400">
          {t('reset_password_description')}
        </p>
      </div>

      {error && (
        <div className="p-4 bg-red-900/20 border border-red-600/50 rounded-lg">
          <p className="text-sm text-red-400">{error}</p>
        </div>
      )}

      <Col className="gap-4">
        <Col>
          <label className="text-sm text-white">{t('email_address')}</label>
          <Input
            type="email"
            name="email"
            autoComplete="email"
            placeholder={t('email_address_hint')}
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </Col>

        <Col>
          <label className="text-sm text-white">{t('verification_code')}</label>
          <Input
            type="text"
            name="code"
            placeholder={t('verification_code_hint')}
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={code}
            onChange={(e) => setCode(e.target.value)}
            maxLength={6}
          />
        </Col>

        <Col>
          <label className="text-sm text-white">{t('new_password')}</label>
          <Input
            type="password"
            name="password"
            autoComplete="new-password"
            placeholder={t('new_password_hint')}
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </Col>

        <Col>
          <label className="text-sm text-white">
            {t('confirm_new_password')}
          </label>
          <Input
            type="password"
            name="confirmPassword"
            autoComplete="new-password"
            placeholder={t('confirm_password_hint')}
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleSubmit();
              }
            }}
          />
        </Col>

        <Row className="justify-between items-center text-sm gap-2">
          <NavLink to={route.forgotPassword()}>
            <Button variant="text" size="sm">
              {t('resend_code')}
            </Button>
          </NavLink>
          <Button
            variant="rounded_primary"
            size="sm"
            onClick={handleSubmit}
            disabled={
              loading ||
              !email.trim() ||
              !code.trim() ||
              !password.trim() ||
              !confirmPassword.trim()
            }
          >
            {loading ? t('resetting') : t('reset_password')}
          </Button>
        </Row>
      </Col>

      <div className="text-center">
        <button
          className="text-sm text-primary/70 hover:text-primary"
          onClick={() => popup.open(<LoginModal disableClose={false} />)}
        >
          {t('back_to_login')}
        </button>
      </div>

      <Footer />
    </div>
  );
}
