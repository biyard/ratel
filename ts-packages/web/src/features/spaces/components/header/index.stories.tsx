import type { Meta, StoryObj } from '@storybook/react';
import SpaceHeader, { SpaceHeaderProps } from './index';
import { Post } from '@/features/posts/types/post';
import { PopupProvider } from '@/lib/contexts/popup-service';
import {
  SpacePublishState,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import { MemoryRouter } from 'react-router';

const mockPost: Post = {
  id: 1,
  title: 'Sample Post Title',
  html_contents: '<p>Sample content</p>',
  likes: 10,
  shares: 5,
  comments: 3,
  rewards: 2,
  author_type: 'user',
  author_profile_url: 'https://example.com/profile.jpg',
  author_display_name: 'John Doe',
  created_at: '2023-01-01T00:00:00Z',
};

const mockVisibility: SpaceVisibility = { type: 'Private' };
const mockPublishState: SpacePublishState = 'Draft';

const meta: Meta<typeof SpaceHeader> = {
  title: 'Features/Spaces/SpaceHeader',
  component: SpaceHeader,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    isEditable: { control: 'boolean' },
    hasEditPermission: { control: 'boolean' },
    isEditingMode: { control: 'boolean' },
    hasUnsavedChanges: { control: 'boolean' },
    publishState: { control: 'select', options: ['Draft', 'Published'] },
  },
  decorators: [
    (Story) => (
      <div className="max-w-desktop w-full p-4">
        <MemoryRouter>
          <PopupProvider>
            <Story />
          </PopupProvider>
        </MemoryRouter>
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    post: mockPost,
    title: 'Sample Title',
    isEditable: true,
    hasEditPermission: true,
    isEditingMode: false,
    hasUnsavedChanges: false,
    visibility: mockVisibility,
    publishState: mockPublishState,
    onStartEdit: () => console.log('Start Edit'),
    onStopEdit: () => console.log('Stop Edit'),
    onSave: async () => console.log('Save'),
    onMakePublic: async () => console.log('Make Public'),
    onPublish: async (type: string) => console.log('Publish', type),
    updateTitle: (newTitle: string) => console.log('Update Title', newTitle),
  },
};

export const EditingMode: Story = {
  args: {
    ...Default.args,
    isEditingMode: true,
    hasUnsavedChanges: true,
    title: 'Edited Title',
  },
};

export const NoEditPermission: Story = {
  args: {
    ...Default.args,
    hasEditPermission: false,
  },
};

export const Published: Story = {
  args: {
    ...Default.args,
    publishState: 'Published',
    visibility: { type: 'Public' },
  },
};
