'use client';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useTranslations } from 'next-intl';
import React from 'react';

interface PlanProps {
  title: string;
  price: number;
  bg: string;
  text: string;
  chip: string;
  features: string[];
}

type CardProps = PlanProps & {
  onClick?: () => Promise<void> | void;
};

const plans: PlanProps[] = [
  {
    title: 'Personal',
    price: 20,
    bg: 'bg-white',
    text: 'text-[#000203]',
    chip: 'text-primary',
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Business',
    price: 50,
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
    title: 'Enterprise',
    price: 100,
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

export default function SubscribePage() {
  const { post } = useApiCall();
  const t = useTranslations('Subscribe');

  if (config.env !== 'local') {
    return (
      <div className="text-text-primary text-base font-medium">
        not implemented page
      </div>
    );
  }

  const handleSubscribeClick = async (i: number) => {
    const subscribeType =
      plans[i].title === 'Personal'
        ? 'personal'
        : plans[i].title === 'Business'
          ? 'business'
          : 'enterprise';

    const res = await post(ratelApi.binances.createSubscription(), {
      subscribe_type: subscribeType,
    });

    if (res?.checkout_url) window.location.href = res.checkout_url;
  };

  return (
    <div className="w-full h-fit bg-card-bg border-card-border px-4 py-12">
      <div className="mx-auto max-w-6xl">
        <div className="text-center">
          <div className="mx-auto inline-flex items-center rounded-full bg-follow-button-bg px-3 py-1 text-xs text-follow-button-text">
            {t('subscribe_desc_1')}
          </div>
          <h2 className="mt-4 text-2xl font-semibold text-text-primary">
            {t('subscribe_desc_2')}
          </h2>
          <p className="mt-2 text-sm text-desc-text">{t('subscribe_desc_3')}</p>
        </div>

        <div className="mt-8 grid w-full grid-cols-1 gap-6 md:grid-cols-3">
          {plans.map((p, i) => (
            <Card key={i} {...p} onClick={() => handleSubscribeClick(i)} />
          ))}
        </div>

        <div className="mt-8 flex items-center justify-center gap-3 text-xs text-desc-text">
          <span>{t('subscribe_info')}</span>
        </div>
      </div>
    </div>
  );
}

function Card({ title, price, bg, text, chip, features, onClick }: CardProps) {
  const isDark = text === 'text-white';
  return (
    <div className={`relative overflow-hidden rounded-[24px] ${bg}`}>
      <div className={`p-6 ${text}`}>
        <h3 className="text-base font-semibold">{title}</h3>
        <div className="mt-2 flex items-baseline gap-1">
          <span className={`text-3xl font-extrabold ${chip}`}>${price}</span>
        </div>

        <ul className="mt-5 space-y-3 text-sm/6">
          {features.map((f, i) => (
            <li key={i} className="flex items-start gap-3">
              <CheckIcon
                className={`mt-0.5 size-5 shrink-0 rounded-full p-[2px] ${isDark ? 'text-white bg-white/20' : 'text-emerald-600 bg-black/10'}`}
              />
              <span className={isDark ? 'text-white/90' : 'text-[#000203]'}>
                {f}
              </span>
            </li>
          ))}
        </ul>

        <button
          type="button"
          onClick={onClick}
          className={`mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition ${
            isDark
              ? 'bg-white/90 text-[#000203] hover:bg-white'
              : 'bg-primary text-black hover:bg-primary/80'
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
