import { CheckCircle } from '@/components/icons';
import { useMembershipsI18n } from '../i18n';

interface MembershipPriceCardProps {
  title: string;
  price: number;
  features: string[];
  buttonEnabled: boolean;
  selected: boolean;
  onClick?: () => Promise<void> | void;
  onUnsubscribe?: () => Promise<void> | void;
}

export default function MembershipPriceCard({
  title,
  price,
  features,
  buttonEnabled,
  selected,
  onClick,
  onUnsubscribe,
}: MembershipPriceCardProps) {
  const i18n = useMembershipsI18n();
  const isRefundable =
    selected && (title === 'PRO' || title === 'MAX' || title === 'VIP');

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
              <CheckCircle className="mt-0.5 size-5 shrink-0 rounded-full p-[2px] text-text-primary" />
              <span className="text-text-primary">{f}</span>
            </li>
          ))}
        </ul>

        {isRefundable ? (
          <button
            type="button"
            onClick={onUnsubscribe}
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition bg-red-500/90 hover:bg-red-500 text-white"
          >
            {i18n.unsubscribe}
          </button>
        ) : buttonEnabled ? (
          <button
            type="button"
            onClick={onClick}
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition bg-primary hover:bg-primary/80 text-text-primary"
          >
            {i18n.select}
          </button>
        ) : (
          <button
            type="button"
            disabled
            className="mt-6 inline-flex w-full items-center justify-center rounded-xl px-4 py-2 text-sm font-semibold shadow transition invisible pointer-events-none"
          >
            {i18n.select}
          </button>
        )}
      </div>
    </div>
  );
}
