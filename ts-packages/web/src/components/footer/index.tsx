import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';

export type FooterProps = React.HTMLAttributes<HTMLDivElement> & {
  info?: {
    companyName?: string;
    ceo?: string;
    businessRegistration?: string;
    address?: string;
    phone?: string;
    email?: string;
    termsUrl?: string;
    privacyUrl?: string;
    refundUrl?: string;
  };
};

export default function Footer({ info, className, ...props }: FooterProps) {
  const { t } = useTranslation('Footer', { keyPrefix: 'footer' });

  // Default business information from config
  const defaultInfo = {
    companyName: t('values.company_name'),
    ceo: t('values.ceo'),
    businessRegistration: t('values.business_registration'),
    address: t('values.address'),
    phone: t('values.phone'),
    email: t('values.email'),
    termsUrl: '/terms',
    privacyUrl: '/privacy',
    refundUrl: '/refund',
  };

  const businessInfo = { ...defaultInfo, ...info };
  const currentYear = new Date().getFullYear();

  return (
    <footer
      className={`w-full bg-component-bg py-8 px-4 ${className || ''}`}
      {...props}
    >
      <Col className="gap-6 mx-auto max-w-7xl">
        {/* Business Information */}
        <div className="grid grid-cols-1 gap-4 text-sm md:grid-cols-2 text-muted-foreground">
          <div className="space-y-2">
            <p className="font-semibold text-foreground">
              {businessInfo.companyName}
            </p>
            <div className="space-y-1">
              <p>
                <span className="font-medium">{t('ceo')}:</span>{' '}
                {businessInfo.ceo}
              </p>
              <p>
                <span className="font-medium">
                  {t('business_registration')}:
                </span>{' '}
                {businessInfo.businessRegistration}
              </p>
              <p>
                <span className="font-medium">{t('phone')}:</span>{' '}
                {businessInfo.phone}
              </p>
              <p>
                <span className="font-medium">{t('email')}:</span>{' '}
                {businessInfo.email}
              </p>
            </div>
          </div>
          <div className="space-y-2">
            <p className="font-medium text-foreground">{t('address')}</p>
            <p>{businessInfo.address}</p>
          </div>
        </div>

        {/* Links Section */}
        <div className="flex flex-wrap gap-4 text-sm">
          <a
            href={businessInfo.termsUrl}
            className="underline transition-colors text-muted-foreground hover:text-foreground"
            target="_blank"
            rel="noopener noreferrer"
          >
            {t('terms_of_use')}
          </a>
          <span className="text-muted-foreground">|</span>
          <a
            href={businessInfo.privacyUrl}
            className="font-semibold underline transition-colors text-muted-foreground hover:text-foreground"
            target="_blank"
            rel="noopener noreferrer"
          >
            {t('privacy_policy')}
          </a>
          <span className="text-muted-foreground">|</span>
          <a
            href={businessInfo.refundUrl}
            className="underline transition-colors text-muted-foreground hover:text-foreground"
            target="_blank"
            rel="noopener noreferrer"
          >
            {t('refund_policy')}
          </a>
        </div>

        {/* Copyright */}
        <div className="pt-4 text-xs text-muted-foreground">
          {t('copyright', {
            year: currentYear,
            companyName: businessInfo.companyName,
          })}
        </div>
      </Col>
    </footer>
  );
}
