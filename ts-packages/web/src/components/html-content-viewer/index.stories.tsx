import type { Meta, StoryObj } from '@storybook/react';
import HtmlContentViewer from './index';

const meta: Meta<typeof HtmlContentViewer> = {
  title: 'Components/HtmlContentViewer',
  component: HtmlContentViewer,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    htmlContent: {
      control: { type: 'text' },
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    htmlContent:
      '<p>This is a simple paragraph with <strong>bold</strong> and <em>italic</em> text.</p>',
  },
};

export const WithHeadings: Story = {
  args: {
    htmlContent: `
      <h1>Main Title</h1>
      <h2>Subtitle</h2>
      <p>Some content under the headings.</p>
    `,
  },
};

export const WithLists: Story = {
  args: {
    htmlContent: `
      <h3>Lists Example</h3>
      <ul>
        <li>Unordered item 1</li>
        <li>Unordered item 2</li>
      </ul>
      <ol>
        <li>Ordered item 1</li>
        <li>Ordered item 2</li>
      </ol>
    `,
  },
};

export const ComplexContent: Story = {
  args: {
    htmlContent: `
      <h1>Complex HTML</h1>
      <p>This is a <strong>bold</strong> paragraph with <u>underline</u> and <em>emphasis</em>.</p>
      <h2>Lists</h2>
      <ul>
        <li>Item 1</li>
        <li>Item 2 with <strong>bold</strong></li>
      </ul>
      <p>End of content.</p>
    `,
  },
};
