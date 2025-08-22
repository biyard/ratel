'use client';

import { useState, useEffect } from 'react';

/**
 * 간단한 로딩 스피너 아이콘 컴포넌트
 */
const LoadingSpinner = () => (
  <svg
    className="animate-spin h-12 w-12 text-white"
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
  >
    <circle
      className="opacity-25"
      cx="12"
      cy="12"
      r="10"
      stroke="currentColor"
      strokeWidth="4"
    ></circle>
    <path
      className="opacity-75"
      fill="currentColor"
      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
    ></path>
  </svg>
);

/**
 * 체크마크 아이콘 컴포넌트
 */
const CheckIcon = () => (
  <svg
    className="h-16 w-16 text-primary"
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M5 13l4 4L19 7"
    />
  </svg>
);

export default function McpLoginPage() {
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false);
      window.location.href = 'claude://';
    }, 5000);

    return () => clearTimeout(timer);
  }, []);
  return (
    <main className="absolute top-0 left-0 h-screen w-screen bg-neutral-80 flex flex-col items-center justify-center">
      <div className="flex flex-col size-100 p-8 rounded-lg bg-neutral-800 justify-center items-center gap-10">
        {isLoading ? (
          <>
            <LoadingSpinner />
            <p className="text-xl font-semibold text-neutral-300">
              Logging in...
            </p>
          </>
        ) : (
          <>
            <CheckIcon />
            <p className="text-2xl font-bold text-primary text-center">
              MCP Authentication Completed
            </p>
          </>
        )}
      </div>
    </main>
  );
}
