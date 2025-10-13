import type { Meta, StoryObj } from '@storybook/react';
import UserTier from './UserTier';

const meta = {
  title: 'App/Social/UserTier',
  component: UserTier,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  decorators: [
    (Story) => (
      <div className="w-80">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof UserTier>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default
export const Default: Story = {};

// In a card
export const InCard: Story = {
  render: () => (
    <div className="border rounded-lg p-4 bg-white">
      <h3 className="font-semibold mb-2">User Profile</h3>
      <div className="text-sm text-gray-600 mb-2">
        John Doe
        <br />
        john@example.com
      </div>
      <UserTier />
    </div>
  ),
};

// In a sidebar
export const InSidebar: Story = {
  render: () => (
    <div className="w-64 border rounded-lg p-4 bg-white">
      <div className="flex items-center gap-3 mb-4">
        <div className="w-12 h-12 bg-blue-500 rounded-full" />
        <div>
          <div className="font-semibold">John Doe</div>
          <div className="text-sm text-gray-600">@johndoe</div>
        </div>
      </div>
      <UserTier />
      <div className="mt-4 pt-4 border-t">
        <div className="flex justify-between text-sm">
          <span className="text-gray-600">Posts</span>
          <span className="font-semibold">124</span>
        </div>
        <div className="flex justify-between text-sm mt-2">
          <span className="text-gray-600">Followers</span>
          <span className="font-semibold">1.2K</span>
        </div>
      </div>
    </div>
  ),
};

// Different tier levels (mockup)
export const TierLevels: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div>
        <p className="text-sm text-gray-600 mb-2">Diamond Tier (100%)</p>
        <UserTier />
      </div>
      <div>
        <p className="text-sm text-gray-600 mb-2">Platinum Tier (75%) - Mockup</p>
        <div className="mt-4">
          <div className="flex justify-between items-center">
            <span className="text-sm">Tier</span>
            <div className="flex items-center gap-1">
              <span className="text-sm">Platinum</span>
              <div className="w-4 h-4 rounded-full bg-gray-400 flex items-center justify-center">
                <svg
                  className="w-2.5 h-2.5 text-white"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
              </div>
            </div>
          </div>
          <div className="mt-1 h-1 w-full bg-gray-700 rounded-full">
            <div className="h-full w-3/4 bg-gray-400 rounded-full"></div>
          </div>
        </div>
      </div>
      <div>
        <p className="text-sm text-gray-600 mb-2">Gold Tier (50%) - Mockup</p>
        <div className="mt-4">
          <div className="flex justify-between items-center">
            <span className="text-sm">Tier</span>
            <div className="flex items-center gap-1">
              <span className="text-sm">Gold</span>
              <div className="w-4 h-4 rounded-full bg-yellow-600 flex items-center justify-center">
                <svg
                  className="w-2.5 h-2.5 text-white"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
              </div>
            </div>
          </div>
          <div className="mt-1 h-1 w-full bg-gray-700 rounded-full">
            <div className="h-full w-1/2 bg-yellow-600 rounded-full"></div>
          </div>
        </div>
      </div>
      <div>
        <p className="text-sm text-gray-600 mb-2">Silver Tier (25%) - Mockup</p>
        <div className="mt-4">
          <div className="flex justify-between items-center">
            <span className="text-sm">Tier</span>
            <div className="flex items-center gap-1">
              <span className="text-sm">Silver</span>
              <div className="w-4 h-4 rounded-full bg-gray-500 flex items-center justify-center">
                <svg
                  className="w-2.5 h-2.5 text-white"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
              </div>
            </div>
          </div>
          <div className="mt-1 h-1 w-full bg-gray-700 rounded-full">
            <div className="h-full w-1/4 bg-gray-500 rounded-full"></div>
          </div>
        </div>
      </div>
    </div>
  ),
};
