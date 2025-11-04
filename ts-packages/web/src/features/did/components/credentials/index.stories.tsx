import type { Meta, StoryObj } from '@storybook/react';
import { Credentials } from './index';

const meta = {
  title: 'Features/DID/Credentials',
  component: Credentials,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof Credentials>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  parameters: {
    docs: {
      description: {
        story: 'Default view of the DID credentials component showing verifiable credential card and attribute verification cards.',
      },
    },
  },
};

export const WithDescription: Story = {
  parameters: {
    docs: {
      description: {
        story: `
The Credentials component displays:
- **Verifiable Credential Card**: Shows the user's DID with a blue radial gradient background and the verified icon
- **My DID Section**: Header for the attribute cards section
- **Age Verification Card**: Displays age range (20-29) with verified status and the verified icon
- **Gender Verification Card**: Shows registration required status with a verify button

The component uses the newly added DID and Verified icons from the icon library.
        `,
      },
    },
  },
};
