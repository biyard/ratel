import type { Meta, StoryObj } from '@storybook/react';
import { MembershipCard } from './membership-card';
import { MembershipPlanItem } from './i18n';

const meta = {
  title: 'Features/Membership/MembershipCard',
  component: MembershipCard,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['vertical', 'horizontal'],
      description: 'Card layout variant',
    },
  },
} satisfies Meta<typeof MembershipCard>;

export default meta;
type Story = StoryObj<typeof meta>;

// Sample membership data
const freeMembership: MembershipPlanItem = {
  name: 'Free',
  description: 'Basic membership open to everyone',
  features: [
    'Publish posts',
    'Publish spaces',
    'Network relationship',
    'Participate reward spaces',
  ],
};

const proMembership: MembershipPlanItem = {
  name: 'Pro',
  description: 'Reward Space setup for small communities',
  features: [
    'Includes all Free',
    '40 monthly credits',
    'Up to 2 credits per a reward space',
    'Earn 10% of the total rewards distributed to participants.',
  ],
  price: '$20 / month',
  btn: 'Get Pro',
};

const maxMembership: MembershipPlanItem = {
  name: 'Max',
  description: 'Advanced Reward Spaces for large communities',
  features: [
    'Includes all Free',
    '190 monthly credits',
    'Up to 10 credits per a reward space',
    'Earn 10% of the total rewards distributed to participants.',
    'Get a trusted creator badge',
  ],
  price: '$90 / month',
  btn: 'Get Max',
};

const vipMembership: MembershipPlanItem = {
  name: 'VIP',
  description: 'Reward Spaces for influencers and promotion',
  features: [
    'Includes all Free',
    '1,360 monthly credits',
    'Up to 100 credits per a reward space',
    'Earn 10% of the total rewards distributed to participants.',
    'Get a trusted creator badge',
    'Access raw participant data',
  ],
  price: '$180 / month',
  btn: 'Get VIP',
};

const enterpriseMembership: MembershipPlanItem = {
  name: 'Enterprise',
  description: 'Customized partner plan for enterprises & organizations',
  features: ['Includes all Free', 'Fully customization'],
  price: 'Starting at $1,000 / month',
  btn: 'Contact Us',
};

export const VerticalFree: Story = {
  args: {
    variant: 'vertical',
    membership: freeMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Vertical card layout for the Free membership tier (no price or button).',
      },
    },
  },
};

export const VerticalPro: Story = {
  args: {
    variant: 'vertical',
    membership: proMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Vertical card layout for the Pro membership tier with price and CTA button.',
      },
    },
  },
};

export const VerticalMax: Story = {
  args: {
    variant: 'vertical',
    membership: maxMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Vertical card layout for the Max membership tier.',
      },
    },
  },
};

export const VerticalVIP: Story = {
  args: {
    variant: 'vertical',
    membership: vipMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Vertical card layout for the VIP membership tier with extended features.',
      },
    },
  },
};

export const VerticalEnterprise: Story = {
  args: {
    variant: 'vertical',
    membership: enterpriseMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Vertical card layout for the Enterprise membership tier.',
      },
    },
  },
};

export const HorizontalFree: Story = {
  args: {
    variant: 'horizontal',
    membership: freeMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Horizontal card layout for the Free membership tier.',
      },
    },
  },
};

export const HorizontalPro: Story = {
  args: {
    variant: 'horizontal',
    membership: proMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Horizontal card layout for the Pro membership tier.',
      },
    },
  },
};

export const HorizontalMax: Story = {
  args: {
    variant: 'horizontal',
    membership: maxMembership,
  },
  parameters: {
    docs: {
      description: {
        story: 'Horizontal card layout for the Max membership tier.',
      },
    },
  },
};

export const AllVerticalCards: Story = {
  render: () => (
    <div className="grid grid-cols-4 gap-2.5">
      <MembershipCard variant="vertical" membership={freeMembership} />
      <MembershipCard variant="vertical" membership={proMembership} />
      <MembershipCard variant="vertical" membership={maxMembership} />
      <MembershipCard variant="vertical" membership={vipMembership} />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'All membership tiers displayed in vertical card layout, as they would appear in a pricing page grid.',
      },
    },
  },
};

