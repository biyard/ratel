import type { Meta, StoryObj } from '@storybook/react';
import { TiptapEditor } from './tiptap-editor';
import { useState } from 'react';

const meta = {
  title: 'Components/TiptapEditor',
  component: TiptapEditor,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    content: {
      control: 'text',
      description: 'HTML content for the editor',
    },
    editable: {
      control: 'boolean',
      description: 'Whether the editor is editable',
    },
    showToolbar: {
      control: 'boolean',
      description: 'Show or hide the toolbar',
    },
    toolbarPosition: {
      control: 'radio',
      options: ['top', 'bottom'],
      description: 'Position of the toolbar',
    },
    minHeight: {
      control: 'text',
      description: 'Minimum height of the editor',
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text',
    },
    onUpdate: {
      action: 'content updated',
      description: 'Callback when content is updated',
    },
  },
  decorators: [
    (Story) => (
      <div className="w-full max-w-4xl mx-auto p-4">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof TiptapEditor>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default editor with toolbar
export const Default: Story = {
  args: {
    content: '<p>Start typing your content here...</p>',
    showToolbar: true,
    editable: true,
    placeholder: 'Type your script',
  },
};

// With initial content
export const WithContent: Story = {
  args: {
    content: `
      <h1>Welcome to Ratel Editor</h1>
      <p>This is a <strong>powerful</strong> and <em>flexible</em> rich text editor with a <u>comprehensive</u> toolbar.</p>
      <h2>Features</h2>
      <ul>
        <li>Text formatting (bold, italic, underline, strikethrough)</li>
        <li>Text and background colors</li>
        <li>Multiple heading levels</li>
        <li>Ordered and unordered lists</li>
      </ul>
      <h3>Getting Started</h3>
      <p>Simply click anywhere and start typing. Use the toolbar above to format your text.</p>
    `,
    showToolbar: true,
    editable: true,
  },
};

// Dark mode (default)
export const DarkMode: Story = {
  args: {
    content:
      '<h2>Dark Theme Editor</h2><p>This editor adapts to the dark theme automatically.</p>',
    showToolbar: true,
  },
  parameters: {
    backgrounds: { default: 'dark' },
  },
};

// Light mode
export const LightMode: Story = {
  args: {
    content:
      '<h2>Light Theme Editor</h2><p>This editor adapts to the light theme automatically.</p>',
    showToolbar: true,
  },
  decorators: [
    (Story) => (
      <div data-theme="light" className="w-full max-w-4xl mx-auto p-4">
        <Story />
      </div>
    ),
  ],
  parameters: {
    backgrounds: { default: 'light' },
  },
};

// Toolbar at bottom
export const ToolbarBottom: Story = {
  args: {
    content: '<p>The toolbar is positioned at the bottom of the editor.</p>',
    showToolbar: true,
    toolbarPosition: 'bottom',
  },
};

// No toolbar
export const NoToolbar: Story = {
  args: {
    content:
      '<p>This editor has no toolbar, showing only the content area.</p>',
    showToolbar: false,
  },
};

// Read-only mode
export const ReadOnly: Story = {
  args: {
    content:
      '<h2>Read-Only Content</h2><p>This editor is in <strong>read-only</strong> mode. You cannot edit the content.</p>',
    editable: false,
    showToolbar: false,
  },
};

// Custom height
export const CustomHeight: Story = {
  args: {
    content: '<p>This editor has a custom minimum height of 400px.</p>',
    minHeight: '400px',
    showToolbar: true,
  },
};

// With max height and scroll
export const ScrollableContent: Story = {
  args: {
    content: `
      <h1>Long Content</h1>
      <p>This editor has a maximum height and will scroll when content exceeds it.</p>
      ${Array.from({ length: 20 }, (_, i) => `<p>Paragraph ${i + 1}: Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>`).join('')}
    `,
    minHeight: '200px',
    maxHeight: '400px',
    showToolbar: true,
  },
};

// Limited features
export const LimitedFeatures: Story = {
  args: {
    content: '<p>This editor has limited formatting options.</p>',
    showToolbar: true,
    enabledFeatures: {
      bold: true,
      italic: true,
      underline: true,
      heading: true,
      lists: true,
      textColor: false,
      highlight: false,
      strike: false,
    },
  },
};

// Only basic formatting
export const BasicFormatting: Story = {
  args: {
    content: '<p>Only basic text formatting is available.</p>',
    showToolbar: true,
    enabledFeatures: {
      bold: true,
      italic: true,
      underline: true,
      strike: true,
    },
  },
};

// Interactive with state tracking
export const Interactive: Story = {
  render: () => {
    const [content, setContent] = useState(
      '<h2>Interactive Editor</h2><p>Start typing to see the content update below...</p>',
    );
    const [focused, setFocused] = useState(false);

    return (
      <div className="flex flex-col gap-4">
        <div>
          <h3 className="text-lg font-semibold mb-2 text-foreground">Editor</h3>
          <div className="relative">
            <TiptapEditor
              content={content}
              onUpdate={setContent}
              onFocus={() => setFocused(true)}
              onBlur={() => setFocused(false)}
              showToolbar={true}
              editable={true}
            />
            {focused && (
              <div className="absolute -top-8 right-0 text-xs text-primary font-medium">
                ● Focused
              </div>
            )}
          </div>
        </div>
        <div>
          <h3 className="text-lg font-semibold mb-2 text-foreground">
            HTML Output
          </h3>
          <pre className="bg-card border border-border p-4 rounded text-xs overflow-auto max-h-60 text-foreground">
            {content}
          </pre>
        </div>
        <div>
          <h3 className="text-lg font-semibold mb-2 text-foreground">
            Character Count
          </h3>
          <p className="text-sm text-foreground-muted">
            {content.replace(/<[^>]*>/g, '').length} characters
          </p>
        </div>
      </div>
    );
  },
};

// Responsive test
export const ResponsiveTest: Story = {
  args: {
    content:
      '<p>Resize your browser window to test the responsive toolbar behavior.</p>',
    showToolbar: true,
  },
  parameters: {
    viewport: {
      viewports: {
        mobile: {
          name: 'Mobile',
          styles: { width: '375px', height: '667px' },
        },
        tablet: {
          name: 'Tablet',
          styles: { width: '768px', height: '1024px' },
        },
        desktop: {
          name: 'Desktop',
          styles: { width: '1440px', height: '900px' },
        },
      },
    },
  },
};

// All features showcase
export const AllFeatures: Story = {
  args: {
    content: `
      <h1>Heading 1</h1>
      <h2>Heading 2</h2>
      <h3>Heading 3</h3>
      <p>Normal paragraph with <strong>bold</strong>, <em>italic</em>, <u>underline</u>, and <s>strikethrough</s> text.</p>
      <p style="color: #ef4444">Red colored text</p>
      <p style="color: #3b82f6">Blue colored text</p>
      <p><mark style="background-color: #fcb300">Highlighted text with yellow background</mark></p>
      <h3>Lists</h3>
      <ul>
        <li>Unordered list item 1</li>
        <li>Unordered list item 2</li>
        <li>Unordered list item 3</li>
      </ul>
      <ol>
        <li>Ordered list item 1</li>
        <li>Ordered list item 2</li>
        <li>Ordered list item 3</li>
      </ol>
    `,
    showToolbar: true,
  },
};

// Empty editor
export const Empty: Story = {
  args: {
    content: '',
    placeholder: 'Start typing here...',
    showToolbar: true,
  },
};

// Compact size
export const Compact: Story = {
  args: {
    content: '<p>Compact editor with minimal height.</p>',
    minHeight: '120px',
    showToolbar: true,
  },
};
