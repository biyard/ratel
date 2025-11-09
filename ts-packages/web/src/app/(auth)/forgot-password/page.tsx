'use client';

import { useState } from 'react';
import { useNavigate } from 'react-router';
import ForgotPasswordForm from '../_components/forgot-password-form';
import { sendVerificationCode } from '@/lib/api/ratel/auth.v3';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export default function ForgotPasswordPage() {
  const _navigate = useNavigate();
  const [_loading, setLoading] = useState(false);

  const handleSendCode = async (email: string) => {
    try {
      setLoading(true);
      await sendVerificationCode(email);
      showSuccessToast('Verification code sent to your email');
    } catch (error) {
      console.error('Failed to send verification code:', error);
      showErrorToast('Failed to send verification code. Please try again.');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="flex min-h-screen w-full items-center justify-center bg-background p-4">
      <div className="w-full max-w-md">
        <ForgotPasswordForm onSendCode={handleSendCode} />
      </div>
    </main>
  );
}
