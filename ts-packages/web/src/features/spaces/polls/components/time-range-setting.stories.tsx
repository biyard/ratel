import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import { TimeRangeSetting } from './time-range-setting';

const meta: Meta<typeof TimeRangeSetting> = {
  title: 'Features/Spaces/Polls/TimeRangeSetting',
  component: TimeRangeSetting,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    startTimestampMillis: {
      control: 'number',
      description: 'Start time in milliseconds since epoch',
    },
    endTimestampMillis: {
      control: 'number',
      description: 'End time in milliseconds since epoch',
    },
    timezone: {
      control: 'text',
      description: 'Timezone string (e.g., "UTC+9", "PST", "GMT")',
    },
    onChangeStartTime: {
      action: 'start time changed',
      description: 'Callback when start time is changed',
    },
    onChangeEndTime: {
      action: 'end time changed',
      description: 'Callback when end time is changed',
    },
  },
};

export default meta;
type Story = StoryObj<typeof TimeRangeSetting>;

// Helper component with state management
const TimeRangeSettingWithState = (args: {
  initialStart: number;
  initialEnd: number;
  timezone: string;
}) => {
  const [startTime, setStartTime] = useState(args.initialStart);
  const [endTime, setEndTime] = useState(args.initialEnd);

  return (
    <div className="max-w-4xl">
      <TimeRangeSetting
        startTimestampMillis={startTime}
        endTimestampMillis={endTime}
        onChangeStartTime={setStartTime}
        onChangeEndTime={setEndTime}
        timezone={args.timezone}
      />
      <div className="mt-4 p-4 bg-gray-100 rounded-md">
        <h3 className="font-semibold mb-2">Selected Time Range:</h3>
        <p>
          <strong>Start:</strong> {new Date(startTime).toLocaleString()}
        </p>
        <p>
          <strong>End:</strong> {new Date(endTime).toLocaleString()}
        </p>
      </div>
    </div>
  );
};

// Get current time and future time
const now = new Date();
const oneHourLater = new Date(now.getTime() + 60 * 60 * 1000);
const oneDayLater = new Date(now.getTime() + 24 * 60 * 60 * 1000);
const oneWeekLater = new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000);

export const Default: Story = {
  render: () => (
    <TimeRangeSettingWithState
      initialStart={now.getTime()}
      initialEnd={oneHourLater.getTime()}
      timezone="UTC+9"
    />
  ),
};

export const ShortDuration: Story = {
  render: () => (
    <TimeRangeSettingWithState
      initialStart={now.getTime()}
      initialEnd={oneHourLater.getTime()}
      timezone="UTC+9"
    />
  ),
};

export const OneDayDuration: Story = {
  render: () => (
    <TimeRangeSettingWithState
      initialStart={now.getTime()}
      initialEnd={oneDayLater.getTime()}
      timezone="PST"
    />
  ),
};

export const OneWeekDuration: Story = {
  render: () => (
    <TimeRangeSettingWithState
      initialStart={now.getTime()}
      initialEnd={oneWeekLater.getTime()}
      timezone="GMT"
    />
  ),
};

export const DifferentTimezones: Story = {
  render: () => (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold mb-2">Korea Time (UTC+9)</h3>
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="UTC+9"
        />
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-2">Pacific Time (PST)</h3>
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="PST"
        />
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-2">
          Eastern Time (EST)
        </h3>
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="EST"
        />
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-2">
          Greenwich Mean Time (GMT)
        </h3>
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="GMT"
        />
      </div>
    </div>
  ),
};

export const CustomStyling: Story = {
  render: () => (
    <TimeRangeSettingWithState
      initialStart={now.getTime()}
      initialEnd={oneHourLater.getTime()}
      timezone="UTC+9"
    />
  ),
};

export const PollSchedule: Story = {
  render: () => {
    const pollStart = new Date();
    pollStart.setDate(pollStart.getDate() + 1);
    pollStart.setHours(9, 0, 0, 0);

    const pollEnd = new Date(pollStart);
    pollEnd.setDate(pollEnd.getDate() + 7);
    pollEnd.setHours(17, 0, 0, 0);

    return (
      <div>
        <h2 className="text-xl font-bold mb-4">Poll Voting Period</h2>
        <TimeRangeSettingWithState
          initialStart={pollStart.getTime()}
          initialEnd={pollEnd.getTime()}
          timezone="UTC+9"
        />
        <p className="mt-4 text-sm text-gray-600">
          This poll will be open for voting from {pollStart.toLocaleDateString()}{' '}
          at 9:00 AM to {pollEnd.toLocaleDateString()} at 5:00 PM (UTC+9)
        </p>
      </div>
    );
  },
};

export const EventSchedule: Story = {
  render: () => {
    const eventStart = new Date();
    eventStart.setDate(eventStart.getDate() + 14);
    eventStart.setHours(14, 0, 0, 0);

    const eventEnd = new Date(eventStart);
    eventEnd.setHours(16, 0, 0, 0);

    return (
      <div>
        <h2 className="text-xl font-bold mb-4">Event Time</h2>
        <TimeRangeSettingWithState
          initialStart={eventStart.getTime()}
          initialEnd={eventEnd.getTime()}
          timezone="PST"
        />
        <p className="mt-4 text-sm text-gray-600">
          Event scheduled for {eventStart.toLocaleDateString()} from{' '}
          {eventStart.toLocaleTimeString()} to {eventEnd.toLocaleTimeString()} (PST)
        </p>
      </div>
    );
  },
};

export const Responsive: Story = {
  render: () => (
    <div>
      <h3 className="text-lg font-semibold mb-2">Desktop View</h3>
      <div className="border p-4 mb-8">
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="UTC+9"
        />
      </div>

      <h3 className="text-lg font-semibold mb-2">Tablet View (max-width: 768px)</h3>
      <div className="border p-4 max-w-md">
        <TimeRangeSettingWithState
          initialStart={now.getTime()}
          initialEnd={oneHourLater.getTime()}
          timezone="UTC+9"
        />
      </div>
    </div>
  ),
};

export const StaticView: Story = {
  args: {
    startTimestampMillis: now.getTime(),
    endTimestampMillis: oneHourLater.getTime(),
    timezone: 'UTC+9',
    onChangeStartTime: () => {},
    onChangeEndTime: () => {},
  },
};
