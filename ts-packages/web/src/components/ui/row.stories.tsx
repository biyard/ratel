import type { Meta, StoryObj } from '@storybook/react';
import { Row } from './row';

const meta = {
  title: 'UI/Row',
  component: Row,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    mainAxisAlignment: {
      control: 'select',
      options: ['start', 'center', 'end', 'between'],
      description: 'Horizontal alignment (justify-content in flex-row)',
    },
    crossAxisAlignment: {
      control: 'select',
      options: ['start', 'center', 'end', 'stretch'],
      description: 'Vertical alignment (align-items in flex-row)',
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
} satisfies Meta<typeof Row>;

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
    <Row className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Default row layout with gap-2.5 spacing between items.',
      },
    },
  },
};

export const MainAxisAlignmentStart: Story = {
  args: {
    mainAxisAlignment: 'start',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items aligned to the left (justify-start). This is the default behavior.',
      },
    },
  },
};

export const MainAxisAlignmentCenter: Story = {
  args: {
    mainAxisAlignment: 'center',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items horizontally centered in the container (justify-center).',
      },
    },
  },
};

export const MainAxisAlignmentEnd: Story = {
  args: {
    mainAxisAlignment: 'end',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items aligned to the right (justify-end).',
      },
    },
  },
};

export const MainAxisAlignmentBetween: Story = {
  args: {
    mainAxisAlignment: 'between',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem>Item 1</DemoItem>
      <DemoItem>Item 2</DemoItem>
      <DemoItem>Item 3</DemoItem>
    </Row>
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
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="h-16">Small</DemoItem>
      <DemoItem className="h-24">Medium</DemoItem>
      <DemoItem className="h-32">Large</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items vertically aligned to the top (items-start). This is the default behavior.',
      },
    },
  },
};

export const CrossAxisAlignmentCenter: Story = {
  args: {
    crossAxisAlignment: 'center',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="h-16">Small</DemoItem>
      <DemoItem className="h-24">Medium</DemoItem>
      <DemoItem className="h-32">Large</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items vertically centered (items-center).',
      },
    },
  },
};

export const CrossAxisAlignmentEnd: Story = {
  args: {
    crossAxisAlignment: 'end',
  },
  render: (args) => (
    <Row {...args} className="border border-gray-300 rounded p-4">
      <DemoItem className="h-16">Small</DemoItem>
      <DemoItem className="h-24">Medium</DemoItem>
      <DemoItem className="h-32">Large</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items vertically aligned to the bottom (items-end).',
      },
    },
  },
};

export const CrossAxisAlignmentStretch: Story = {
  args: {
    crossAxisAlignment: 'stretch',
  },
  render: (args) => (
    <Row {...args} className="h-32 border border-gray-300 rounded p-4">
      <DemoItem className="flex items-center">Stretched Item 1</DemoItem>
      <DemoItem className="flex items-center">Stretched Item 2</DemoItem>
      <DemoItem className="flex items-center">Stretched Item 3</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Items stretched to fill container height (items-stretch).',
      },
    },
  },
};

export const CombinedAlignment: Story = {
  render: () => (
    <div className="space-y-4">
      <div>
        <p className="text-sm text-gray-500 mb-2">Center + Center:</p>
        <Row
          mainAxisAlignment="center"
          crossAxisAlignment="center"
          className="h-32 border border-gray-300 rounded p-4"
        >
          <DemoItem>Perfectly Centered</DemoItem>
        </Row>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Between + Center:</p>
        <Row
          mainAxisAlignment="between"
          crossAxisAlignment="center"
          className="h-32 border border-gray-300 rounded p-4"
        >
          <DemoItem>Left Item</DemoItem>
          <DemoItem>Right Item</DemoItem>
        </Row>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">End + End:</p>
        <Row
          mainAxisAlignment="end"
          crossAxisAlignment="end"
          className="h-32 border border-gray-300 rounded p-4"
        >
          <DemoItem className="h-20">Bottom Right</DemoItem>
        </Row>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Start + Stretch:</p>
        <Row
          mainAxisAlignment="start"
          crossAxisAlignment="stretch"
          className="h-32 border border-gray-300 rounded p-4"
        >
          <DemoItem className="flex items-center">Full Height 1</DemoItem>
          <DemoItem className="flex items-center">Full Height 2</DemoItem>
        </Row>
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
        <Row className="border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Row>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Custom gap (gap-6):</p>
        <Row className="gap-6 border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Row>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">No gap (gap-0):</p>
        <Row className="gap-0 border border-gray-300 rounded p-4">
          <DemoItem>Item 1</DemoItem>
          <DemoItem>Item 2</DemoItem>
          <DemoItem>Item 3</DemoItem>
        </Row>
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

export const NavigationExample: Story = {
  render: () => (
    <Row
      mainAxisAlignment="between"
      crossAxisAlignment="center"
      className="border border-gray-300 rounded-lg p-4"
    >
      <div className="flex items-center gap-2">
        <div className="w-8 h-8 bg-primary rounded"></div>
        <span className="font-bold">Logo</span>
      </div>
      <Row className="gap-4">
        <a href="#" className="text-gray-700 hover:text-primary">Home</a>
        <a href="#" className="text-gray-700 hover:text-primary">About</a>
        <a href="#" className="text-gray-700 hover:text-primary">Contact</a>
      </Row>
      <button className="px-4 py-2 bg-primary text-white rounded">Sign In</button>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Navigation bar with logo, links, and action button.',
      },
    },
  },
};

