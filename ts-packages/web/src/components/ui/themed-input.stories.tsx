import type { Meta, StoryObj } from '@storybook/react';
import { ThemedInput } from './themed-input';

const meta = {
  title: 'UI/ThemedInput',
  component: ThemedInput,
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
    disabled: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof ThemedInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    placeholder: 'Enter text...',
    variant: 'default',
  },
};

export const PostVariant: Story = {
  args: {
    placeholder: 'Enter post content...',
    variant: 'post',
  },
};

export const WithRequiredMarker: Story = {
  args: {
    placeholder: 'Title',
    variant: 'post',
    showRequiredMarker: true,
  },
};

export const Disabled: Story = {
  args: {
    placeholder: 'Disabled input',
    variant: 'post',
    disabled: true,
  },
};

export const WithValue: Story = {
  args: {
    placeholder: 'Title',
    variant: 'post',
    defaultValue: 'Sample post title',
    showRequiredMarker: true,
  },
};

export const AllVariants: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div>
        <label className="block text-sm font-medium mb-2">Default Variant</label>
        <ThemedInput placeholder="Default input..." variant="default" />
      </div>
      <div>
        <label className="block text-sm font-medium mb-2">Post Variant</label>
        <ThemedInput placeholder="Post input..." variant="post" />
      </div>
      <div>
        <label className="block text-sm font-medium mb-2">With Required Marker</label>
        <ThemedInput placeholder="Title" variant="post" showRequiredMarker />
      </div>
      <div>
        <label className="block text-sm font-medium mb-2">Disabled</label>
        <ThemedInput placeholder="Disabled..." variant="post" disabled />
      </div>
    </div>
  ),
};
