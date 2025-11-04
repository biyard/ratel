'use client';

import { useTermsI18n } from './i18n';
import { Col } from '@/components/ui/col';
import Heading from '@/components/ui/heading';
import { Paragraph } from '@/components/ui/paragraph';
import { config } from '@/config';
import { useTranslation } from 'react-i18next';

export function Terms() {
  const t = useTermsI18n();
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
            {/* 1. Acceptance of Terms */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.acceptance.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.acceptance.content}
              </Paragraph>
            </section>

            {/* 2. Description of Service */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.serviceDescription.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.serviceDescription.content}
              </Paragraph>
            </section>

            {/* 3. User Accounts */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.userAccounts.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.userAccounts.content}
              </Paragraph>
            </section>

            {/* 4. User Conduct */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.userConduct.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.userConduct.content}
              </Paragraph>
            </section>

            {/* 5. Intellectual Property Rights */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.intellectualProperty.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.intellectualProperty.content}
              </Paragraph>
            </section>

            {/* 6. User-Generated Content */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.userContent.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.userContent.content}
              </Paragraph>
            </section>

            {/* 7. Privacy Policy */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.privacyPolicy.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.privacyPolicy.content}
              </Paragraph>
            </section>

            {/* 8. Termination */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.termination.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.termination.content}
              </Paragraph>
            </section>

            {/* 9. Disclaimers */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.disclaimers.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.disclaimers.content}
              </Paragraph>
            </section>

            {/* 10. Limitation of Liability */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.limitationOfLiability.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.limitationOfLiability.content}
              </Paragraph>
            </section>

            {/* 11. Modifications to Terms */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.modifications.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.modifications.content}
              </Paragraph>
            </section>

            {/* 12. Governing Law */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.governingLaw.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.governingLaw.content}
              </Paragraph>
            </section>

            {/* 13. Contact Information */}
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
