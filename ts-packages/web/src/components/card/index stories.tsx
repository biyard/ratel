import type { Meta, StoryObj } from '@storybook/react';
import Card from './index';

const meta: Meta<typeof Card> = {
  title: 'UI/Card',
  component: Card,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'secondary'],
    },
    children: {
      control: { type: 'text' },
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    variant: 'default',
    children: (
      <div>
        <h3>Card Title</h3>
        <p>This is the default card variant.</p>
      </div>
    ),
  },
};

export const Secondary: Story = {
  args: {
    variant: 'secondary',
    children: (
      <div>
        <h3>Secondary Card</h3>
        <p>This is the secondary card variant.</p>
      </div>
    ),
  },
};

export const WithCustomClass: Story = {
  args: {
    variant: 'default',
    className: 'max-w-sm shadow-lg',
    children: (
      <div>
        <h3>Custom Styled Card</h3>
        <p>Card with additional className.</p>
      </div>
    ),
  },
};
