import type { Meta, StoryObj } from '@storybook/react';
import { Button } from './button';

const meta = {
  title: 'UI/Button',
  component: Button,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'primary', 'rounded_primary', 'rounded_secondary', 'outline', 'text'],
      description: 'The visual style variant of the button',
    },
    size: {
      control: 'select',
      options: ['sm', 'default', 'lg'],
      description: 'The size of the button',
    },
    platform: {
      control: 'select',
      options: ['web', 'mobile'],
      description: 'The platform target for the button',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled',
    },
    asChild: {
      control: 'boolean',
      description: 'Render as a child element using Radix UI Slot',
    },
  },
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default button
export const Default: Story = {
  args: {
    children: 'Button',
    variant: 'default',
    size: 'default',
  },
};

// Primary variant
export const Primary: Story = {
  args: {
    children: 'Primary Button',
    variant: 'primary',
    size: 'default',
  },
};

// Rounded Primary variant
export const RoundedPrimary: Story = {
  args: {
    children: 'Rounded Primary',
    variant: 'rounded_primary',
    size: 'default',
  },
};

// Rounded Secondary variant
export const RoundedSecondary: Story = {
  args: {
    children: 'Secondary Button',
    variant: 'rounded_secondary',
    size: 'default',
  },
};

// Outline variant
export const Outline: Story = {
  args: {
    children: 'Outline Button',
    variant: 'outline',
    size: 'default',
  },
  parameters: {
    backgrounds: { default: 'dark' },
  },
};

// Text variant
export const Text: Story = {
  args: {
    children: 'Text Button',
    variant: 'text',
    size: 'default',
  },
  parameters: {
    backgrounds: { default: 'dark' },
  },
};

// Small size
export const Small: Story = {
  args: {
    children: 'Small Button',
    variant: 'default',
    size: 'sm',
  },
};

// Large size
export const Large: Story = {
  args: {
    children: 'Large Button',
    variant: 'default',
    size: 'lg',
  },
};

// Disabled state
export const Disabled: Story = {
  args: {
    children: 'Disabled Button',
    variant: 'default',
    disabled: true,
  },
};

// With icon (left)
export const WithIconLeft: Story = {
  args: {
    children: (
      <>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M12 4.5v15m7.5-7.5h-15"
          />
        </svg>
        Add Item
      </>
    ),
    variant: 'rounded_primary',
  },
};

// With icon (right)
export const WithIconRight: Story = {
  args: {
    children: (
      <>
        Continue
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3"
          />
        </svg>
      </>
    ),
    variant: 'rounded_primary',
  },
};

// Icon only
export const IconOnly: Story = {
  args: {
    children: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth={1.5}
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          d="M6 18 18 6M6 6l12 12"
        />
      </svg>
    ),
    variant: 'default',
    className: 'px-2.5',
  },
};

// All variants showcase
export const AllVariants: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div className="flex gap-4 items-center flex-wrap">
        <Button variant="default">Default</Button>
        <Button variant="primary">Primary</Button>
        <Button variant="rounded_primary">Rounded Primary</Button>
        <Button variant="rounded_secondary">Rounded Secondary</Button>
      </div>
      <div className="flex gap-4 items-center bg-gray-800 p-4 rounded-lg">
        <Button variant="outline">Outline</Button>
        <Button variant="text">Text</Button>
      </div>
    </div>
  ),
};

// All sizes showcase
export const AllSizes: Story = {
  render: () => (
    <div className="flex gap-4 items-center">
      <Button size="sm" variant="rounded_primary">
        Small
      </Button>
      <Button size="default" variant="rounded_primary">
        Default
      </Button>
      <Button size="lg" variant="rounded_primary">
        Large
      </Button>
    </div>
  ),
};

// All states showcase
export const AllStates: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div className="flex gap-4 items-center">
        <Button variant="rounded_primary">Normal</Button>
        <Button variant="rounded_primary" disabled>
          Disabled
        </Button>
      </div>
      <div className="flex gap-4 items-center">
        <Button variant="primary">Normal</Button>
        <Button variant="primary" disabled>
          Disabled
        </Button>
      </div>
      <div className="flex gap-4 items-center">
        <Button variant="default">Normal</Button>
        <Button variant="default" disabled>
          Disabled
        </Button>
      </div>
    </div>
  ),
};

// Platform showcase
export const Platforms: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div className="flex gap-4 items-center">
        <Button variant="rounded_primary" platform="web">
          Web Platform
        </Button>
        <Button variant="rounded_primary" platform="mobile">
          Mobile Platform
        </Button>
      </div>
    </div>
  ),
};
