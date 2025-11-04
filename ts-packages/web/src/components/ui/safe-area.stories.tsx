import type { Meta, StoryObj } from '@storybook/react';
import { SafeArea } from './safe-area';

const meta = {
  title: 'UI/SafeArea',
  component: SafeArea,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'row'],
      description: 'Layout variant - default (column) or row',
    },
    asChild: {
      control: 'boolean',
      description: 'Whether to render as a child component using Radix UI Slot',
    },
    className: {
      control: 'text',
      description: 'Additional CSS classes to apply',
    },
  },
} satisfies Meta<typeof SafeArea>;

export default meta;
type Story = StoryObj<typeof meta>;

// Helper component for demo content
const DemoCard = ({ children, className = '' }: { children: React.ReactNode; className?: string }) => (
  <div className={`p-6 bg-primary/10 border border-primary rounded-lg ${className}`}>
    {children}
  </div>
);

export const Default: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea>
        <DemoCard>
          <h2 className="text-xl font-bold mb-2">Content 1</h2>
          <p className="text-gray-600">This content is within a safe area with default column layout.</p>
        </DemoCard>
        <DemoCard>
          <h2 className="text-xl font-bold mb-2">Content 2</h2>
          <p className="text-gray-600">The safe area provides consistent spacing and max width.</p>
        </DemoCard>
        <DemoCard>
          <h2 className="text-xl font-bold mb-2">Content 3</h2>
          <p className="text-gray-600">Items are stacked vertically with gap-10 spacing.</p>
        </DemoCard>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Default SafeArea with column layout. Content is centered with max-w-desktop width, py-10 vertical padding, and gap-10 between items.',
      },
    },
  },
};

export const RowVariant: Story = {
  args: {
    variant: 'row',
  },
  render: (args) => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea {...args}>
        <DemoCard className="flex-1">
          <h2 className="text-lg font-bold mb-2">Column 1</h2>
          <p className="text-gray-600">Row layout variant.</p>
        </DemoCard>
        <DemoCard className="flex-1">
          <h2 className="text-lg font-bold mb-2">Column 2</h2>
          <p className="text-gray-600">Items arranged horizontally.</p>
        </DemoCard>
        <DemoCard className="flex-1">
          <h2 className="text-lg font-bold mb-2">Column 3</h2>
          <p className="text-gray-600">With gap-10 between them.</p>
        </DemoCard>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'SafeArea with row layout variant. Items are arranged horizontally with gap-10 spacing.',
      },
    },
  },
};

export const ResponsivePadding: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea>
        <div className="p-6 bg-yellow-100 border border-yellow-500 rounded-lg">
          <h2 className="text-xl font-bold mb-2">Responsive Padding Demo</h2>
          <p className="text-gray-700 mb-4">
            Resize your browser window to see the padding behavior:
          </p>
          <ul className="list-disc list-inside space-y-2 text-gray-700">
            <li><strong>Desktop (max-w-desktop):</strong> No horizontal padding (content fills the max width)</li>
            <li><strong>Mobile (below desktop breakpoint):</strong> px-2.5 horizontal padding on both sides</li>
            <li><strong>All sizes:</strong> py-10 vertical padding top and bottom</li>
          </ul>
        </div>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates responsive padding. On desktop, no horizontal padding is applied. On smaller screens, px-2.5 is added for comfortable margins.',
      },
    },
  },
};

export const MaxWidthConstraint: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea>
        <div className="p-6 bg-blue-100 border-2 border-blue-500 rounded-lg">
          <h2 className="text-xl font-bold mb-2">Max Width Constraint</h2>
          <p className="text-gray-700 mb-4">
            The SafeArea component constrains content to <code className="bg-blue-200 px-2 py-1 rounded">max-w-desktop</code> width and centers it horizontally with <code className="bg-blue-200 px-2 py-1 rounded">mx-auto</code>.
          </p>
          <p className="text-gray-700">
            This ensures content doesn't stretch too wide on large screens, maintaining optimal readability and visual balance.
          </p>
        </div>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Shows how the max-w-desktop constraint keeps content width reasonable on large screens.',
      },
    },
  },
};

