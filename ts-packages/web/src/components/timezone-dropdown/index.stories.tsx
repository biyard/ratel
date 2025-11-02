import TimezoneDropdown from './index';
import type { Meta, StoryObj } from '@storybook/react';

const meta = {
  title: 'Components/TimezoneDropdown',
  component: TimezoneDropdown,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof TimezoneDropdown>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    value: 'America/Los_Angeles',
    onChange: (timezone: string) => console.log('Selected timezone:', timezone),
    canEdit: true,
  },
};

export const Disabled: Story = {
  args: {
    value: 'Europe/London',
    onChange: (timezone: string) => console.log('Selected timezone:', timezone),
    canEdit: false,
  },
};

export const AsianTimezone: Story = {
  args: {
    value: 'Asia/Seoul',
    onChange: (timezone: string) => console.log('Selected timezone:', timezone),
    canEdit: true,
  },
};
