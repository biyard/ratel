import type { Meta, StoryObj } from '@storybook/react';
import Footer from './index';

const meta = {
  title: 'Components/Footer',
  component: Footer,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof Footer>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

export const CustomInfo: Story = {
  args: {
    info: {
      companyName: 'Ratel Corporation',
      ceo: 'John Doe',
      businessRegistration: '123-45-67890',
      address: '123 Tech Street, Seoul, South Korea',
      phone: '+82-2-1234-5678',
      email: 'contact@ratel.com',
      termsUrl: '/terms',
      privacyUrl: '/privacy',
      refundUrl: '/refund',
    },
  },
};
