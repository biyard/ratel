import type { Meta, StoryObj } from '@storybook/react';
import { Col } from './col';

const meta = {
  title: 'UI/Col',
  component: Col,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    mainAxisAlignment: {
      control: 'select',
      options: ['start', 'center', 'end', 'between'],
      description: 'Vertical alignment (justify-content in flex-col)',
    },
    crossAxisAlignment: {
      control: 'select',
      options: ['start', 'center', 'end', 'stretch'],
      description: 'Horizontal alignment (align-items in flex-col)',
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
} satisfies Meta<typeof Col>;

export default meta;
type Story = StoryObj<typeof meta>;

// Helper component for demo items
const DemoItem = ({ children, className = '' }: { children: React.ReactNode; className?: string }) => (
  <div className={`px-4 py-2 bg-primary/20 border border-primary rounded ${className}`}>
    {children}
  </div>
);

export const Default: Story = {
  render: () => (
    <Col className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Default column layout with gap-2.5 spacing between items.',
      },
    },
  },
};

export const MainAxisAlignmentStart: Story = {
  args: {
    mainAxisAlignment: 'start',
  },
  render: (args) => (
    <Col {...args} className="h-64 border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items aligned to the top (justify-start). This is the default behavior.',
      },
    },
  },
};

export const MainAxisAlignmentCenter: Story = {
  args: {
    mainAxisAlignment: 'center',
  },
  render: (args) => (
    <Col {...args} className="h-64 border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items vertically centered in the container (justify-center).',
      },
    },
  },
};

export const MainAxisAlignmentEnd: Story = {
  args: {
    mainAxisAlignment: 'end',
  },
  render: (args) => (
    <Col {...args} className="h-64 border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items aligned to the bottom (justify-end).',
      },
    },
  },
};

export const MainAxisAlignmentBetween: Story = {
  args: {
    mainAxisAlignment: 'between',
  },
  render: (args) => (
    <Col {...args} className="h-64 border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items distributed with equal space between them (justify-between).',
      },
    },
  },
};

export const CrossAxisAlignmentStart: Story = {
  args: {
    crossAxisAlignment: 'start',
  },
  render: (args) => (
    <Col {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="w-1/4">Small</DemoItem>
      <DemoItem className="w-1/2">Medium</DemoItem>
      <DemoItem className="w-3/4">Large</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items horizontally aligned to the left (items-start). This is the default behavior.',
      },
    },
  },
};

export const CrossAxisAlignmentCenter: Story = {
  args: {
    crossAxisAlignment: 'center',
  },
  render: (args) => (
    <Col {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="w-1/4">Small</DemoItem>
      <DemoItem className="w-1/2">Medium</DemoItem>
      <DemoItem className="w-3/4">Large</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items horizontally centered (items-center).',
      },
    },
  },
};

export const CrossAxisAlignmentEnd: Story = {
  args: {
    crossAxisAlignment: 'end',
  },
  render: (args) => (
    <Col {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="w-1/4">Small</DemoItem>
      <DemoItem className="w-1/2">Medium</DemoItem>
      <DemoItem className="w-3/4">Large</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items horizontally aligned to the right (items-end).',
      },
    },
  },
};

export const CrossAxisAlignmentStretch: Story = {
  args: {
    crossAxisAlignment: 'stretch',
  },
  render: (args) => (
    <Col {...args} className="border border-gray-300 rounded p-4">
      <DemoItem>Stretched Item 1</DemoItem>
      <DemoItem>Stretched Item 2</DemoItem>
      <DemoItem>Stretched Item 3</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items stretched to fill container width (items-stretch).',
      },
    },
  },
};

export const CombinedAlignment: Story = {
  render: () => (
    <div className="grid grid-cols-2 gap-4">
      <div>
        <p className="text-sm text-gray-500 mb-2">Center + Center:</p>
        <Col
          mainAxisAlignment="center"
          crossAxisAlignment="center"
          className="h-48 border border-gray-300 rounded p-4"
        >
          <DemoItem>Perfectly Centered</DemoItem>
        </Col>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Between + Center:</p>
        <Col
          mainAxisAlignment="between"
          crossAxisAlignment="center"
          className="h-48 border border-gray-300 rounded p-4"
        >
          <DemoItem>Top Item</DemoItem>
          <DemoItem>Bottom Item</DemoItem>
        </Col>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">End + End:</p>
        <Col
          mainAxisAlignment="end"
          crossAxisAlignment="end"
          className="h-48 border border-gray-300 rounded p-4"
        >
          <DemoItem className="w-1/2">Bottom Right</DemoItem>
        </Col>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Start + Stretch:</p>
        <Col
          mainAxisAlignment="start"
          crossAxisAlignment="stretch"
          className="h-48 border border-gray-300 rounded p-4"
        >
          <DemoItem>Full Width Item 1</DemoItem>
          <DemoItem>Full Width Item 2</DemoItem>
        </Col>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Examples combining different main and cross axis alignment props.',
      },
    },
  },
};

