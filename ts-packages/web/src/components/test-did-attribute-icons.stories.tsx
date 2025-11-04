import type { Meta, StoryObj } from '@storybook/react';
import { TestDidAttributeIcons } from './test-did-attribute-icons';

const meta = {
  title: 'Components/Icons/DID Attributes',
  component: TestDidAttributeIcons,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof TestDidAttributeIcons>;

export default meta;
type Story = StoryObj<typeof meta>;

export const AllAttributeIcons: Story = {
  parameters: {
    docs: {
      description: {
        story: 'All DID attribute icons (Age and Gender) in various sizes and colors.',
      },
    },
  },
};

export const IconDetails: Story = {
  parameters: {
    docs: {
      description: {
        story: `
## DID Attribute Icons

These icons are used to represent different user attributes in the DID verification system.

### Age Icon
The Age icon represents age-related verification attributes. It displays an ID card style icon.

**Usage:**
\`\`\`tsx
import { Age } from '@/components/icons';

<Age className="w-6 h-6" />
\`\`\`

### Gender Icon
The Gender icon represents gender-related verification attributes. It displays a combined gender symbol.

**Usage:**
\`\`\`tsx
import { Gender } from '@/components/icons';

<Gender className="w-6 h-6" />
\`\`\`

### Available Sizes
- Default: 24px (w-6 h-6)
- Large: 32px (w-8 h-8)
- Extra Large: 48px (w-12 h-12)

### Customization
Both icons support:
- Custom sizing via className
- Color customization via text-* classes
- All standard SVG attributes
        `,
      },
    },
  },
};
