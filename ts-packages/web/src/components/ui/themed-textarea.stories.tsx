import type { Meta, StoryObj } from '@storybook/react';
import { ThemedTextarea } from './themed-textarea';

const meta = {
  title: 'UI/ThemedTextarea',
  component: ThemedTextarea,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'post'],
      description: 'Visual variant of the textarea',
    },
    disabled: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof ThemedTextarea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    placeholder: 'Enter multiple lines...',
    variant: 'default',
  },
};

export const PostVariant: Story = {
  args: {
    placeholder: 'Write your post content...',
    variant: 'post',
    rows: 6,
  },
};

export const Disabled: Story = {
  args: {
    placeholder: 'Disabled textarea',
    variant: 'post',
    disabled: true,
    rows: 4,
  },
};

export const WithValue: Story = {
  args: {
    placeholder: 'Content',
    variant: 'post',
    defaultValue: 'Sample post content that spans multiple lines.\n\nYou can write as much as you want here.',
    rows: 6,
  },
};

export const AllVariants: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-96">
      <div>
        <label className="block text-sm font-medium mb-2">Default Variant</label>
        <ThemedTextarea placeholder="Default textarea..." variant="default" rows={4} />
      </div>
      <div>
        <label className="block text-sm font-medium mb-2">Post Variant</label>
        <ThemedTextarea placeholder="Post textarea..." variant="post" rows={4} />
      </div>
      <div>
        <label className="block text-sm font-medium mb-2">Disabled</label>
        <ThemedTextarea placeholder="Disabled..." variant="post" disabled rows={4} />
      </div>
    </div>
  ),
};