export const PageLayoutExample: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea>
        {/* Header */}
        <div className="p-6 bg-white border border-gray-200 rounded-lg">
          <h1 className="text-3xl font-bold mb-2">Page Title</h1>
          <p className="text-gray-600">Welcome to this example page layout</p>
        </div>

        {/* Main Content */}
        <div className="p-6 bg-white border border-gray-200 rounded-lg">
          <h2 className="text-2xl font-bold mb-4">Main Content Section</h2>
          <p className="text-gray-600 mb-4">
            This demonstrates a typical page layout using SafeArea. Each section has consistent spacing thanks to the gap-10 property.
          </p>
          <p className="text-gray-600">
            The SafeArea ensures all content stays within readable width boundaries and maintains proper spacing on all screen sizes.
          </p>
        </div>

        {/* Cards Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="p-6 bg-white border border-gray-200 rounded-lg">
            <h3 className="text-lg font-bold mb-2">Feature 1</h3>
            <p className="text-gray-600">Description of feature 1</p>
          </div>
          <div className="p-6 bg-white border border-gray-200 rounded-lg">
            <h3 className="text-lg font-bold mb-2">Feature 2</h3>
            <p className="text-gray-600">Description of feature 2</p>
          </div>
          <div className="p-6 bg-white border border-gray-200 rounded-lg">
            <h3 className="text-lg font-bold mb-2">Feature 3</h3>
            <p className="text-gray-600">Description of feature 3</p>
          </div>
        </div>

        {/* Footer */}
        <div className="p-6 bg-white border border-gray-200 rounded-lg">
          <p className="text-gray-600 text-center">Footer content</p>
        </div>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Complete page layout with header, content sections, cards grid, and footer.',
      },
    },
  },
};

export const FormPageExample: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea>
        <div className="p-6 bg-white border border-gray-200 rounded-lg">
          <h1 className="text-2xl font-bold mb-2">Contact Form</h1>
          <p className="text-gray-600">Fill out the form below to get in touch</p>
        </div>

        <div className="p-6 bg-white border border-gray-200 rounded-lg">
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Name</label>
              <input
                type="text"
                placeholder="Enter your name"
                className="w-full px-3 py-2 border border-gray-300 rounded"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Email</label>
              <input
                type="email"
                placeholder="Enter your email"
                className="w-full px-3 py-2 border border-gray-300 rounded"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Message</label>
              <textarea
                placeholder="Enter your message"
                rows={4}
                className="w-full px-3 py-2 border border-gray-300 rounded"
              />
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-4">
          <button className="px-6 py-2 border border-gray-300 rounded hover:bg-gray-50">
            Cancel
          </button>
          <button className="px-6 py-2 bg-primary text-white rounded hover:opacity-90">
            Submit
          </button>
        </div>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Form page with proper spacing and layout.',
      },
    },
  },
};

export const SidebarLayoutExample: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea variant="row">
        <div className="w-64 p-6 bg-white border border-gray-200 rounded-lg">
          <h2 className="text-lg font-bold mb-4">Sidebar</h2>
          <nav className="space-y-2">
            <a href="#" className="block p-2 rounded hover:bg-gray-100">Dashboard</a>
            <a href="#" className="block p-2 rounded hover:bg-gray-100">Settings</a>
            <a href="#" className="block p-2 rounded hover:bg-gray-100">Profile</a>
            <a href="#" className="block p-2 rounded hover:bg-gray-100">Help</a>
          </nav>
        </div>
        <div className="flex-1 p-6 bg-white border border-gray-200 rounded-lg">
          <h1 className="text-2xl font-bold mb-4">Main Content</h1>
          <p className="text-gray-600 mb-4">
            Using the row variant for sidebar layouts. The sidebar has fixed width while the main content area grows to fill available space.
          </p>
          <p className="text-gray-600">
            This is a common pattern for dashboard and application layouts.
          </p>
        </div>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Sidebar layout using the row variant. Perfect for dashboard-style interfaces.',
      },
    },
  },
};

export const CustomClassName: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen">
      <SafeArea className="py-20 gap-20 bg-gradient-to-br from-blue-50 to-purple-50">
        <DemoCard>
          <h2 className="text-xl font-bold mb-2">Custom Styling</h2>
          <p className="text-gray-600">
            You can override default styles using the className prop.
          </p>
        </DemoCard>
        <DemoCard>
          <h2 className="text-xl font-bold mb-2">Enhanced Spacing</h2>
          <p className="text-gray-600">
            This example has py-20 (instead of py-10) and gap-20 (instead of gap-10).
          </p>
        </DemoCard>
      </SafeArea>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates overriding default spacing and adding custom styles using className prop.',
      },
    },
  },
};

export const VariantComparison: Story = {
  render: () => (
    <div className="bg-gray-50 min-h-screen space-y-16">
      <div>
        <div className="text-center py-6 bg-white border-b">
          <h3 className="text-lg font-semibold">Default Variant (Column Layout)</h3>
        </div>
        <SafeArea>
          <DemoCard><p>Item 1</p></DemoCard>
          <DemoCard><p>Item 2</p></DemoCard>
          <DemoCard><p>Item 3</p></DemoCard>
        </SafeArea>
      </div>

      <div>
        <div className="text-center py-6 bg-white border-b">
          <h3 className="text-lg font-semibold">Row Variant (Horizontal Layout)</h3>
        </div>
        <SafeArea variant="row">
          <DemoCard className="flex-1"><p>Item 1</p></DemoCard>
          <DemoCard className="flex-1"><p>Item 2</p></DemoCard>
          <DemoCard className="flex-1"><p>Item 3</p></DemoCard>
        </SafeArea>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Side-by-side comparison of default (column) and row variants.',
      },
    },
  },
};
