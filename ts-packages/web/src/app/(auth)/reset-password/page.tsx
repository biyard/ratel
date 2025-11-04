'use client';

import { useState } from 'react';
import { useNavigate } from 'react-router';
import ResetPasswordForm from '../_components/reset-password-form';
import { resetPassword } from '@/lib/api/ratel/auth.v3';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { route } from '@/route';

export default function ResetPasswordPage() {
  const navigate = useNavigate();
  const [_loading, setLoading] = useState(false);

  const handleResetPassword = async (
    email: string,
    code: string,
    password: string,
  ) => {
    try {
      setLoading(true);
      await resetPassword({ email, code, password });
      showSuccessToast(
        'Password reset successfully! Please login with your new password.',
      );
      setTimeout(() => {
        navigate(route.login());
      }, 1500);
    } catch (error) {
      console.error('Failed to reset password:', error);
      showErrorToast(
        'Failed to reset password. Please check your code and try again.',
      );
      throw error;
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="flex min-h-screen w-full items-center justify-center bg-background p-4">
      <div className="w-full max-w-md">
        <ResetPasswordForm onResetPassword={handleResetPassword} />
      </div>
    </main>
  );
}
