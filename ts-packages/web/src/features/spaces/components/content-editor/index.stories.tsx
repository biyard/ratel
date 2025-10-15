import type { Meta, StoryObj } from '@storybook/react';
import SpaceContentEditor from './index';

const meta: Meta<typeof SpaceContentEditor> = {
  title: 'Features/Spaces/SpaceContentEditor',
  component: SpaceContentEditor,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    htmlContent: {
      control: { type: 'text' },
    },
    isEditMode: {
      control: { type: 'boolean' },
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const ViewMode: Story = {
  args: {
    htmlContent:
      '<p>This is view mode content with <strong>bold</strong> text.</p>',
    isEditMode: false,
    onContentChange: (newContent: string) =>
      console.log('Content changed:', newContent),
  },
};

export const EditMode: Story = {
  args: {
    htmlContent: '<p>This is edit mode content.</p>',
    isEditMode: true,
    onContentChange: (newContent: string) =>
      console.log('Content changed:', newContent),
  },
};

export const WithComplexContent: Story = {
  args: {
    htmlContent: `
      <h1>Complex Content</h1>
      <p>This is a paragraph with <em>emphasis</em> and <strong>bold</strong>.</p>
      <ul>
        <li>List item 1</li>
        <li>List item 2</li>
      </ul>
    `,
    isEditMode: false,
    onContentChange: (newContent: string) =>
      console.log('Content changed:', newContent),
  },
};