export const CardHeaderExample: Story = {
  render: () => (
    <div className="max-w-md border border-gray-300 rounded-lg">
      <Row
        mainAxisAlignment="between"
        crossAxisAlignment="center"
        className="p-4 border-b border-gray-300"
      >
        <h3 className="text-lg font-bold">Card Title</h3>
        <Row className="gap-2">
          <button className="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50">
            Edit
          </button>
          <button className="px-3 py-1 text-sm bg-primary text-white rounded hover:opacity-90">
            Save
          </button>
        </Row>
      </Row>
      <div className="p-4">
        <p className="text-gray-600">Card content goes here...</p>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Card header with title and action buttons.',
      },
    },
  },
};

export const FormActionsExample: Story = {
  render: () => (
    <div className="max-w-md border border-gray-300 rounded-lg p-4">
      <div className="space-y-4 mb-6">
        <input
          type="text"
          placeholder="Enter name"
          className="w-full px-3 py-2 border border-gray-300 rounded"
        />
        <input
          type="email"
          placeholder="Enter email"
          className="w-full px-3 py-2 border border-gray-300 rounded"
        />
      </div>
      <Row mainAxisAlignment="end" className="gap-2">
        <button className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50">
          Cancel
        </button>
        <button className="px-4 py-2 bg-primary text-white rounded hover:opacity-90">
          Submit
        </button>
      </Row>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Real-world example: Form with action buttons aligned to the right.',
      },
    },
  },
};

export const WithFlexGrow: Story = {
  render: () => (
    <Row className="border border-gray-300 rounded p-4">
      <DemoItem>Left</DemoItem>
      <DemoItem className="flex-1 text-center">Main Content (flex-1)</DemoItem>
      <DemoItem>Right</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Using flex-1 on middle item to create flexible layouts where content grows to fill available space.',
      },
    },
  },
};

export const NestedRows: Story = {
  render: () => (
    <Row className="border border-gray-300 rounded p-4 gap-4">
      <DemoItem>Outer 1</DemoItem>
      <Row className="border-2 border-blue-300 rounded p-4 gap-1">
        <DemoItem className="bg-blue-100 border-blue-300">Inner 1</DemoItem>
        <DemoItem className="bg-blue-100 border-blue-300">Inner 2</DemoItem>
        <DemoItem className="bg-blue-100 border-blue-300">Inner 3</DemoItem>
      </Row>
      <DemoItem>Outer 2</DemoItem>
    </Row>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Nested Row components with different gap values for complex layouts.',
      },
    },
  },
};

export const AllAlignmentOptions: Story = {
  render: () => (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold mb-4">Main Axis (Horizontal) Alignment</h3>
        <div className="space-y-4">
          {(['start', 'center', 'end', 'between'] as const).map((align) => (
            <div key={align}>
              <p className="text-xs text-gray-500 mb-2 capitalize">{align}</p>
              <Row
                mainAxisAlignment={align}
                className="border border-gray-300 rounded p-2"
              >
                <DemoItem className="text-xs py-1">1</DemoItem>
                <DemoItem className="text-xs py-1">2</DemoItem>
                <DemoItem className="text-xs py-1">3</DemoItem>
              </Row>
            </div>
          ))}
        </div>
      </div>
      <div>
        <h3 className="text-lg font-semibold mb-4">Cross Axis (Vertical) Alignment</h3>
        <div className="space-y-4">
          {(['start', 'center', 'end', 'stretch'] as const).map((align) => (
            <div key={align}>
              <p className="text-xs text-gray-500 mb-2 capitalize">{align}</p>
              <Row
                crossAxisAlignment={align}
                className="h-24 border border-gray-300 rounded p-2"
              >
                <DemoItem className="text-xs py-1 h-8 flex items-center">S</DemoItem>
                <DemoItem className="text-xs py-1 h-12 flex items-center">M</DemoItem>
                <DemoItem className="text-xs py-1 h-16 flex items-center">L</DemoItem>
              </Row>
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
