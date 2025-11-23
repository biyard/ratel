import type { Meta, StoryObj } from '@storybook/react';
import { Input } from './input';

const meta = {
  title: 'UI/Input',
  component: Input,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'post'],
      description: 'Visual variant of the input',
    },
    showRequiredMarker: {
      control: 'boolean',
      description: 'Show a red asterisk for required fields',
    },
    type: {
      control: 'select',
      options: [
        'text',
        'email',
        'password',
        'number',
        'tel',
        'url',
        'search',
        'date',
        'time',
        'datetime-local',
      ],
      description: 'The type of input',
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-80">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default input
export const Default: Story = {
  args: {
    placeholder: 'Enter text...',
    type: 'text',
    variant: 'default',
  },
};

// Post variant input
export const PostVariant: Story = {
  args: {
    placeholder: 'Enter post title...',
    type: 'text',
    variant: 'post',
  },
};

// Post variant with required marker
export const PostWithRequiredMarker: Story = {
  args: {
    placeholder: 'Title',
    type: 'text',
    variant: 'post',
    showRequiredMarker: true,
  },
};

// Email input
export const Email: Story = {
  args: {
    type: 'email',
    placeholder: 'email@example.com',
  },
};

// Password input
export const Password: Story = {
  args: {
    type: 'password',
    placeholder: 'Enter password',
  },
};

// Number input
export const Number: Story = {
  args: {
    type: 'number',
    placeholder: 'Enter number',
  },
};

// Search input
export const Search: Story = {
  args: {
    type: 'search',
    placeholder: 'Search...',
  },
};

// Disabled state
export const Disabled: Story = {
  args: {
    placeholder: 'Disabled input',
    disabled: true,
    value: 'Cannot edit',
  },
};

// With value
export const WithValue: Story = {
  args: {
    defaultValue: 'Pre-filled value',
  },
};

// Invalid state
export const Invalid: Story = {
  args: {
    placeholder: 'Enter email',
    type: 'email',
    'aria-invalid': true,
  },
};

// Date input
export const Date: Story = {
  args: {
    type: 'date',
  },
};

// File input
export const File: Story = {
  args: {
    type: 'file',
  },
};

// Different states showcase
export const AllStates: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div>
        <label className="text-sm font-medium block mb-1">Default</label>
        <Input placeholder="Enter text..." />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">With Value</label>
        <Input defaultValue="Some text" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Disabled</label>
        <Input placeholder="Disabled" disabled />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Invalid</label>
        <Input
          placeholder="Invalid input"
          defaultValue="invalid@"
          aria-invalid
        />
      </div>
    </div>
  ),
};

// Different input types
export const InputTypes: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div>
        <label className="text-sm font-medium block mb-1">Text</label>
        <Input type="text" placeholder="Enter text" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Email</label>
        <Input type="email" placeholder="email@example.com" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Password</label>
        <Input type="password" placeholder="Password" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Number</label>
        <Input type="number" placeholder="123" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Tel</label>
        <Input type="tel" placeholder="+1 (555) 000-0000" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">URL</label>
        <Input type="url" placeholder="https://example.com" />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Search</label>
        <Input type="search" placeholder="Search..." />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Date</label>
        <Input type="date" />
      </div>
    </div>
  ),
};

// Form example
export const FormExample: Story = {
  render: () => (
    <form className="flex flex-col gap-4 w-80">
      <div>
        <label htmlFor="name" className="text-sm font-medium block mb-1">
          Full Name
        </label>
        <Input id="name" type="text" placeholder="John Doe" required />
      </div>
      <div>
        <label htmlFor="email" className="text-sm font-medium block mb-1">
          Email Address
        </label>
        <Input
          id="email"
          type="email"
          placeholder="john@example.com"
          required
        />
      </div>
      <div>
        <label htmlFor="phone" className="text-sm font-medium block mb-1">
          Phone Number
        </label>
        <Input id="phone" type="tel" placeholder="+1 (555) 000-0000" />
      </div>
      <button
        type="submit"
        className="mt-2 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
      >
        Submit
      </button>
    </form>
  ),
};
