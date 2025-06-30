import type { Meta, StoryObj } from '@storybook/nextjs';
import { Button } from '../components/ui/button';

import ArrowRight from '@/assets/icons/right.svg';
const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'outline', 'rounded_primary', 'rounded_secondary'],
    },
    size: {
      control: { type: 'select' },
      options: ['default', 'sm', 'lg'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const RoundedPrimary: Story = {
  args: {
    variant: 'rounded_primary',
    size: 'default',
    children: 'BUY $RATEL',
  },
};

export const RoundedSecondary: Story = {
  args: {
    variant: 'rounded_secondary',
    size: 'default',
    children: 'BUY $RATEL',
  },
};

export const Default: Story = {
  args: {
    variant: 'default',
    size: 'default',
    children: (
      <div className="flex justify-center items-center gap-2.5 ">
        View All
        <ArrowRight className="[&>path]:stroke-black" />
      </div>
    ),
  },
};

export const Outline: Story = {
  args: {
    variant: 'outline',
    size: 'default',
    children: (
      <>
        View All
        <ArrowRight className="[&>path]:stroke-white" />
      </>
    ),
  },
};
