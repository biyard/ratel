import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import FollowButton from './follow-button';

const meta = {
  title: 'App/Teams/FollowButton',
  component: FollowButton,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    onClick: {
      action: 'clicked',
      description: 'Callback when button is clicked',
    },
  },
} satisfies Meta<typeof FollowButton>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default
export const Default: Story = {
  args: {
    onClick: () => {},
  },
};

// Interactive toggle
export const InteractiveToggle: Story = {
  render: () => {
    const [isFollowing, setIsFollowing] = useState(false);
    return (
      <div className="flex flex-col gap-4 items-center">
        {isFollowing ? (
          <div className="px-3 py-1.5 border border-gray-300 rounded-full text-xs font-bold text-gray-700 bg-gray-100">
            Following
          </div>
        ) : (
          <FollowButton onClick={() => setIsFollowing(true)} />
        )}
        <div className="text-sm text-gray-600">
          Status: {isFollowing ? 'Following' : 'Not Following'}
        </div>
        {isFollowing && (
          <button
            onClick={() => setIsFollowing(false)}
            className="text-xs text-blue-500 hover:underline"
          >
            Reset
          </button>
        )}
      </div>
    );
  },
};

// In a user card
export const InUserCard: Story = {
  render: () => (
    <div className="border rounded-lg p-4 max-w-sm">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 bg-blue-500 rounded-full" />
          <div>
            <div className="font-semibold">Jane Smith</div>
            <div className="text-sm text-gray-600">@janesmith</div>
          </div>
        </div>
        <FollowButton onClick={() => {}} />
      </div>
      <p className="text-sm text-gray-700 mt-3">
        Software engineer and open source enthusiast
      </p>
    </div>
  ),
};

// In a user list
export const InUserList: Story = {
  render: () => (
    <div className="max-w-md border rounded-lg divide-y">
      {['Alice Johnson', 'Bob Williams', 'Charlie Brown'].map((name) => (
        <div key={name} className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-purple-500 rounded-full" />
            <div>
              <div className="font-medium text-sm">{name}</div>
              <div className="text-xs text-gray-600">
                @{name.toLowerCase().replace(' ', '')}
              </div>
            </div>
          </div>
          <FollowButton onClick={() => {}} />
        </div>
      ))}
    </div>
  ),
};

// With follow count
export const WithFollowCount: Story = {
  render: () => {
    const [followers, setFollowers] = useState(1234);
    const [isFollowing, setIsFollowing] = useState(false);

    return (
      <div className="border rounded-lg p-4 max-w-sm">
        <div className="flex items-center gap-3 mb-3">
          <div className="w-16 h-16 bg-green-500 rounded-full" />
          <div>
            <div className="font-semibold text-lg">Alex Taylor</div>
            <div className="text-sm text-gray-600">@alextaylor</div>
          </div>
        </div>
        <div className="flex items-center gap-4 mb-3">
          <div className="text-sm">
            <span className="font-semibold">{followers.toLocaleString()}</span>
            <span className="text-gray-600"> followers</span>
          </div>
          <div className="text-sm">
            <span className="font-semibold">532</span>
            <span className="text-gray-600"> following</span>
          </div>
        </div>
        {isFollowing ? (
          <div className="px-3 py-1.5 w-fit border border-gray-300 rounded-full text-xs font-bold text-gray-700 bg-gray-100">
            Following
          </div>
        ) : (
          <FollowButton
            onClick={() => {
              setIsFollowing(true);
              setFollowers(followers + 1);
            }}
          />
        )}
      </div>
    );
  },
};

// Multiple buttons
export const MultipleButtons: Story = {
  render: () => (
    <div className="flex flex-wrap gap-3">
      <FollowButton onClick={() => {}} />
      <FollowButton onClick={() => {}} />
      <FollowButton onClick={() => {}} />
      <FollowButton onClick={() => {}} />
    </div>
  ),
};

// In team profile header
export const InTeamProfile: Story = {
  render: () => (
    <div className="border rounded-lg overflow-hidden max-w-2xl">
      <div className="h-32 bg-gradient-to-r from-blue-500 to-purple-500" />
      <div className="p-6">
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-4">
            <div className="w-24 h-24 bg-white border-4 border-white rounded-full -mt-16 shadow-lg" />
            <div>
              <h2 className="font-bold text-2xl">Engineering Team</h2>
              <p className="text-gray-600">@engineering</p>
            </div>
          </div>
          <FollowButton onClick={() => {}} />
        </div>
        <p className="text-gray-700 mt-4">
          We build amazing products and share our knowledge with the community.
        </p>
        <div className="flex gap-4 mt-3 text-sm">
          <div>
            <span className="font-semibold">42</span>
            <span className="text-gray-600"> members</span>
          </div>
          <div>
            <span className="font-semibold">1.2K</span>
            <span className="text-gray-600"> followers</span>
          </div>
        </div>
      </div>
    </div>
  ),
};
