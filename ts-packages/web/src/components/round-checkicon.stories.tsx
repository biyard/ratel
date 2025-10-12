import type { Meta, StoryObj } from '@storybook/react';
import RoundCheckIcon from './round-checkicon';

const meta = {
  title: 'Components/RoundCheckIcon',
  component: RoundCheckIcon,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof RoundCheckIcon>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default
export const Default: Story = {};

// Multiple icons
export const MultipleIcons: Story = {
  render: () => (
    <div className="flex gap-4">
      <RoundCheckIcon />
      <RoundCheckIcon />
      <RoundCheckIcon />
    </div>
  ),
};

// In a success message
export const InSuccessMessage: Story = {
  render: () => (
    <div className="flex items-center gap-3 p-4 bg-green-50 border border-green-200 rounded-lg max-w-md">
      <RoundCheckIcon />
      <div>
        <div className="font-semibold text-green-800">Success!</div>
        <div className="text-sm text-green-700">
          Your changes have been saved.
        </div>
      </div>
    </div>
  ),
};

// In a checklist
export const InChecklist: Story = {
  render: () => (
    <div className="max-w-md border rounded-lg p-4">
      <h3 className="font-semibold mb-4">Setup Checklist</h3>
      <div className="space-y-3">
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Create an account</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Verify your email</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Complete your profile</span>
        </div>
        <div className="flex items-center gap-3">
          <div className="w-[30px] h-[30px] rounded-full border-2 border-gray-300" />
          <span className="text-sm text-gray-400">
            Invite team members (pending)
          </span>
        </div>
      </div>
    </div>
  ),
};

// With steps/progress
export const WithSteps: Story = {
  render: () => (
    <div className="max-w-md">
      <h3 className="font-semibold mb-4">Order Status</h3>
      <div className="space-y-4">
        <div className="flex items-start gap-3">
          <RoundCheckIcon />
          <div>
            <div className="font-medium text-sm">Order Placed</div>
            <div className="text-xs text-gray-600">March 15, 2024</div>
          </div>
        </div>
        <div className="flex items-start gap-3">
          <RoundCheckIcon />
          <div>
            <div className="font-medium text-sm">Payment Confirmed</div>
            <div className="text-xs text-gray-600">March 15, 2024</div>
          </div>
        </div>
        <div className="flex items-start gap-3">
          <RoundCheckIcon />
          <div>
            <div className="font-medium text-sm">Shipped</div>
            <div className="text-xs text-gray-600">March 16, 2024</div>
          </div>
        </div>
        <div className="flex items-start gap-3">
          <div className="w-[30px] h-[30px] rounded-full border-2 border-gray-300 flex items-center justify-center">
            <div className="w-2 h-2 rounded-full bg-gray-300" />
          </div>
          <div>
            <div className="font-medium text-sm text-gray-400">Delivered</div>
            <div className="text-xs text-gray-400">Pending</div>
          </div>
        </div>
      </div>
    </div>
  ),
};

// In a feature list
export const InFeatureList: Story = {
  render: () => (
    <div className="max-w-md border rounded-lg p-6">
      <h3 className="font-semibold text-lg mb-2">Pro Plan</h3>
      <p className="text-sm text-gray-600 mb-4">
        Everything you need to get started
      </p>
      <div className="space-y-3">
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Unlimited projects</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Advanced analytics</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Priority support</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">Custom branding</span>
        </div>
        <div className="flex items-center gap-3">
          <RoundCheckIcon />
          <span className="text-sm">API access</span>
        </div>
      </div>
      <button className="w-full mt-6 bg-blue-500 text-white py-2 rounded hover:bg-blue-600">
        Get Started
      </button>
    </div>
  ),
};

// In a card grid
export const InCardGrid: Story = {
  render: () => (
    <div className="grid grid-cols-3 gap-4">
      <div className="text-center p-4 border rounded-lg">
        <div className="flex justify-center mb-2">
          <RoundCheckIcon />
        </div>
        <div className="font-medium text-sm">Fast</div>
        <div className="text-xs text-gray-600 mt-1">
          Lightning quick performance
        </div>
      </div>
      <div className="text-center p-4 border rounded-lg">
        <div className="flex justify-center mb-2">
          <RoundCheckIcon />
        </div>
        <div className="font-medium text-sm">Secure</div>
        <div className="text-xs text-gray-600 mt-1">
          Enterprise-grade security
        </div>
      </div>
      <div className="text-center p-4 border rounded-lg">
        <div className="flex justify-center mb-2">
          <RoundCheckIcon />
        </div>
        <div className="font-medium text-sm">Reliable</div>
        <div className="text-xs text-gray-600 mt-1">99.9% uptime guarantee</div>
      </div>
    </div>
  ),
};

// In a confirmation dialog
export const InConfirmationDialog: Story = {
  render: () => (
    <div className="max-w-sm border rounded-lg p-6 shadow-lg">
      <div className="flex justify-center mb-4">
        <RoundCheckIcon />
      </div>
      <h3 className="font-semibold text-center text-lg mb-2">
        Payment Successful
      </h3>
      <p className="text-sm text-gray-600 text-center mb-4">
        Your payment of $99.00 has been processed successfully.
      </p>
      <button className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600">
        Continue
      </button>
    </div>
  ),
};
