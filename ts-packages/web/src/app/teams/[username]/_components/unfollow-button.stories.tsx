import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import UnfollowButton from './unfollow-button';

const meta = {
  title: 'App/Teams/UnfollowButton',
  component: UnfollowButton,
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
} satisfies Meta<typeof UnfollowButton>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default (not hovered)
export const Default: Story = {
  args: {
    onClick: () => {},
  },
};

// Interactive with hover effect
export const InteractiveHover: Story = {
  render: () => (
    <div className="flex flex-col gap-4 items-center">
      <UnfollowButton onClick={() => {}} />
      <p className="text-sm text-gray-600 text-center max-w-xs">
        Hover over the button to see it change from "Following" to "Unfollow"
      </p>
    </div>
  ),
};

// Interactive toggle between follow/unfollow
export const InteractiveToggle: Story = {
  render: () => {
    const [isFollowing, setIsFollowing] = useState(true);
    return (
      <div className="flex flex-col gap-4 items-center">
        {isFollowing ? (
          <UnfollowButton onClick={() => setIsFollowing(false)} />
        ) : (
          <div
            className="cursor-pointer px-3 py-1.5 rounded-full bg-blue-500 text-white text-xs font-bold hover:bg-blue-600"
            onClick={() => setIsFollowing(true)}
          >
            Follow
          </div>
        )}
        <div className="text-sm text-gray-600">
          Status: {isFollowing ? 'Following' : 'Not Following'}
        </div>
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
        <UnfollowButton onClick={() => {}} />
      </div>
      <p className="text-sm text-gray-700 mt-3">
        Software engineer and open source enthusiast
      </p>
    </div>
  ),
};

// In a following list
export const InFollowingList: Story = {
  render: () => (
    <div className="max-w-md border rounded-lg divide-y">
      <div className="p-4 border-b">
        <h3 className="font-semibold">Following</h3>
      </div>
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
          <UnfollowButton onClick={() => {}} />
        </div>
      ))}
    </div>
  ),
};

// With confirmation dialog
export const WithConfirmation: Story = {
  render: () => {
    const [showConfirm, setShowConfirm] = useState(false);
    const [isFollowing, setIsFollowing] = useState(true);

    return (
      <div className="flex flex-col gap-4 items-center">
        {isFollowing && (
          <UnfollowButton onClick={() => setShowConfirm(true)} />
        )}
        {!isFollowing && (
          <div className="text-sm text-gray-600">
            You unfollowed this user
          </div>
        )}
        {showConfirm && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center">
            <div className="bg-white rounded-lg p-6 max-w-sm">
              <h3 className="font-semibold text-lg mb-2">Unfollow User?</h3>
              <p className="text-sm text-gray-600 mb-4">
                Are you sure you want to unfollow this user? Their posts will no
                longer appear in your feed.
              </p>
              <div className="flex gap-3">
                <button
                  onClick={() => setShowConfirm(false)}
                  className="flex-1 px-4 py-2 border rounded hover:bg-gray-50"
                >
                  Cancel
                </button>
                <button
                  onClick={() => {
                    setShowConfirm(false);
                    setIsFollowing(false);
                  }}
                  className="flex-1 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                >
                  Unfollow
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    );
  },
};

// With follower count
export const WithFollowerCount: Story = {
  render: () => {
    const [followers, setFollowers] = useState(1234);
    const [isFollowing, setIsFollowing] = useState(true);

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
          <UnfollowButton
            onClick={() => {
              setIsFollowing(false);
              setFollowers(followers - 1);
            }}
          />
        ) : (
          <div
            className="cursor-pointer px-3 py-1.5 w-fit rounded-full bg-blue-500 text-white text-xs font-bold hover:bg-blue-600"
            onClick={() => {
              setIsFollowing(true);
              setFollowers(followers + 1);
            }}
          >
            Follow
          </div>
        )}
      </div>
    );
  },
};

// Multiple buttons
export const MultipleButtons: Story = {
  render: () => (
    <div className="flex flex-wrap gap-3">
      <UnfollowButton onClick={() => {}} />
      <UnfollowButton onClick={() => {}} />
      <UnfollowButton onClick={() => {}} />
      <UnfollowButton onClick={() => {}} />
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
          <UnfollowButton onClick={() => {}} />
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

// Both states side by side
export const BothStates: Story = {
  render: () => (
    <div className="flex gap-8 items-center">
      <div className="text-center">
        <div className="mb-2">
          <div
            className="cursor-pointer px-3 py-1.5 rounded-full bg-blue-500 text-white text-xs font-bold hover:bg-blue-600"
            onClick={() => {}}
          >
            Follow
          </div>
        </div>
        <p className="text-xs text-gray-600">Not Following</p>
      </div>
      <div className="text-center">
        <div className="mb-2">
          <UnfollowButton onClick={() => {}} />
        </div>
        <p className="text-xs text-gray-600">Following (hover to unfollow)</p>
      </div>
    </div>
  ),
};
