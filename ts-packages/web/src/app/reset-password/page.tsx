'use client';

import ResetPasswordForm from '../(auth)/_components/reset-password-form';
import { useNavigate } from 'react-router';
import { route } from '@/route';

export default function ResetPasswordPage() {
  const navigate = useNavigate();

  const handleSuccess = () => {
    navigate(route.login());
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-gradient-radial from-[rgba(252,179,0,0.15)] via-[rgba(252,179,0,0.05)] to-transparent">
      <div className="flex flex-col gap-10 w-full max-w-100 mx-auto p-10 rounded-2xl bg-[#0A0B0D]/80 backdrop-blur-md border border-gray-800">
        <div className="text-center">
          <h1 className="text-3xl font-bold text-white mb-2">Reset Password</h1>
          <p className="text-gray-400">
            Enter your email to receive a verification code
          </p>
        </div>

        <ResetPasswordForm onSuccess={handleSuccess} />
      </div>
    </div>
  );
}