export const CustomGap: Story = {
  render: () => (
    <div className="space-y-6">
      <div>
        <p className="text-sm text-gray-500 mb-2">Default gap (gap-2.5):</p>
        <Col className="border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Col>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Custom gap (gap-6):</p>
        <Col className="gap-6 border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Col>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">No gap (gap-0):</p>
        <Col className="gap-0 border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Col>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Override the default gap-2.5 spacing using the className prop.',
      },
    },
  },
};

export const ProductCardExample: Story = {
  render: () => (
    <Col className="max-w-md border border-gray-300 rounded-lg p-6 gap-5">
      <Col className="gap-2">
        <h3 className="text-xl font-bold">Premium Membership</h3>
        <p className="text-gray-600">Get access to all premium features</p>
      </Col>
      <Col className="gap-3 flex-1">
        <div className="flex items-center gap-2">
          <span className="text-primary">✓</span>
          <span>Unlimited projects</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-primary">✓</span>
          <span>Priority support</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-primary">✓</span>
          <span>Advanced analytics</span>
        </div>
      </Col>
      <Col className="gap-2">
        <p className="text-2xl font-bold">$29.99/mo</p>
        <button className="w-full bg-primary text-white py-2 rounded hover:opacity-90">
          Subscribe Now
        </button>
      </Col>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Product card using nested Col components with different gap values.',
      },
    },
  },
};

export const FormLayout: Story = {
  render: () => (
    <Col className="max-w-md">
      <div>
        <label className="text-sm font-medium block mb-1">Name</label>
        <input
          type="text"
          className="w-full px-3 py-2 border border-gray-300 rounded"
          placeholder="Enter your name"
        />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Email</label>
        <input
          type="email"
          className="w-full px-3 py-2 border border-gray-300 rounded"
          placeholder="Enter your email"
        />
      </div>
      <div>
        <label className="text-sm font-medium block mb-1">Message</label>
        <textarea
          className="w-full px-3 py-2 border border-gray-300 rounded"
          placeholder="Your message"
          rows={4}
        />
      </div>
      <button className="w-full bg-primary text-white py-2 rounded hover:opacity-90">
        Submit
      </button>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Common use case: Form layout with consistent spacing between fields.',
      },
    },
  },
};

export const WithFlexGrow: Story = {
  render: () => (
    <Col className="h-96 border border-gray-300 rounded p-4">
      <DemoItem>Header</DemoItem>
      <DemoItem className="flex-1 flex items-center justify-center">
        Main Content Area (flex-1)
      </DemoItem>
      <DemoItem>Footer</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Using flex-1 on middle item to create flexible layouts where content grows to fill available space.',
      },
    },
  },
};

export const NestedColumns: Story = {
  render: () => (
    <Col className="border border-gray-300 rounded p-4 gap-4">
      <DemoItem>Outer Item 1</DemoItem>
      <Col className="border-2 border-blue-300 rounded p-4 gap-1">
        <DemoItem className="bg-blue-100 border-blue-300">Nested Item 1</DemoItem>
        <DemoItem className="bg-blue-100 border-blue-300">Nested Item 2</DemoItem>
        <DemoItem className="bg-blue-100 border-blue-300">Nested Item 3</DemoItem>
      </Col>
      <DemoItem>Outer Item 2</DemoItem>
    </Col>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Nested Col components with different gap values for complex layouts.',
      },
    },
  },
};

export const AllAlignmentOptions: Story = {
  render: () => (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold mb-4">Main Axis (Vertical) Alignment</h3>
        <div className="grid grid-cols-4 gap-4">
          {(['start', 'center', 'end', 'between'] as const).map((align) => (
            <div key={align}>
              <p className="text-xs text-gray-500 mb-2 capitalize">{align}</p>
              <Col
                mainAxisAlignment={align}
                className="h-40 border border-gray-300 rounded p-2"
              >
                <DemoItem className="text-xs py-1">1</DemoItem>
                <DemoItem className="text-xs py-1">2</DemoItem>
                <DemoItem className="text-xs py-1">3</DemoItem>
              </Col>
            </div>
          ))}
        </div>
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-4">Cross Axis (Horizontal) Alignment</h3>
        <div className="grid grid-cols-4 gap-4">
          {(['start', 'center', 'end', 'stretch'] as const).map((align) => (
            <div key={align}>
              <p className="text-xs text-gray-500 mb-2 capitalize">{align}</p>
              <Col
                crossAxisAlignment={align}
                className="h-40 border border-gray-300 rounded p-2"
              >
                <DemoItem className="text-xs py-1 w-1/4">S</DemoItem>
                <DemoItem className="text-xs py-1 w-1/2">M</DemoItem>
                <DemoItem className="text-xs py-1 w-3/4">L</DemoItem>
              </Col>
            </div>
          ))}
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Visual reference showing all available alignment variant options.',
      },
    },
  },
};
