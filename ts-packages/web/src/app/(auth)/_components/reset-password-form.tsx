import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { useState } from 'react';
import { validateEmail, validatePassword } from '@/lib/valid-utils';
import { sha3 } from '@/lib/utils';
import Footer from './footer';
import LabelInput from './label-input';
import {
  sendVerificationCode,
  verifyCode,
  resetPassword,
} from '@/lib/api/ratel/auth.v3';
import { NavLink } from 'react-router';
import { route } from '@/route';
import { useTranslation } from 'react-i18next';

const Warning = {
  Email: 'Email',
  Code: 'Code',
  Password: 'Password',
  ConfirmPassword: 'ConfirmPassword',
} as const;

type Warning = (typeof Warning)[keyof typeof Warning];

enum ResetStep {
  Email = 'Email',
  Code = 'Code',
  NewPassword = 'NewPassword',
  Success = 'Success',
}

interface ResetPasswordFormProps {
  onSuccess?: () => void;
}

export default function ResetPasswordForm({
  onSuccess,
}: ResetPasswordFormProps) {
  const { t } = useTranslation('ResetPassword');

  const [currentStep, setCurrentStep] = useState<ResetStep>(ResetStep.Email);
  const [errors, setErrors] = useState<Partial<Record<Warning, string>>>({});

  // Form fields
  const [email, setEmail] = useState('');
  const [code, setCode] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');

  const [sendButtonClicked, setSendButtonClicked] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

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
    if (!email) {
      insertError(Warning.Email, t('Email.error.required'));
      return;
    }

    if (!validateEmail(email)) {
      insertError(Warning.Email, t('Email.error.invalid'));
      return;
    }

    setIsLoading(true);
    try {
      await sendVerificationCode(email);
      setSendButtonClicked(true);
      popError(Warning.Email);
    } catch (error) {
      console.error(error);
      insertError(Warning.Email, t('Email.error.send_failed'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleVerifyCode = async () => {
    if (!code) {
      insertError(Warning.Code, t('Code.error.required'));
      return;
    }

    setIsLoading(true);
    try {
      await verifyCode(email, code);
      setCurrentStep(ResetStep.NewPassword);
      popError(Warning.Code);
    } catch (error) {
      console.error(error);
      insertError(Warning.Code, t('Code.error.invalid'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleResetPassword = async () => {
    // Validate password
    if (!password) {
      insertError(Warning.Password, t('Password.error.required'));
      return;
    }

    if (!validatePassword(password)) {
      insertError(Warning.Password, t('Password.error.invalid'));
      return;
    }

    // Validate confirm password
    if (!confirmPassword) {
      insertError(Warning.ConfirmPassword, t('ConfirmPassword.error.required'));
      return;
    }

    if (password !== confirmPassword) {
      insertError(Warning.ConfirmPassword, t('ConfirmPassword.error.mismatch'));
      return;
    }

    setIsLoading(true);
    try {
      const hashedPassword = sha3(password);
      await resetPassword(email, hashedPassword, code);
      setCurrentStep(ResetStep.Success);
      popError(Warning.Password);
      popError(Warning.ConfirmPassword);
      if (onSuccess) {
        setTimeout(() => {
          onSuccess();
        }, 2000);
      }
    } catch (error) {
      console.error(error);
      insertError(Warning.Password, t('Password.error.reset_failed'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleUpdateEmail = (value: string) => {
    setEmail(value);
    if (validateEmail(value)) {
      popError(Warning.Email);
    } else if (value) {
      insertError(Warning.Email, t('Email.error.invalid'));
    }
    // Reset state if email changed
    setSendButtonClicked(false);
    setCode('');
  };

  const handleUpdateCode = (value: string) => {
    setCode(value);
    popError(Warning.Code);
  };

  const handleUpdatePassword = (value: string) => {
    setPassword(value);
    if (!validatePassword(value)) {
      insertError(Warning.Password, t('Password.error.invalid'));
    } else {
      popError(Warning.Password);
    }
  };

  const handleUpdateConfirmPassword = (value: string) => {
    setConfirmPassword(value);
    if (value && value !== password) {
      insertError(Warning.ConfirmPassword, t('ConfirmPassword.error.mismatch'));
    } else {
      popError(Warning.ConfirmPassword);
    }
  };

  if (currentStep === ResetStep.Success) {
    return (
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-col gap-3.75 text-center">
          <div className="text-2xl font-semibold text-white">
            {t('Success.title')}
          </div>
          <div className="text-base text-gray-400">{t('Success.message')}</div>
        </div>

        <NavLink to={route.login()}>
          <Button variant="rounded_secondary" className="w-full">
            {t('Success.back_to_login')}
          </Button>
        </NavLink>

        <Footer />
      </div>
    );
  }

  return (
    <div className="flex flex-col w-full gap-5">
      <Row className="justify-start items-center text-sm gap-1">
        <label className="text-white font-medium">
          {t('remember_password')}
        </label>
        <NavLink to={route.login()}>
          <button className="text-primary/70 hover:text-primary">
            {t('back_to_login')}
          </button>
        </NavLink>
      </Row>

      {currentStep === ResetStep.Email && (
        <div className="flex flex-col gap-3.75">
          <div className="text-base text-gray-400">{t('Email.description')}</div>

          <LabelInput
            id="input-email"
            type="email"
            label={t('Email.label')}
            placeholder={t('Email.placeholder')}
            value={email}
            onChange={handleUpdateEmail}
            errorMessage={errors[Warning.Email]}
          >
            <Button
              className="w-20"
              variant="rounded_secondary"
              size="sm"
              onClick={handleSendCode}
              disabled={
                errors[Warning.Email] !== undefined ||
                !email ||
                sendButtonClicked ||
                isLoading
              }
            >
              {isLoading ? t('Email.sending') : t('Email.send_button_label')}
            </Button>
          </LabelInput>

          {sendButtonClicked && (
            <>
              <LabelInput
                id="input-code"
                label={t('Code.label')}
                placeholder={t('Code.placeholder')}
                value={code}
                onChange={handleUpdateCode}
                errorMessage={errors[Warning.Code]}
              >
                <Button
                  className="w-20"
                  variant="rounded_secondary"
                  size="sm"
                  disabled={code.length === 0 || isLoading}
                  onClick={handleVerifyCode}
                >
                  {isLoading ? t('Code.verifying') : t('Code.verify_button_label')}
                </Button>
              </LabelInput>
            </>
          )}
        </div>
      )}

      {currentStep === ResetStep.NewPassword && (
        <div className="flex flex-col gap-3.75">
          <div className="text-base text-gray-400">
            {t('Password.description')}
          </div>

          <LabelInput
            id="input-new-password"
            type="password"
            label={t('Password.label')}
            placeholder={t('Password.placeholder')}
            value={password}
            onChange={handleUpdatePassword}
            errorMessage={errors[Warning.Password]}
          />

          <LabelInput
            id="input-confirm-password"
            type="password"
            label={t('ConfirmPassword.label')}
            placeholder={t('ConfirmPassword.placeholder')}
            value={confirmPassword}
            onChange={handleUpdateConfirmPassword}
            errorMessage={errors[Warning.ConfirmPassword]}
          />

          <Row className="justify-end items-center text-sm">
            <Button
              variant="rounded_secondary"
              size="sm"
              onClick={handleResetPassword}
              disabled={
                !password ||
                !confirmPassword ||
                !!errors[Warning.Password] ||
                !!errors[Warning.ConfirmPassword] ||
                isLoading
              }
            >
              {isLoading ? t('Password.resetting') : t('Password.reset_button_label')}
            </Button>
          </Row>
        </div>
      )}

      <Footer />
    </div>
  );
}
