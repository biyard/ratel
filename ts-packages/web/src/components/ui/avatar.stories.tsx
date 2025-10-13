import type { Meta, StoryObj } from '@storybook/react';
import { Avatar, AvatarImage, AvatarFallback } from './avatar';

const meta = {
  title: 'UI/Avatar',
  component: Avatar,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof Avatar>;

export default meta;
type Story = StoryObj<typeof meta>;

// With image
export const WithImage: Story = {
  render: () => (
    <Avatar>
      <AvatarImage
        src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
        alt="Avatar"
      />
      <AvatarFallback>JD</AvatarFallback>
    </Avatar>
  ),
};

// With fallback
export const WithFallback: Story = {
  render: () => (
    <Avatar>
      <AvatarImage src="/invalid-image.jpg" alt="Avatar" />
      <AvatarFallback>JD</AvatarFallback>
    </Avatar>
  ),
};

// Fallback only
export const FallbackOnly: Story = {
  render: () => (
    <Avatar>
      <AvatarFallback>AB</AvatarFallback>
    </Avatar>
  ),
};

// Different sizes
export const Sizes: Story = {
  render: () => (
    <div className="flex gap-4 items-center">
      <Avatar className="size-6">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="Small"
        />
        <AvatarFallback className="text-xs">XS</AvatarFallback>
      </Avatar>
      <Avatar className="size-8">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="Default"
        />
        <AvatarFallback className="text-sm">SM</AvatarFallback>
      </Avatar>
      <Avatar className="size-12">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="Medium"
        />
        <AvatarFallback>MD</AvatarFallback>
      </Avatar>
      <Avatar className="size-16">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="Large"
        />
        <AvatarFallback className="text-lg">LG</AvatarFallback>
      </Avatar>
      <Avatar className="size-24">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="Extra Large"
        />
        <AvatarFallback className="text-2xl">XL</AvatarFallback>
      </Avatar>
    </div>
  ),
};

// With different fallback styles
export const FallbackStyles: Story = {
  render: () => (
    <div className="flex gap-4 items-center">
      <Avatar>
        <AvatarFallback className="bg-blue-500 text-white">JD</AvatarFallback>
      </Avatar>
      <Avatar>
        <AvatarFallback className="bg-green-500 text-white">AB</AvatarFallback>
      </Avatar>
      <Avatar>
        <AvatarFallback className="bg-red-500 text-white">XY</AvatarFallback>
      </Avatar>
      <Avatar>
        <AvatarFallback className="bg-purple-500 text-white">
          MN
        </AvatarFallback>
      </Avatar>
      <Avatar>
        <AvatarFallback className="bg-yellow-500 text-black">
          PQ
        </AvatarFallback>
      </Avatar>
    </div>
  ),
};

// Avatar group
export const AvatarGroup: Story = {
  render: () => (
    <div className="flex -space-x-2">
      <Avatar className="border-2 border-white">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="User 1"
        />
        <AvatarFallback>U1</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-white">
        <AvatarImage
          src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&h=100&fit=crop"
          alt="User 2"
        />
        <AvatarFallback>U2</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-white">
        <AvatarImage
          src="https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&h=100&fit=crop"
          alt="User 3"
        />
        <AvatarFallback>U3</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-white">
        <AvatarImage
          src="https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100&h=100&fit=crop"
          alt="User 4"
        />
        <AvatarFallback>U4</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-white">
        <AvatarFallback className="bg-gray-300 text-gray-600">
          +5
        </AvatarFallback>
      </Avatar>
    </div>
  ),
};

// With status indicator
export const WithStatusIndicator: Story = {
  render: () => (
    <div className="flex gap-8 items-center">
      <div className="relative">
        <Avatar>
          <AvatarImage
            src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
            alt="Online user"
          />
          <AvatarFallback>ON</AvatarFallback>
        </Avatar>
        <span className="absolute bottom-0 right-0 block size-3 rounded-full bg-green-500 ring-2 ring-white" />
      </div>
      <div className="relative">
        <Avatar>
          <AvatarImage
            src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&h=100&fit=crop"
            alt="Away user"
          />
          <AvatarFallback>AW</AvatarFallback>
        </Avatar>
        <span className="absolute bottom-0 right-0 block size-3 rounded-full bg-yellow-500 ring-2 ring-white" />
      </div>
      <div className="relative">
        <Avatar>
          <AvatarImage
            src="https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&h=100&fit=crop"
            alt="Busy user"
          />
          <AvatarFallback>BS</AvatarFallback>
        </Avatar>
        <span className="absolute bottom-0 right-0 block size-3 rounded-full bg-red-500 ring-2 ring-white" />
      </div>
      <div className="relative">
        <Avatar>
          <AvatarImage
            src="https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100&h=100&fit=crop"
            alt="Offline user"
          />
          <AvatarFallback>OF</AvatarFallback>
        </Avatar>
        <span className="absolute bottom-0 right-0 block size-3 rounded-full bg-gray-400 ring-2 ring-white" />
      </div>
    </div>
  ),
};

// User profile card
export const UserProfileCard: Story = {
  render: () => (
    <div className="flex items-center gap-4 p-4 border rounded-lg max-w-sm">
      <Avatar className="size-16">
        <AvatarImage
          src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
          alt="John Doe"
        />
        <AvatarFallback>JD</AvatarFallback>
      </Avatar>
      <div className="flex-1">
        <h3 className="font-semibold">John Doe</h3>
        <p className="text-sm text-gray-500">Software Engineer</p>
        <p className="text-xs text-gray-400">john@example.com</p>
      </div>
    </div>
  ),
};

// Comment thread
export const CommentThread: Story = {
  render: () => (
    <div className="flex flex-col gap-4 max-w-md">
      <div className="flex gap-3">
        <Avatar className="size-10">
          <AvatarImage
            src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop"
            alt="User 1"
          />
          <AvatarFallback>U1</AvatarFallback>
        </Avatar>
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <span className="font-semibold text-sm">John Doe</span>
            <span className="text-xs text-gray-500">2 hours ago</span>
          </div>
          <p className="text-sm mt-1">
            This is a great feature! Looking forward to using it.
          </p>
        </div>
      </div>
      <div className="flex gap-3">
        <Avatar className="size-10">
          <AvatarImage
            src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&h=100&fit=crop"
            alt="User 2"
          />
          <AvatarFallback>U2</AvatarFallback>
        </Avatar>
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <span className="font-semibold text-sm">Jane Smith</span>
            <span className="text-xs text-gray-500">1 hour ago</span>
          </div>
          <p className="text-sm mt-1">
            Agreed! The design looks fantastic as well.
          </p>
        </div>
      </div>
    </div>
  ),
};
