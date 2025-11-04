# Footer Component

A comprehensive footer component for PG (Payment Gateway) approval requirements, displaying business information, legal links, and copyright notice.

## Features

- **Business Information Display**: Company name, CEO, business registration number, address, phone, and email
- **Legal Links**: Terms of Use, Privacy Policy, and Refund Policy
- **Internationalization**: Supports Korean (ko) and English (en)
- **Responsive Design**: Mobile-friendly layout that adapts to different screen sizes
- **Customizable**: Can be customized via props or environment variables

## Usage

### Basic Usage

```tsx
import Footer from '@/components/footer';

// Use with default config values
<Footer />
```

### Custom Information

```tsx
import Footer from '@/components/footer';

<Footer
  info={{
    companyName: 'Your Company Inc.',
    ceo: 'John Doe',
    businessRegistration: '123-45-67890',
    address: '123 Tech Street, Seoul, South Korea',
    phone: '+82-2-1234-5678',
    email: 'contact@yourcompany.com',
    termsUrl: '/terms',
    privacyUrl: '/privacy',
    refundUrl: '/refund',
  }}
/>
```

## Environment Variables

Configure business information via environment variables (in `.env` file):

```bash
# Business Information (for PG approval footer)
VITE_COMPANY_NAME="Your Company Inc."
VITE_COMPANY_CEO="CEO Name"
VITE_BUSINESS_REGISTRATION="123-45-67890"
VITE_BUSINESS_ADDRESS="123 Tech Street, Seoul, South Korea"
VITE_BUSINESS_PHONE="+82-2-1234-5678"
VITE_BUSINESS_EMAIL="contact@yourcompany.com"
```

## PG Approval Requirements

For Korean PG (Payment Gateway) approval, the footer must display:

1. ✅ Business Registration Number (사업자등록번호)
2. ✅ Business Address (사업장 주소)
3. ✅ CEO Name (대표자명)
4. ✅ Phone Number (전화번호)
5. ✅ Usage Terms Link (이용약관)
6. ✅ Privacy Policy Link (개인정보처리방침) - **Must be bold/prominent**
7. ✅ Refund Policy Link (환불규정)

All these requirements are met by this component.

## Props

### FooterProps

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| info | object | No | Override default business information |
| className | string | No | Additional CSS classes |
| ...props | HTMLAttributes | No | Any other HTML div attributes |

### info Object

| Property | Type | Description |
|----------|------|-------------|
| companyName | string | Company name |
| ceo | string | CEO name |
| businessRegistration | string | Business registration number |
| address | string | Business address |
| phone | string | Phone number |
| email | string | Email address |
| termsUrl | string | URL to Terms of Use page |
| privacyUrl | string | URL to Privacy Policy page |
| refundUrl | string | URL to Refund Policy page |

## Internationalization

The component uses i18next for translations. Translation keys are defined in `i18n.tsx`:

- English (`en`)
- Korean (`ko`)

Translations are automatically registered in the i18n config.

## Styling

The component uses:
- Tailwind CSS for styling
- Responsive grid layout (1 column on mobile, 2 columns on desktop)
- Theme-aware colors (supports light/dark mode)

## Files

- `index.tsx` - Main component
- `i18n.tsx` - Translation strings
- `index.stories.tsx` - Storybook stories
- `README.md` - This file

## Storybook

View the component in Storybook:

```bash
npm run storybook
```

Navigate to **Components > Footer** to see examples.
