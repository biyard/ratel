import type { Meta, StoryObj } from '@storybook/react';
import { Textarea } from './textarea';

const meta = {
  title: 'UI/Textarea',
  component: Textarea,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    placeholder: {
      control: 'text',
      description: 'Placeholder text',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the textarea is disabled',
    },
    rows: {
      control: 'number',
      description: 'Number of visible text lines',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-96">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default textarea
export const Default: Story = {
  args: {
    placeholder: 'Enter your message...',
  },
};

// With rows
export const WithRows: Story = {
  args: {
    placeholder: 'Enter your message...',
    rows: 5,
  },
};

// With value
export const WithValue: Story = {
  args: {
    defaultValue:
      'This is a pre-filled textarea with some content. You can edit this text.',
  },
};

// Disabled
export const Disabled: Story = {
  args: {
    placeholder: 'This textarea is disabled',
    disabled: true,
    defaultValue: 'You cannot edit this content',
  },
};

// Invalid state
export const Invalid: Story = {
  args: {
    placeholder: 'Enter your message...',
    defaultValue: 'This input has an error',
    'aria-invalid': true,
  },
};

// Different sizes
export const DifferentSizes: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-96">
      <div>
        <label className="text-sm font-medium block mb-1">Small (3 rows)</label>
        <Textarea rows={3} placeholder="Small textarea" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">
          Medium (5 rows)
        </label>
        <Textarea rows={5} placeholder="Medium textarea" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Large (10 rows)</label>
        <Textarea rows={10} placeholder="Large textarea" />
      </div>
    </div>
  ),
};

// All states
export const AllStates: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-96">
      <div>
        <label className="text-sm font-medium block mb-1">Default</label>
        <Textarea placeholder="Enter text..." />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">With Value</label>
        <Textarea defaultValue="This textarea has content" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Disabled</label>
        <Textarea placeholder="Disabled" disabled />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Invalid</label>
        <Textarea
          placeholder="Invalid textarea"
          defaultValue="This has an error"
          aria-invalid
        />
      </div>
    </div>
  ),
};

// Form example
export const FormExample: Story = {
  render: () => (
    <form className="flex flex-col gap-4 w-96">
      <div>
        <label htmlFor="subject" className="text-sm font-medium block mb-1">
          Subject
        </label>
        <input
          id="subject"
          type="text"
          className="w-full px-3 py-2 border rounded"
          placeholder="Enter subject"
        />
      </div>
      <div>
        <label htmlFor="message" className="text-sm font-medium block mb-1">
          Message
        </label>
        <Textarea
          id="message"
          placeholder="Write your message here..."
          rows={6}
        />
        <p className="text-xs text-gray-500 mt-1">
          Please provide as much detail as possible
        </p>
      </div>
      <button
        type="submit"
        className="mt-2 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
      >
        Send Message
      </button>
    </form>
  ),
};

// Character counter example
export const WithCharacterCount: Story = {
  render: () => {
    const [value, setValue] = React.useState('');
    const maxLength = 200;

    return (
      <div className="w-96">
        <label htmlFor="bio" className="text-sm font-medium block mb-1">
          Bio
        </label>
        <Textarea
          id="bio"
          placeholder="Tell us about yourself..."
          rows={4}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          maxLength={maxLength}
        />
        <div className="flex justify-between items-center mt-1">
          <p className="text-xs text-gray-500">Maximum {maxLength} characters</p>
          <p className="text-xs text-gray-500">
            {value.length}/{maxLength}
          </p>
        </div>
      </div>
    );
  },
};

// Auto-resize example (using field-sizing-content)
export const AutoResize: Story = {
  render: () => (
    <div className="w-96">
      <label htmlFor="auto" className="text-sm font-medium block mb-1">
        Auto-resizing Textarea
      </label>
      <Textarea
        id="auto"
        placeholder="This textarea will grow as you type..."
      />
      <p className="text-xs text-gray-500 mt-1">
        Uses CSS field-sizing-content for automatic resizing
      </p>
    </div>
  ),
};
