'use client';

import { useRefundI18n } from './i18n';
import { Col } from '@/components/ui/col';
import Heading from '@/components/ui/heading';
import { Paragraph } from '@/components/ui/paragraph';
import { useTranslation } from 'react-i18next';

export function Refund() {
  const t = useRefundI18n();
  const { t: ft } = useTranslation('Footer', {
    keyPrefix: 'footer.values',
  });

  // Get business info from footer translations
  const companyName = ft('company_name');
  const email = ft('email');
  const address = ft('address');

  return (
    <div className="w-full min-h-screen bg-bg">
      <div className="py-12 px-4 mx-auto max-w-4xl">
        <Col className="gap-8">
          {/* Header */}
          <div className="text-center">
            <Heading variant="heading1" className="mb-4">
              {t.title}
            </Heading>
            <Paragraph className="text-muted-foreground">
              {t.lastUpdated}: {t.effectiveDate}
            </Paragraph>
          </div>

          {/* Content Sections */}
          <Col className="gap-6 mt-8">
            {/* 1. Introduction */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.introduction.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.introduction.content}
              </Paragraph>
            </section>

            {/* 2. Refund Eligibility */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.eligibility.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.eligibility.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.eligibility.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 3. Non-Refundable Items */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.nonRefundable.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.nonRefundable.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.nonRefundable.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 4. Refund Request Process */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.requestProcess.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.requestProcess.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.requestProcess.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 5. Refund Processing Time */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.processingTime.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.processingTime.content}
              </Paragraph>
            </section>

            {/* 6. Partial Refunds */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.partialRefunds.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.partialRefunds.content}
              </Paragraph>
            </section>

            {/* 7. Subscription Cancellations */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.subscriptions.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.subscriptions.content}
              </Paragraph>
            </section>

            {/* 8. Chargebacks and Disputes */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.chargebacks.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.chargebacks.content}
              </Paragraph>
            </section>

            {/* 9. Exceptions and Special Circumstances */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.exceptions.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.exceptions.content}
              </Paragraph>
            </section>

            {/* 10. Changes to This Policy */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.modifications.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.modifications.content}
              </Paragraph>
            </section>

            {/* 11. Contact Information */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.contact.title}
              </Heading>
              <Paragraph className="mb-4 leading-relaxed text-foreground">
                {t.sections.contact.content}
              </Paragraph>
              <Col className="gap-2 pl-4">
                <Paragraph className="text-foreground">
                  <span className="font-semibold">{companyName}</span>
                </Paragraph>
                <Paragraph className="text-foreground">
                  <span className="font-semibold">
                    {t.sections.contact.email}:
                  </span>{' '}
                  <a
                    href={`mailto:${email}`}
                    className="hover:underline text-primary"
                  >
                    {email}
                  </a>
                </Paragraph>
                <Paragraph className="text-foreground">
                  <span className="font-semibold">
                    {t.sections.contact.address}:
                  </span>{' '}
                  {address}
                </Paragraph>
              </Col>
            </section>
          </Col>

          {/* Footer Note */}
          <div className="pt-8 mt-12 border-t border-border">
            <Paragraph className="text-sm text-center text-muted-foreground">
              © {new Date().getFullYear()} {companyName}. All rights reserved.
            </Paragraph>
          </div>
        </Col>
      </div>
    </div>
  );
}
