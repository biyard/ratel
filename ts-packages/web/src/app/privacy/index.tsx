'use client';

import { usePrivacyI18n } from './i18n';
import { Col } from '@/components/ui/col';
import Heading from '@/components/ui/heading';
import { Paragraph } from '@/components/ui/paragraph';
import { useTranslation } from 'react-i18next';

export function Privacy() {
  const t = usePrivacyI18n();
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

            {/* 2. Information We Collect */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.informationCollection.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.informationCollection.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.informationCollection.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 3. How We Use Your Information */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.howWeUseInfo.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.howWeUseInfo.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.howWeUseInfo.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 4. Information Sharing and Disclosure */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.informationSharing.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.informationSharing.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.informationSharing.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 5. Data Security */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.dataSecurity.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.dataSecurity.content}
              </Paragraph>
            </section>

            {/* 6. Data Retention */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.dataRetention.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.dataRetention.content}
              </Paragraph>
            </section>

            {/* 7. Your Rights */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.yourRights.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground mb-3">
                {t.sections.yourRights.content}
              </Paragraph>
              <ul className="list-disc list-inside pl-4 space-y-2">
                {t.sections.yourRights.items.map((item, index) => (
                  <li key={index} className="text-foreground">
                    {item}
                  </li>
                ))}
              </ul>
            </section>

            {/* 8. Cookies and Tracking */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.cookies.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.cookies.content}
              </Paragraph>
            </section>

            {/* 9. Third-Party Links */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.thirdPartyLinks.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.thirdPartyLinks.content}
              </Paragraph>
            </section>

            {/* 10. Children's Privacy */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.childrenPrivacy.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.childrenPrivacy.content}
              </Paragraph>
            </section>

            {/* 11. International Data Transfers */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.internationalTransfers.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.internationalTransfers.content}
              </Paragraph>
            </section>

            {/* 12. Changes to This Policy */}
            <section>
              <Heading variant="heading3" className="mb-3">
                {t.sections.changes.title}
              </Heading>
              <Paragraph className="leading-relaxed text-foreground">
                {t.sections.changes.content}
              </Paragraph>
            </section>

            {/* 13. Contact Us */}
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
