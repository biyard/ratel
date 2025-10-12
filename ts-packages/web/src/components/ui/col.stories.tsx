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
    className: {
      control: 'text',
      description: 'Additional CSS classes',
    },
  },
} satisfies Meta<typeof Col>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default column
export const Default: Story = {
  render: () => (
    <Col>
      <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
      <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
      <div className="bg-red-500 text-white p-4 rounded">Item 3</div>
    </Col>
  ),
};

// With different gap
export const CustomGap: Story = {
  render: () => (
    <Col className="gap-8">
      <div className="bg-blue-500 text-white p-4 rounded">Large Gap</div>
      <div className="bg-green-500 text-white p-4 rounded">Between</div>
      <div className="bg-red-500 text-white p-4 rounded">Items</div>
    </Col>
  ),
};

// With alignment
export const HorizontalAlignment: Story = {
  render: () => (
    <div className="flex flex-col gap-8">
      <div>
        <p className="text-sm mb-2 text-gray-600">Align Start (default)</p>
        <Col className="items-start">
          <div className="bg-blue-500 text-white p-4 rounded w-32">
            Item 1
          </div>
          <div className="bg-green-500 text-white p-4 rounded w-24">
            Item 2
          </div>
          <div className="bg-red-500 text-white p-4 rounded w-40">Item 3</div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Align Center</p>
        <Col className="items-center">
          <div className="bg-blue-500 text-white p-4 rounded w-32">
            Item 1
          </div>
          <div className="bg-green-500 text-white p-4 rounded w-24">
            Item 2
          </div>
          <div className="bg-red-500 text-white p-4 rounded w-40">Item 3</div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Align End</p>
        <Col className="items-end">
          <div className="bg-blue-500 text-white p-4 rounded w-32">
            Item 1
          </div>
          <div className="bg-green-500 text-white p-4 rounded w-24">
            Item 2
          </div>
          <div className="bg-red-500 text-white p-4 rounded w-40">Item 3</div>
        </Col>
      </div>
    </div>
  ),
};

// With justify content
export const VerticalAlignment: Story = {
  render: () => (
    <div className="flex flex-col gap-8">
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Start (default)</p>
        <Col className="justify-start h-64 border rounded-lg">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Center</p>
        <Col className="justify-center h-64 border rounded-lg">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify End</p>
        <Col className="justify-end h-64 border rounded-lg">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Between</p>
        <Col className="justify-between h-64 border rounded-lg">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Col>
      </div>
    </div>
  ),
};

// Practical use cases
export const UseCases: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div>
        <p className="text-sm mb-2 font-semibold">Form Layout</p>
        <Col className="max-w-md">
          <div>
            <label className="text-sm font-medium">Name</label>
            <input
              type="text"
              className="w-full mt-1 px-3 py-2 border rounded"
              placeholder="Enter your name"
            />
          </div>
          <div>
            <label className="text-sm font-medium">Email</label>
            <input
              type="email"
              className="w-full mt-1 px-3 py-2 border rounded"
              placeholder="Enter your email"
            />
          </div>
          <div>
            <label className="text-sm font-medium">Message</label>
            <textarea
              className="w-full mt-1 px-3 py-2 border rounded"
              placeholder="Your message"
              rows={3}
            />
          </div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 font-semibold">Card Stack</p>
        <Col className="max-w-md">
          <div className="border rounded-lg p-4">
            <h3 className="font-semibold">Card 1</h3>
            <p className="text-sm text-gray-600">Content for first card</p>
          </div>
          <div className="border rounded-lg p-4">
            <h3 className="font-semibold">Card 2</h3>
            <p className="text-sm text-gray-600">Content for second card</p>
          </div>
          <div className="border rounded-lg p-4">
            <h3 className="font-semibold">Card 3</h3>
            <p className="text-sm text-gray-600">Content for third card</p>
          </div>
        </Col>
      </div>
      <div>
        <p className="text-sm mb-2 font-semibold">List Items</p>
        <Col className="max-w-md gap-0">
          <div className="border-b p-3 hover:bg-gray-50">List Item 1</div>
          <div className="border-b p-3 hover:bg-gray-50">List Item 2</div>
          <div className="border-b p-3 hover:bg-gray-50">List Item 3</div>
          <div className="p-3 hover:bg-gray-50">List Item 4</div>
        </Col>
      </div>
    </div>
  ),
};
