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
    className: {
      control: 'text',
      description: 'Additional CSS classes',
    },
  },
} satisfies Meta<typeof Row>;

export default meta;
type Story = StoryObj<typeof meta>;

// Default row
export const Default: Story = {
  render: () => (
    <Row>
      <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
      <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
      <div className="bg-red-500 text-white p-4 rounded">Item 3</div>
    </Row>
  ),
};

// With different gap
export const CustomGap: Story = {
  render: () => (
    <Row className="gap-8">
      <div className="bg-blue-500 text-white p-4 rounded">Large Gap</div>
      <div className="bg-green-500 text-white p-4 rounded">Between</div>
      <div className="bg-red-500 text-white p-4 rounded">Items</div>
    </Row>
  ),
};

// With alignment
export const VerticalAlignment: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div>
        <p className="text-sm mb-2 text-gray-600">Align Start (default)</p>
        <Row className="items-start">
          <div className="bg-blue-500 text-white p-4 rounded h-20">Tall Item</div>
          <div className="bg-green-500 text-white p-4 rounded">Short</div>
          <div className="bg-red-500 text-white p-4 rounded">Items</div>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Align Center</p>
        <Row className="items-center">
          <div className="bg-blue-500 text-white p-4 rounded h-20">Tall Item</div>
          <div className="bg-green-500 text-white p-4 rounded">Short</div>
          <div className="bg-red-500 text-white p-4 rounded">Items</div>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Align End</p>
        <Row className="items-end">
          <div className="bg-blue-500 text-white p-4 rounded h-20">Tall Item</div>
          <div className="bg-green-500 text-white p-4 rounded">Short</div>
          <div className="bg-red-500 text-white p-4 rounded">Items</div>
        </Row>
      </div>
    </div>
  ),
};

// With justify content
export const HorizontalAlignment: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Start (default)</p>
        <Row className="justify-start">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Center</p>
        <Row className="justify-center">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify End</p>
        <Row className="justify-end">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 text-gray-600">Justify Between</p>
        <Row className="justify-between">
          <div className="bg-blue-500 text-white p-4 rounded">Item 1</div>
          <div className="bg-green-500 text-white p-4 rounded">Item 2</div>
        </Row>
      </div>
    </div>
  ),
};

// Practical use cases
export const UseCases: Story = {
  render: () => (
    <div className="flex flex-col gap-6">
      <div>
        <p className="text-sm mb-2 font-semibold">Form Actions</p>
        <Row className="justify-end">
          <button className="px-4 py-2 border rounded">Cancel</button>
          <button className="px-4 py-2 bg-blue-500 text-white rounded">
            Save
          </button>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 font-semibold">Navigation Items</p>
        <Row className="items-center">
          <a href="#" className="px-3 py-2 hover:bg-gray-100 rounded">
            Home
          </a>
          <a href="#" className="px-3 py-2 hover:bg-gray-100 rounded">
            About
          </a>
          <a href="#" className="px-3 py-2 hover:bg-gray-100 rounded">
            Contact
          </a>
        </Row>
      </div>
      <div>
        <p className="text-sm mb-2 font-semibold">Card Header</p>
        <Row className="items-center justify-between border rounded-lg p-4">
          <h3 className="font-semibold">Card Title</h3>
          <button className="text-sm text-blue-500">Edit</button>
        </Row>
      </div>
    </div>
  ),
};
