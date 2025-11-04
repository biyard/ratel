import type { Meta, StoryObj } from '@storybook/react';
import Heading from './heading';

const meta = {
  title: 'UI/Heading',
  component: Heading,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: [
        'heading1',
        'heading2',
        'heading3',
        'heading4',
        'heading5',
        'heading6',
      ],
      description: 'The heading level (h1-h6) with corresponding semantic HTML elements',
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
} satisfies Meta<typeof Heading>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Heading1: Story = {
  args: {
    children: 'Main Page Title (H1)',
    variant: 'heading1',
  },
  parameters: {
    docs: {
      description: {
        story: 'H1 heading - Used for main page titles. Font: bold, Size: 3xl/4xl/5xl (responsive)',
      },
    },
  },
};

export const Heading2: Story = {
  args: {
    children: 'Section Heading (H2)',
    variant: 'heading2',
  },
  parameters: {
    docs: {
      description: {
        story: 'H2 heading - Used for major section headings. Font: bold, Size: 2xl/3xl/4xl (responsive)',
      },
    },
  },
};

export const Heading3: Story = {
  args: {
    children: 'Subsection Heading (H3)',
    variant: 'heading3',
  },
  parameters: {
    docs: {
      description: {
        story: 'H3 heading - Used for subsection headings. Font: semibold, Size: xl/2xl/3xl (responsive)',
      },
    },
  },
};

export const Heading4: Story = {
  args: {
    children: 'Minor Heading (H4)',
    variant: 'heading4',
  },
  parameters: {
    docs: {
      description: {
        story: 'H4 heading - Used for minor headings. Font: semibold, Size: lg/xl/2xl (responsive)',
      },
    },
  },
};

export const Heading5: Story = {
  args: {
    children: 'Small Heading (H5)',
    variant: 'heading5',
  },
  parameters: {
    docs: {
      description: {
        story: 'H5 heading - Used for small headings like card titles. Font: semibold, Size: base/lg/xl (responsive)',
      },
    },
  },
};

export const Heading6: Story = {
  args: {
    children: 'Tiny Heading (H6)',
    variant: 'heading6',
  },
  parameters: {
    docs: {
      description: {
        story: 'H6 heading - Used for tiny headings and labels. Font: semibold, Size: sm/base/lg (responsive)',
      },
    },
  },
};

export const LongHeading: Story = {
  args: {
    children:
      'This is a Very Long Heading That Demonstrates Text Wrapping Behavior Across Multiple Lines When the Content Exceeds the Container Width',
    variant: 'heading1',
  },
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates how headings handle long text with proper wrapping and line height.',
      },
    },
  },
};

export const WithCustomClassName: Story = {
  args: {
    children: 'Custom Styled Heading',
    variant: 'heading2',
    className: 'text-primary underline decoration-2',
  },
  parameters: {
    docs: {
      description: {
        story: 'You can override styles using the className prop for custom styling needs.',
      },
    },
  },
};

export const Responsive: Story = {
  render: () => (
    <div className="space-y-6">
      <div className="p-4 border rounded-lg">
        <p className="text-sm text-gray-500 mb-3">
          Mobile (sm): text-3xl → Tablet (md): text-4xl → Desktop (lg): text-5xl
        </p>
        <Heading variant="heading1">Responsive H1 Heading</Heading>
      </div>
      <div className="p-4 border rounded-lg">
        <p className="text-sm text-gray-500 mb-3">
          Mobile (sm): text-2xl → Tablet (md): text-3xl → Desktop (lg): text-4xl
        </p>
        <Heading variant="heading2">Responsive H2 Heading</Heading>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story:
          'All headings are fully responsive and adjust size based on screen width. Resize your browser to see the changes.',
      },
    },
  },
};

export const AllHeadingLevels: Story = {
  render: () => (
    <div className="space-y-6">
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 1 (H1) - Main Titles:</p>
        <Heading variant="heading1">Main Page Title</Heading>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 2 (H2) - Section Titles:</p>
        <Heading variant="heading2">Section Heading</Heading>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 3 (H3) - Subsections:</p>
        <Heading variant="heading3">Subsection Heading</Heading>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 4 (H4) - Minor Headings:</p>
        <Heading variant="heading4">Minor Heading</Heading>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 5 (H5) - Small Headings:</p>
        <Heading variant="heading5">Small Heading</Heading>
      </div>
      <div>
        <p className="text-sm text-gray-500 mb-2">Heading 6 (H6) - Tiny Headings:</p>
        <Heading variant="heading6">Tiny Heading</Heading>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story:
          'Visual comparison of all 6 heading levels showing the complete typographic hierarchy from H1 to H6.',
      },
    },
  },
};

export const SemanticHTML: Story = {
  render: () => (
    <div className="space-y-4 p-4 border rounded-lg">
      <div className="text-sm text-gray-500 mb-4">
        <strong>Semantic HTML:</strong> Each variant automatically renders the correct HTML element:
      </div>
      <div className="space-y-3 font-mono text-xs">
        <div className="flex items-center gap-3">
          <code className="bg-gray-100 px-2 py-1 rounded">variant="heading1"</code>
          <span>→</span>
          <Heading variant="heading1">&lt;h1&gt; Element</Heading>
        </div>
        <div className="flex items-center gap-3">
          <code className="bg-gray-100 px-2 py-1 rounded">variant="heading2"</code>
          <span>→</span>
          <Heading variant="heading2">&lt;h2&gt; Element</Heading>
        </div>
        <div className="flex items-center gap-3">
          <code className="bg-gray-100 px-2 py-1 rounded">variant="heading3"</code>
          <span>→</span>
          <Heading variant="heading3">&lt;h3&gt; Element</Heading>
        </div>
      </div>
      <p className="text-sm text-gray-600 mt-4">
        This ensures proper document structure and improves accessibility.
      </p>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story:
          'The component automatically renders the semantically correct HTML element based on the variant prop.',
      },
    },
  },
};

export const UsageExample: Story = {
  render: () => (
    <div className="max-w-2xl space-y-6">
      <Heading variant="heading1">Article Title</Heading>
      <p className="text-gray-600">
        This is the main article content. Notice how the heading hierarchy creates a
        clear visual structure.
      </p>
      <Heading variant="heading2">First Section</Heading>
      <p className="text-gray-600">
        Content for the first section goes here. H2 headings are used for major sections.
      </p>
      <Heading variant="heading3">Subsection 1.1</Heading>
      <p className="text-gray-600">
        Subsection content. H3 headings divide sections into smaller parts.
      </p>
      <Heading variant="heading3">Subsection 1.2</Heading>
      <p className="text-gray-600">More subsection content with proper hierarchy.</p>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story:
          'Real-world example showing how different heading levels work together to create a clear content hierarchy.',
      },
    },
  },
};
