'use client';
import React from 'react';

interface PlanProps {
  title: string;
  price: number;
  accent: string;
  bg: string;
  text: string;
  chip: string;
  features: string[];
}

const plans: PlanProps[] = [
  {
    title: 'Personal',
    price: 19,
    accent: 'from-amber-500 to-amber-400',
    bg: 'bg-white',
    text: 'text-zinc-800',
    chip: 'text-amber-500',
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Business',
    price: 39,
    accent: 'from-sky-500 to-sky-400',
    bg: 'bg-[#2A4D73]',
    text: 'text-white',
    chip: 'text-white',
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Startup',
    price: 49,
    accent: 'from-fuchsia-500 to-fuchsia-400',
    bg: 'bg-[#7E4774]',
    text: 'text-white',
    chip: 'text-white',
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
];

export default function PricingCardsDark() {
  return (
    <div className="min-h-screen w-full bg-[#0B0D10] text-zinc-200 px-4 py-12">
      <div className="mx-auto max-w-6xl">
        <div className="text-center">
          <div className="mx-auto inline-flex items-center rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-400">
            전 세계에서 애용하는 프리미엄
          </div>
          <h2 className="mt-4 text-2xl font-semibold text-white">
            수백 만명이 선택한 Ratel 프리미엄으로 앞서가세요.
          </h2>
          <p className="mt-2 text-sm text-zinc-400">
            지금 1개월 무료 이용을 시작하세요.
          </p>
        </div>

        <div className="mt-8 grid w-full grid-cols-1 gap-6 md:grid-cols-3">
          {plans.map((p, i) => (
            <Card key={i} {...p} />
          ))}
        </div>

        <div className="mt-8 flex items-center justify-center gap-3 text-xs text-zinc-500">
          <span>계획은 언제든 변경 또는 취소할 수 있습니다.</span>
        </div>
      </div>
    </div>
  );
}

function Card({ title, price, accent, bg, text, chip, features }: PlanProps) {
  const isDark = text === 'text-white';
  return (
    <div
      className={`relative overflow-hidden rounded-[24px] shadow-xl ring-1 ring-black/10 ${bg}`}
    >
      <div
        className={`pointer-events-none absolute inset-x-0 top-0 h-2 bg-gradient-to-r ${accent}`}
      />

      <div className={`p-6 ${text}`}>
        <h3 className="text-base font-semibold">{title}</h3>
        <div className="mt-2 flex items-baseline gap-1">
          <span className={`text-3xl font-extrabold ${chip}`}>${price}</span>
        </div>

        <ul className="mt-5 space-y-3 text-sm/6">
          {features.map((f, i) => (
            <li key={i} className="flex items-start gap-3">
              <CheckIcon
                className={`mt-0.5 size-5 shrink-0 rounded-full p-[2px] ${
                  isDark
                    ? 'text-white bg-white/20'
                    : 'text-emerald-600 bg-black/10'
                }`}
              />
              <span className={isDark ? 'text-white/90' : 'text-zinc-700'}>
                {f}
              </span>
            </li>
          ))}
        </ul>

        <button
          type="button"
          className={`mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition ${
            isDark
              ? 'bg-white/90 text-zinc-900 hover:bg-white'
              : 'bg-amber-500 text-black hover:bg-amber-400'
          }`}
        >
          Select
        </button>
      </div>
    </div>
  );
}

function CheckIcon({ className = '' }: { className?: string }) {
  return (
    <svg viewBox="0 0 20 20" fill="currentColor" className={className}>
      <path
        fillRule="evenodd"
        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 10-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
        clipRule="evenodd"
      />
    </svg>
  );
}
