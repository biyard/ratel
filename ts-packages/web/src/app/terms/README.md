# Terms of Service Page

A comprehensive Terms of Service page for Ratel platform, fully localized in both English and Korean.

## Features

- **Full Internationalization**: Complete translations for English (en) and Korean (ko)
- **Comprehensive Legal Coverage**: 13 sections covering all essential legal aspects
- **Responsive Design**: Mobile-friendly layout with centered content
- **Theme Support**: Works with both light and dark modes
- **Business Information Integration**: Automatically uses company info from footer translations
- **Professional Layout**: Clean, readable format with clear section hierarchy

## Sections Included

1. **Acceptance of Terms** - User agreement to terms
2. **Description of Service** - Overview of Ratel's platform and features
3. **User Accounts** - Account responsibility and security
4. **User Conduct** - Acceptable use policies
5. **Intellectual Property Rights** - Content ownership and protection
6. **User-Generated Content** - Content licensing and ownership
7. **Privacy Policy** - Reference to privacy practices
8. **Termination** - Service termination conditions
9. **Disclaimers** - Service warranties and limitations
10. **Limitation of Liability** - Legal liability limits
11. **Modifications to Terms** - Policy update procedures
12. **Governing Law** - Legal jurisdiction (Republic of Korea)
13. **Contact Information** - Company contact details

## File Structure

```
src/app/terms/
├── index.tsx          # Main Terms component
├── page.tsx           # Next.js route page
├── i18n.tsx           # Translation strings and hooks
└── README.md          # This file
```

## Usage

The page is accessible at `/terms` route.

### Example
```tsx
import { Terms } from '@/app/terms';

// Use in your app
<Terms />
```

## Customization

### Update Effective Date

Edit the `effectiveDate` in `i18n.tsx`:

```tsx
effectiveDate: 'January 1, 2024', // English
effectiveDate: '2024년 1월 1일',   // Korean
```

### Modify Content

All content is defined in `i18n.tsx` under the `sections` object. Each section has:
- `title` - Section heading
- `content` - Section body text

### Update Contact Information

Contact information is automatically pulled from the Footer translations (`footer.values`):
- Company name
- Email address
- Physical address

To update, modify `/components/footer/i18n.tsx`.

## Styling

The page uses:
- Tailwind CSS for styling
- Custom UI components (Col, Heading, Paragraph)
- Responsive max-width container (max-w-4xl)
- Theme-aware colors
- Consistent spacing and typography

## Internationalization

Translations are managed through react-i18next:

```tsx
import { useTermsI18n } from './i18n';

function MyComponent() {
  const t = useTermsI18n();

  return <div>{t.title}</div>;
}
```

## Integration with Footer

The Terms page links are integrated with the Footer component:
- Footer displays "Terms of Use" link
- Link points to `/terms` route
- Contact information is shared between Footer and Terms page

## Legal Compliance

This terms page includes all standard clauses required for:
- ✅ User agreement and acceptance
- ✅ Service description
- ✅ User conduct and content policies
- ✅ Intellectual property protection
- ✅ Privacy policy reference
- ✅ Liability limitations
- ✅ Governing law specification
- ✅ Contact information

## SEO Considerations

The page includes:
- Clear, hierarchical heading structure (h1, h3)
- Semantic HTML sections
- Proper page title
- Last updated date
- Company information

## Accessibility

- Semantic HTML structure
- Clear heading hierarchy
- Readable font sizes
- Sufficient color contrast
- Keyboard navigation support

## Related Pages

- [Privacy Policy](#) - Data protection and privacy practices
- [Refund Policy](#) - Payment and refund terms
- [Footer Component](../../components/footer/README.md) - Footer with business info
