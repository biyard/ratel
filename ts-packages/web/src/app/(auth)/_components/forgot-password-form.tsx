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

interface ForgotPasswordFormProps {
  onSendCode: (email: string) => Promise<void>;
}

export default function ForgotPasswordForm({
  onSendCode,
}: ForgotPasswordFormProps) {
  const { t } = useTranslation('SignIn');
  const popup = usePopup();
  const [email, setEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [sent, setSent] = useState(false);

  const handleSubmit = async () => {
    if (!email.trim()) return;

    try {
      setLoading(true);
      await onSendCode(email);
      setSent(true);
    } catch (error) {
      console.error('Failed to send verification code:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col w-full gap-5">
      <div className="flex flex-col gap-2">
        <h1 className="text-2xl font-bold text-white">
          {t('forgot_password_title')}
        </h1>
        <p className="text-sm text-gray-400">
          {sent
            ? t('verification_code_sent_message')
            : t('forgot_password_description')}
        </p>
      </div>

      {sent ? (
        <div className="flex flex-col gap-4">
          <div className="p-4 bg-green-900/20 border border-green-600/50 rounded-lg">
            <p className="text-sm text-green-400">
              {t('verification_code_sent_to', { email })}
            </p>
          </div>

          <Row className="justify-between items-center gap-2">
            <Button
              variant="rounded_secondary"
              size="sm"
              className="flex-1"
              onClick={() => popup.open(<LoginModal disableClose={false} />)}
            >
              {t('back_to_login')}
            </Button>
            <NavLink to={route.resetPassword()} className="flex-1">
              <Button variant="rounded_primary" size="sm" className="w-full">
                {t('reset_password')}
              </Button>
            </NavLink>
          </Row>
        </div>
      ) : (
        <>
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
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    handleSubmit();
                  }
                }}
              />
            </Col>

            <Row className="justify-end items-center text-sm gap-2">
              <Button
                variant="text"
                size="sm"
                onClick={() => popup.open(<LoginModal disableClose={false} />)}
              >
                {t('back_to_login')}
              </Button>
              <Button
                variant="rounded_primary"
                size="sm"
                onClick={handleSubmit}
                disabled={loading || !email.trim()}
              >
                {loading ? t('sending') : t('send_code')}
              </Button>
            </Row>
          </Col>
        </>
      )}

      <Footer />
    </div>
  );
}
