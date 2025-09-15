'use client';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useTranslations } from 'next-intl';
import React from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { QK_USERS_GET_INFO } from '@/constants';
import { config } from '@/config';

interface PlanProps {
  title: string;
  price: number;
  features: string[];
}

type CardProps = PlanProps & {
  buttonEnabled: boolean;
  selected: boolean;
  onClick?: () => Promise<void> | void;
};

const plans: PlanProps[] = [
  {
    title: 'Free',
    price: 0,
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Pro',
    price: 20,
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Premium',
    price: 50,
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Vip',
    price: 100,
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
  {
    title: 'Admin',
    price: 0,
    features: [
      'Business Solutions',
      'Regular News Update',
      'Live Chat and Support',
      'Social Media Marketing',
    ],
  },
];

export default function SubscribePage() {
  const qc = useQueryClient();
  const { post } = useApiCall();
  const userInfo = useSuspenseUserInfo();
  const { data: user } = userInfo;

  const t = useTranslations('Subscribe');

  const selected =
    user.membership == 1
      ? 'Free'
      : user.membership == 2
        ? 'Pro'
        : user.membership == 3
          ? 'Premium'
          : user.membership == 4
            ? 'Vip'
            : 'Admin';

  if (config.env !== 'local') {
    return (
      <div className="text-text-primary text-base font-medium">
        not implemented page
      </div>
    );
  }

  const handleSubscribeClick = async (i: number) => {
    const subscribeType =
      plans[i].title === 'Pro'
        ? 'pro'
        : plans[i].title === 'Premium'
          ? 'premium'
          : 'vip';

    const res = await post(ratelApi.binances.createSubscription(), {
      subscribe_type: subscribeType,
    });
    if (res?.checkout_url) {
      window.location.href = res.checkout_url;
      await qc.invalidateQueries({ queryKey: [QK_USERS_GET_INFO] });
    }
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
        <div className="mt-8 grid w-full grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-5">
          {plans.map((p, i) => (
            <Card
              key={i}
              {...p}
              buttonEnabled={
                (selected == 'Free' || selected == 'Admin') && p.title != 'Free'
              }
              selected={p.title == selected}
              onClick={() => handleSubscribeClick(i)}
            />
          ))}
        </div>

        <div className="mt-8 flex items-center justify-center gap-3 text-xs text-desc-text">
          <span>{t('subscribe_info')}</span>
        </div>
      </div>
    </div>
  );
}

function Card({
  title,
  price,
  features,
  buttonEnabled,
  selected,
  onClick,
}: CardProps) {
  const t = useTranslations('Subscribe');
  const isRefundable =
    selected && (title === 'Pro' || title === 'Premium' || title === 'Vip');

  return (
    <div
      className={`relative overflow-hidden rounded-2xl  ${selected ? 'bg-primary/10' : 'bg-card-bg'} border border-card-enable-border ring-1 ring-white/10`}
    >
      <div className="p-6 text-text-primary">
        <h3 className="text-xl font-semibold tracking-tight">{title}</h3>

        <div className="mt-3 flex items-baseline gap-2">
          <span className="text-3xl font-extrabold">${price}</span>
          {/* <span className="text-base font-medium opacity-80">/mo</span> */}
        </div>
        {/* <div className="mt-1 text-xs opacity-70">billed yearly</div> */}

        <ul className="mt-6 space-y-3 text-sm/6">
          {features.map((f, i) => (
            <li key={i} className="flex items-start gap-3">
              <CheckIcon className="mt-0.5 size-5 shrink-0 rounded-full p-[2px] text-text-primary bg-text-primary/20" />
              <span className="text-text-primary">{f}</span>
            </li>
          ))}
        </ul>

        {isRefundable ? (
          <button
            type="button"
            onClick={onClick}
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition bg-red-500/90 hover:bg-red-500 text-white"
          >
            {t('unsubscribe')}
          </button>
        ) : buttonEnabled ? (
          <button
            type="button"
            onClick={onClick}
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition bg-primary hover:bg-primary/80 text-text-primary"
          >
            {t('select')}
          </button>
        ) : (
          <button
            type="button"
            disabled
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition invisible pointer-events-none"
          >
            {t('select')}
          </button>
        )}
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
