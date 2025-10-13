import type { Meta, StoryObj } from '@storybook/react';
import FeedEmptyState from './feed-empty-state';

const meta = {
  title: 'App/Social/FeedEmptyState',
  component: FeedEmptyState,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    message: {
      control: 'text',
      description: 'Empty state message to display',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-[500px]">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof FeedEmptyState>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default message
export const Default: Story = {};

// Custom message
export const CustomMessage: Story = {
  args: {
    message: 'No posts available at the moment',
  },
};

// No followers message
export const NoFollowers: Story = {
  args: {
    message: 'You have no followers yet',
  },
};

// No notifications
export const NoNotifications: Story = {
  args: {
    message: 'No notifications to show',
  },
};

// Search no results
export const SearchNoResults: Story = {
  args: {
    message: 'No results found for your search',
  },
};

// Empty drafts
export const EmptyDrafts: Story = {
  args: {
    message: 'You have no draft posts',
  },
};

// In a feed container
export const InFeedContainer: Story = {
  render: () => (
    <div className="max-w-2xl border rounded-lg">
      <div className="border-b p-4">
        <h2 className="font-semibold text-lg">My Feed</h2>
      </div>
      <div className="p-4">
        <FeedEmptyState />
      </div>
    </div>
  ),
};

// Multiple empty states
export const MultipleStates: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div>
        <h4 className="font-semibold text-sm mb-2">Default</h4>
        <FeedEmptyState />
      </div>
      <div>
        <h4 className="font-semibold text-sm mb-2">No Posts</h4>
        <FeedEmptyState message="No posts to display" />
      </div>
      <div>
        <h4 className="font-semibold text-sm mb-2">No Comments</h4>
        <FeedEmptyState message="No comments yet. Be the first to comment!" />
      </div>
      <div>
        <h4 className="font-semibold text-sm mb-2">No Messages</h4>
        <FeedEmptyState message="Your inbox is empty" />
      </div>
    </div>
  ),
};

// With action button
export const WithActionButton: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <FeedEmptyState message="No posts yet" />
      <button className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 w-fit mx-auto">
        Create Your First Post
      </button>
    </div>
  ),
};

// In different containers
export const DifferentContainers: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div>
        <h4 className="font-semibold mb-2">In a Sidebar</h4>
        <div className="w-64 border rounded-lg p-3">
          <FeedEmptyState message="No activity" />
        </div>
      </div>
      <div>
        <h4 className="font-semibold mb-2">In Main Content</h4>
        <div className="w-full border rounded-lg p-4">
          <FeedEmptyState message="No content available" />
        </div>
      </div>
      <div>
        <h4 className="font-semibold mb-2">In a Modal</h4>
        <div className="w-96 border rounded-lg p-4 shadow-lg">
          <h3 className="font-semibold mb-3">Select an Item</h3>
          <FeedEmptyState message="No items to select" />
        </div>
      </div>
    </div>
  ),
};

// With icon (custom implementation)
export const WithIcon: Story = {
  render: () => (
    <div className="flex flex-col items-center gap-4 p-6 border border-gray-500 rounded-lg">
      <svg
        className="w-16 h-16 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
        />
      </svg>
      <div className="text-gray-500 font-medium text-base">
        Feeds data is empty
      </div>
      <p className="text-sm text-gray-400 text-center max-w-xs">
        There are no feeds to display right now. Check back later or follow more users to see their posts.
      </p>
    </div>
  ),
};

// Contextual empty states
export const ContextualStates: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div className="border rounded-lg p-4">
        <h3 className="font-semibold mb-3">Followers</h3>
        <FeedEmptyState message="You don't have any followers yet. Share your profile to grow your network!" />
      </div>
      <div className="border rounded-lg p-4">
        <h3 className="font-semibold mb-3">Following</h3>
        <FeedEmptyState message="You're not following anyone yet. Discover interesting people to follow!" />
      </div>
      <div className="border rounded-lg p-4">
        <h3 className="font-semibold mb-3">Saved Posts</h3>
        <FeedEmptyState message="You haven't saved any posts yet. Save posts to access them later!" />
      </div>
    </div>
  ),
};