export const AllHorizontalCards: Story = {
  render: () => (
    <div className="space-y-4">
      <MembershipCard variant="horizontal" membership={freeMembership} />
      <MembershipCard variant="horizontal" membership={proMembership} />
      <MembershipCard variant="horizontal" membership={maxMembership} />
      <MembershipCard variant="horizontal" membership={vipMembership} />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'All membership tiers displayed in horizontal card layout, stacked vertically.',
      },
    },
  },
};

export const VariantComparison: Story = {
  render: () => (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold mb-4">Vertical Variant</h3>
        <div className="grid grid-cols-4 gap-2.5">
          <MembershipCard variant="vertical" membership={freeMembership} />
          <MembershipCard variant="vertical" membership={proMembership} />
          <MembershipCard variant="vertical" membership={maxMembership} />
        </div>
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-4">Horizontal Variant</h3>
        <div className="space-y-4">
          <MembershipCard variant="horizontal" membership={freeMembership} />
          <MembershipCard variant="horizontal" membership={proMembership} />
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Side-by-side comparison of vertical and horizontal card variants.',
      },
    },
  },
};

export const CustomMembership: Story = {
  args: {
    variant: 'vertical',
    membership: {
      name: 'Custom Plan',
      description: 'A custom membership plan with unique features',
      features: [
        'Custom feature 1',
        'Custom feature 2',
        'Custom feature 3',
        'Custom feature 4',
        'Custom feature 5',
      ],
      price: '$99 / month',
      btn: 'Subscribe Now',
    },
  },
  parameters: {
    docs: {
      description: {
        story: 'Example of a custom membership card with custom data.',
      },
    },
  },
};

export const MinimalMembership: Story = {
  args: {
    variant: 'vertical',
    membership: {
      name: 'Starter',
      description: 'Just getting started',
      features: ['Basic access', 'Community support'],
    },
  },
  parameters: {
    docs: {
      description: {
        story: 'Minimal membership card with just name, description, and features (no price or button).',
      },
    },
  },
};

export const ExtendedFeatures: Story = {
  args: {
    variant: 'vertical',
    membership: {
      name: 'Ultimate',
      description: 'Everything you need and more',
      features: [
        'Unlimited posts',
        'Unlimited spaces',
        'Premium support 24/7',
        'Advanced analytics',
        'Custom branding',
        'API access',
        'Priority feature requests',
        'Dedicated account manager',
        'White-label options',
        'Enterprise SLA',
      ],
      price: '$499 / month',
      btn: 'Get Ultimate',
    },
  },
  parameters: {
    docs: {
      description: {
        story: 'Membership card with an extended list of features to demonstrate scrolling behavior.',
      },
    },
  },
};

export const ResponsiveGrid: Story = {
  render: () => (
    <div className="grid grid-cols-4 gap-2.5">
      <MembershipCard variant="vertical" membership={freeMembership} />
      <MembershipCard variant="vertical" membership={proMembership} />
      <MembershipCard variant="vertical" membership={maxMembership} />
      <MembershipCard variant="vertical" membership={vipMembership} />
      <MembershipCard variant="vertical" membership={enterpriseMembership} />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Responsive grid showing how cards adapt to different screen sizes (col-span-4 on mobile, md:col-span-2 on tablet, lg:col-span-1 on desktop).',
      },
    },
  },
};

export const MixedContent: Story = {
  render: () => (
    <div className="grid grid-cols-4 gap-2.5">
      <MembershipCard
        variant="vertical"
        membership={{
          name: 'Short',
          description: 'Brief description',
          features: ['Feature 1', 'Feature 2'],
          price: '$10',
          btn: 'Buy',
        }}
      />
      <MembershipCard
        variant="vertical"
        membership={{
          name: 'Medium Plan',
          description: 'Medium length description here',
          features: [
            'Feature 1',
            'Feature 2',
            'Feature 3',
            'Feature 4',
            'Feature 5',
          ],
          price: '$50 / month',
          btn: 'Get Started',
        }}
      />
      <MembershipCard
        variant="vertical"
        membership={{
          name: 'Long Plan Name Here',
          description: 'This is a much longer description that explains the benefits of this membership plan in more detail',
          features: [
            'First feature with details',
            'Second feature description',
            'Third feature',
            'Fourth feature with more info',
            'Fifth feature',
            'Sixth feature',
            'Seventh feature',
          ],
          price: '$100 / month + tax',
          btn: 'Subscribe Today',
        }}
      />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Cards with varying content lengths to demonstrate how the layout handles different amounts of text.',
      },
    },
  },
};
