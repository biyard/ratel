import type { Meta, StoryObj } from '@storybook/react';
import TimeAgo from './time-ago';

const meta = {
  title: 'Components/TimeAgo',
  component: TimeAgo,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    timestamp: {
      control: 'number',
      description: 'Unix timestamp in seconds',
    },
  },
} satisfies Meta<typeof TimeAgo>;

export default meta;
type Story = StoryObj<typeof meta>;

// Just now
export const JustNow: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000),
  },
};

// 1 minute ago
export const OneMinuteAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 60,
  },
};

// 5 minutes ago
export const FiveMinutesAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 300,
  },
};

// 1 hour ago
export const OneHourAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 3600,
  },
};

// 1 day ago
export const OneDayAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 86400,
  },
};

// 1 week ago
export const OneWeekAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 604800,
  },
};

// 1 month ago
export const OneMonthAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 2592000,
  },
};

// 1 year ago
export const OneYearAgo: Story = {
  args: {
    timestamp: Math.floor(Date.now() / 1000) - 31536000,
  },
};

// Various timestamps showcase
export const VariousTimestamps: Story = {
  render: () => {
    const now = Math.floor(Date.now() / 1000);
    const timestamps = [
      { label: 'Just now', value: now },
      { label: '30 seconds ago', value: now - 30 },
      { label: '2 minutes ago', value: now - 120 },
      { label: '15 minutes ago', value: now - 900 },
      { label: '1 hour ago', value: now - 3600 },
      { label: '6 hours ago', value: now - 21600 },
      { label: '1 day ago', value: now - 86400 },
      { label: '3 days ago', value: now - 259200 },
      { label: '1 week ago', value: now - 604800 },
      { label: '2 weeks ago', value: now - 1209600 },
      { label: '1 month ago', value: now - 2592000 },
      { label: '6 months ago', value: now - 15552000 },
      { label: '1 year ago', value: now - 31536000 },
    ];

    return (
      <div className="flex flex-col gap-2">
        {timestamps.map((ts) => (
          <div
            key={ts.value}
            className="flex items-center justify-between gap-8 py-1"
          >
            <span className="text-sm text-gray-500 min-w-[120px]">
              {ts.label}:
            </span>
            <TimeAgo timestamp={ts.value} />
          </div>
        ))}
      </div>
    );
  },
};

// In a comment/post context
export const InCommentContext: Story = {
  render: () => {
    const now = Math.floor(Date.now() / 1000);
    const comments = [
      { author: 'John Doe', time: now - 120, text: 'Great post!' },
      {
        author: 'Jane Smith',
        time: now - 3600,
        text: 'Very informative, thanks for sharing.',
      },
      {
        author: 'Bob Johnson',
        time: now - 86400,
        text: 'I have a question about this...',
      },
    ];

    return (
      <div className="max-w-md space-y-4">
        {comments.map((comment, idx) => (
          <div key={idx} className="border rounded-lg p-4">
            <div className="flex items-center justify-between mb-2">
              <span className="font-semibold text-sm">{comment.author}</span>
              <TimeAgo timestamp={comment.time} />
            </div>
            <p className="text-sm text-gray-700">{comment.text}</p>
          </div>
        ))}
      </div>
    );
  },
};

// In a feed/timeline
export const InFeedContext: Story = {
  render: () => {
    const now = Math.floor(Date.now() / 1000);
    const posts = [
      { title: 'New Product Launch', time: now - 300 },
      { title: 'Team Meeting Notes', time: now - 7200 },
      { title: 'Monthly Newsletter', time: now - 172800 },
      { title: 'Annual Report 2024', time: now - 604800 },
    ];

    return (
      <div className="max-w-md space-y-3">
        {posts.map((post, idx) => (
          <div key={idx} className="flex items-start gap-3 p-3 border rounded">
            <div className="w-12 h-12 bg-gray-200 rounded flex-shrink-0" />
            <div className="flex-1">
              <div className="font-medium text-sm">{post.title}</div>
              <TimeAgo timestamp={post.time} />
            </div>
          </div>
        ))}
      </div>
    );
  },
};

// With profile/avatar
export const WithAvatar: Story = {
  render: () => {
    const now = Math.floor(Date.now() / 1000);
    const activities = [
      {
        user: 'Alice',
        action: 'liked your post',
        time: now - 600,
      },
      {
        user: 'Bob',
        action: 'commented on your photo',
        time: now - 10800,
      },
      {
        user: 'Charlie',
        action: 'started following you',
        time: now - 259200,
      },
    ];

    return (
      <div className="max-w-md space-y-3">
        {activities.map((activity, idx) => (
          <div key={idx} className="flex items-center gap-3">
            <div className="w-10 h-10 bg-blue-500 rounded-full flex-shrink-0" />
            <div className="flex-1">
              <p className="text-sm">
                <span className="font-semibold">{activity.user}</span>{' '}
                {activity.action}
              </p>
              <TimeAgo timestamp={activity.time} />
            </div>
          </div>
        ))}
      </div>
    );
  },
};
