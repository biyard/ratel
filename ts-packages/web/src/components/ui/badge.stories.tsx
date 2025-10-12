import type { Meta, StoryObj } from '@storybook/react';
import { Badge } from './badge';

const meta = {
  title: 'UI/Badge',
  component: Badge,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'secondary', 'destructive', 'outline'],
      description: 'The visual style variant of the badge',
    },
    size: {
      control: 'select',
      options: ['sm', 'default', 'lg'],
      description: 'The size of the badge',
    },
    asChild: {
      control: 'boolean',
      description: 'Render as a child element using Radix UI Slot',
    },
  },
} satisfies Meta<typeof Badge>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default badge
export const Default: Story = {
  args: {
    children: 'Badge',
    variant: 'default',
    size: 'default',
  },
};

// Secondary variant
export const Secondary: Story = {
  args: {
    children: 'Secondary',
    variant: 'secondary',
    size: 'default',
  },
};

// Destructive variant
export const Destructive: Story = {
  args: {
    children: 'Destructive',
    variant: 'destructive',
    size: 'default',
  },
};

// Outline variant
export const Outline: Story = {
  args: {
    children: 'Outline',
    variant: 'outline',
    size: 'default',
  },
};

// Small size
export const Small: Story = {
  args: {
    children: 'Small',
    variant: 'default',
    size: 'sm',
  },
};

// Large size
export const Large: Story = {
  args: {
    children: 'Large',
    variant: 'default',
    size: 'lg',
  },
};

// With icon
export const WithIcon: Story = {
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
            d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
          />
        </svg>
        Verified
      </>
    ),
    variant: 'default',
  },
};

// As link
export const AsLink: Story = {
  render: () => (
    <a href="#" className="inline-block">
      <Badge variant="outline">Clickable Badge</Badge>
    </a>
  ),
};

// All variants showcase
export const AllVariants: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Badge variant="default">Default</Badge>
      <Badge variant="secondary">Secondary</Badge>
      <Badge variant="destructive">Destructive</Badge>
      <Badge variant="outline">Outline</Badge>
    </div>
  ),
};

// All sizes showcase
export const AllSizes: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4 items-center">
      <Badge size="sm" variant="default">
        Small
      </Badge>
      <Badge size="default" variant="default">
        Default
      </Badge>
      <Badge size="lg" variant="default">
        Large
      </Badge>
    </div>
  ),
};

// Use cases
export const UseCases: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div className="flex gap-2 items-center">
        <span className="text-sm">Status:</span>
        <Badge variant="default">Active</Badge>
      </div>
      <div className="flex gap-2 items-center">
        <span className="text-sm">Priority:</span>
        <Badge variant="destructive">High</Badge>
      </div>
      <div className="flex gap-2 items-center">
        <span className="text-sm">Category:</span>
        <Badge variant="secondary">Development</Badge>
      </div>
      <div className="flex gap-2 items-center">
        <span className="text-sm">Count:</span>
        <Badge variant="outline">12</Badge>
      </div>
    </div>
  ),
};
