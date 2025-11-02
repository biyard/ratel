import { useState } from 'react';
import { Col } from '../ui/col';
import { Row } from '../ui/row';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { LoginPopupFooter } from './login-popup-footer';
import { usePopup } from '@/lib/contexts/popup-service';
import { ratelSdk } from '@/lib/api/ratel';
import { sha3 } from '@/lib/utils';
import { useTranslation } from 'react-i18next';

enum ResetPasswordStep {
  EMAIL = 'email',
  CODE = 'code',
  PASSWORD = 'password',
  SUCCESS = 'success',
}

interface ForgotPasswordPopupProps {
  id?: string;
}

export const ForgotPasswordPopup = ({
  id = 'forgot_password_popup',
}: ForgotPasswordPopupProps) => {
  const { t } = useTranslation('SignIn');
  const popup = usePopup();
  const [step, setStep] = useState<ResetPasswordStep>(ResetPasswordStep.EMAIL);
  const [email, setEmail] = useState('');
  const [code, setCode] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const validateEmail = (email: string) => {
    // Use consistent email validation with login
    return email.includes('@') && email.length > 3;
  };

  const validatePassword = (pw: string) => {
    const regex =
      /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}[\]:;<>,.?~\\/-]).{8,}$/;
    return regex.test(pw);
  };

  const validateCode = (code: string) => {
    // Validate code is 6 characters of letters and numbers only
    return /^[A-Za-z0-9]{6}$/.test(code);
  };

  const handleSendCode = async () => {
    setError('');
    if (!validateEmail(email)) {
      setError(t('invalid_email_format'));
      return;
    }

    setIsLoading(true);
    try {
      await ratelSdk.auth.sendVerificationCode(email);
      setStep(ResetPasswordStep.CODE);
    } catch (err) {
      setError(t('forgot_password.send_code_failed'));
      console.error('Send code error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleVerifyCode = async () => {
    setError('');
    if (!validateCode(code)) {
      setError(t('forgot_password.invalid_code_format'));
      return;
    }

    setIsLoading(true);
    try {
      await ratelSdk.auth.verifyCode(email, code);
      setStep(ResetPasswordStep.PASSWORD);
    } catch (err) {
      setError(t('forgot_password.verify_code_failed'));
      console.error('Verify code error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleResetPassword = async () => {
    setError('');
    if (!validatePassword(password)) {
      setError(t('invalid_password_format'));
      return;
    }
    if (password !== confirmPassword) {
      setError(t('forgot_password.passwords_not_match'));
      return;
    }

    setIsLoading(true);
    try {
      const hashedPassword = sha3(password);
      await ratelSdk.auth.resetPassword(email, code, hashedPassword);
      setStep(ResetPasswordStep.SUCCESS);
    } catch (err) {
      setError(t('forgot_password.reset_password_failed'));
      console.error('Reset password error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent, action: () => void) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      action();
    }
  };

  const renderEmailStep = () => (
    <Col className="gap-4">
      <div className="text-center">
        <h2 className="text-2xl font-bold text-text-primary mb-2">
          Forgot your password?
        </h2>
        <p className="text-sm text-text-secondary">
          Enter your email address and we'll send you a verification code.
        </p>
      </div>
      <Col>
        <label className="text-sm">{t('email_address')}</label>
        <Input
          type="email"
          placeholder={t('email_address_hint')}
          className="w-full bg-input-box-bg border border-input-box-border rounded-[10px] px-5 py-5.5 text-text-primary font-light"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          onKeyDown={(e) => handleKeyDown(e, handleSendCode)}
        />
        {error && <div className="text-red-500 text-xs mt-1">{error}</div>}
      </Col>
      <Row className="justify-between items-center text-sm gap-2">
        <button
          className="text-text-secondary hover:text-text-primary"
          onClick={() => popup.close()}
        >
          Back to Sign in
        </button>
        <Button
          variant="rounded_secondary"
          className="text-xs py-1.5 px-4"
          onClick={handleSendCode}
          disabled={isLoading}
        >
          {isLoading ? 'Sending...' : 'Send Code'}
        </Button>
      </Row>
    </Col>
  );

  const renderCodeStep = () => (
    <Col className="gap-4">
      <div className="text-center">
        <h2 className="text-2xl font-bold text-text-primary mb-2">
          Enter verification code
        </h2>
        <p className="text-sm text-text-secondary">
          We've sent a 6-character code to {email}
        </p>
      </div>
      <Col>
        <label className="text-sm">Verification Code</label>
        <Input
          type="text"
          placeholder="Enter 6-character code"
          className="w-full bg-input-box-bg border border-input-box-border rounded-[10px] px-5 py-5.5 text-text-primary font-light text-center text-lg tracking-widest"
          value={code}
          onChange={(e) => setCode(e.target.value.toUpperCase())}
          onKeyDown={(e) => handleKeyDown(e, handleVerifyCode)}
          maxLength={6}
        />
        {error && <div className="text-red-500 text-xs mt-1">{error}</div>}
      </Col>
      <Row className="justify-between items-center text-sm gap-2">
        <button
          className="text-text-secondary hover:text-text-primary"
          onClick={() => setStep(ResetPasswordStep.EMAIL)}
        >
          Change email
        </button>
        <Button
          variant="rounded_secondary"
          className="text-xs py-1.5 px-4"
          onClick={handleVerifyCode}
          disabled={isLoading}
        >
          {isLoading ? 'Verifying...' : 'Verify Code'}
        </Button>
      </Row>
    </Col>
  );

  const renderPasswordStep = () => (
    <Col className="gap-4">
      <div className="text-center">
        <h2 className="text-2xl font-bold text-text-primary mb-2">
          Set new password
        </h2>
        <p className="text-sm text-text-secondary">
          Create a strong password for your account.
        </p>
      </div>
      <Col>
        <label className="text-sm">New Password</label>
        <Input
          type="password"
          placeholder="Enter your new password"
          className="w-full bg-input-box-bg border border-input-box-border rounded-[10px] px-5 py-5.5 text-text-primary font-light"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          onKeyDown={(e) => handleKeyDown(e, handleResetPassword)}
        />
      </Col>
      <Col>
        <label className="text-sm">Confirm Password</label>
        <Input
          type="password"
          placeholder="Re-enter your new password"
          className="w-full bg-input-box-bg border border-input-box-border rounded-[10px] px-5 py-5.5 text-text-primary font-light"
          value={confirmPassword}
          onChange={(e) => setConfirmPassword(e.target.value)}
          onKeyDown={(e) => handleKeyDown(e, handleResetPassword)}
        />
        {error && <div className="text-red-500 text-xs mt-1">{error}</div>}
      </Col>
      <Row className="justify-end items-center text-sm">
        <Button
          variant="rounded_secondary"
          className="text-xs py-1.5 px-4"
          onClick={handleResetPassword}
          disabled={isLoading}
        >
          {isLoading ? 'Resetting...' : 'Reset Password'}
        </Button>
      </Row>
    </Col>
  );

  const renderSuccessStep = () => (
    <Col className="gap-4">
      <div className="text-center">
        <div className="text-6xl mb-4">✓</div>
        <h2 className="text-2xl font-bold text-text-primary mb-2">
          Password reset successful!
        </h2>
        <p className="text-sm text-text-secondary">
          You can now log in with your new password.
        </p>
      </div>
      <Row className="justify-center items-center text-sm">
        <Button
          variant="rounded_secondary"
          className="text-xs py-1.5 px-4"
          onClick={() => popup.close()}
        >
          Back to Sign in
        </Button>
      </Row>
    </Col>
  );

  const renderStep = () => {
    switch (step) {
      case ResetPasswordStep.EMAIL:
        return renderEmailStep();
      case ResetPasswordStep.CODE:
        return renderCodeStep();
      case ResetPasswordStep.PASSWORD:
        return renderPasswordStep();
      case ResetPasswordStep.SUCCESS:
        return renderSuccessStep();
      default:
        return renderEmailStep();
    }
  };

  return (
    <div
      id={id}
      className="flex flex-col w-100 max-w-100 mx-1.25 max-mobile:!w-full max-mobile:!max-w-full gap-5"
    >
      {renderStep()}
      {step !== ResetPasswordStep.SUCCESS && <LoginPopupFooter />}
    </div>
  );
};
