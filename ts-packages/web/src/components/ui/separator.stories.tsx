import type { Meta, StoryObj } from '@storybook/react';
import { Separator } from './separator';

const meta = {
  title: 'UI/Separator',
  component: Separator,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    orientation: {
      control: 'select',
      options: ['horizontal', 'vertical'],
      description: 'The orientation of the separator',
    },
    decorative: {
      control: 'boolean',
      description: 'Whether the separator is purely decorative',
    },
  },
} satisfies Meta<typeof Separator>;

export default meta;
type Story = StoryObj<typeof meta>;

// Horizontal separator (default)
export const Horizontal: Story = {
  render: () => (
    <div className="w-80">
      <div className="space-y-4">
        <h4 className="text-sm font-medium">Section Title</h4>
        <Separator />
        <p className="text-sm text-gray-600">
          This is some content below the separator.
        </p>
      </div>
    </div>
  ),
};

// Vertical separator
export const Vertical: Story = {
  render: () => (
    <div className="flex h-20 items-center gap-4">
      <span>Item 1</span>
      <Separator orientation="vertical" />
      <span>Item 2</span>
      <Separator orientation="vertical" />
      <span>Item 3</span>
    </div>
  ),
};

// In a text list
export const InTextList: Story = {
  render: () => (
    <div className="w-80 space-y-1">
      <h4 className="text-sm font-medium mb-2">Navigation</h4>
      <a href="#" className="block px-2 py-1 text-sm hover:bg-gray-100 rounded">
        Home
      </a>
      <a href="#" className="block px-2 py-1 text-sm hover:bg-gray-100 rounded">
        About
      </a>
      <Separator className="my-2" />
      <a href="#" className="block px-2 py-1 text-sm hover:bg-gray-100 rounded">
        Settings
      </a>
      <a href="#" className="block px-2 py-1 text-sm hover:bg-gray-100 rounded">
        Logout
      </a>
    </div>
  ),
};

// In a card
export const InCard: Story = {
  render: () => (
    <div className="w-80 border rounded-lg overflow-hidden">
      <div className="p-4">
        <h3 className="font-semibold">Card Header</h3>
        <p className="text-sm text-gray-600">This is the header section</p>
      </div>
      <Separator />
      <div className="p-4">
        <p className="text-sm">This is the body section with some content.</p>
      </div>
      <Separator />
      <div className="p-4">
        <button className="text-sm text-blue-500">Action Button</button>
      </div>
    </div>
  ),
};

// With text sections
export const WithTextSections: Story = {
  render: () => (
    <div className="w-96 space-y-4">
      <div>
        <h3 className="text-lg font-semibold mb-2">Introduction</h3>
        <p className="text-sm text-gray-600">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua.
        </p>
      </div>
      <Separator />
      <div>
        <h3 className="text-lg font-semibold mb-2">Features</h3>
        <p className="text-sm text-gray-600">
          Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
          nisi ut aliquip ex ea commodo consequat.
        </p>
      </div>
      <Separator />
      <div>
        <h3 className="text-lg font-semibold mb-2">Conclusion</h3>
        <p className="text-sm text-gray-600">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum
          dolore eu fugiat nulla pariatur.
        </p>
      </div>
    </div>
  ),
};

// In a toolbar
export const InToolbar: Story = {
  render: () => (
    <div className="flex items-center gap-2 p-2 border rounded-lg w-fit">
      <button className="p-2 hover:bg-gray-100 rounded">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="w-5 h-5"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M3 4.5h14.25M3 9h9.75M3 13.5h5.25m5.25-.75L17.25 9m0 0L21 12.75M17.25 9v12"
          />
        </svg>
      </button>
      <button className="p-2 hover:bg-gray-100 rounded">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="w-5 h-5"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
          />
        </svg>
      </button>
      <Separator orientation="vertical" className="h-6" />
      <button className="p-2 hover:bg-gray-100 rounded">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="w-5 h-5"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0 0 21 18V6a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v12a2.25 2.25 0 0 0 2.25 2.25Z"
          />
        </svg>
      </button>
      <button className="p-2 hover:bg-gray-100 rounded">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="w-5 h-5"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
          />
        </svg>
      </button>
    </div>
  ),
};

// In a breadcrumb
export const InBreadcrumb: Story = {
  render: () => (
    <nav className="flex items-center text-sm">
      <a href="#" className="text-blue-500 hover:underline">
        Home
      </a>
      <Separator orientation="vertical" className="mx-2 h-4" />
      <a href="#" className="text-blue-500 hover:underline">
        Products
      </a>
      <Separator orientation="vertical" className="mx-2 h-4" />
      <a href="#" className="text-blue-500 hover:underline">
        Electronics
      </a>
      <Separator orientation="vertical" className="mx-2 h-4" />
      <span className="text-gray-600">Laptop</span>
    </nav>
  ),
};

// Custom styling
export const CustomStyling: Story = {
  render: () => (
    <div className="w-80 space-y-8">
      <div className="space-y-2">
        <p className="text-sm font-medium">Thick separator</p>
        <Separator className="h-1 bg-blue-500" />
      </div>
      <div className="space-y-2">
        <p className="text-sm font-medium">Dashed separator</p>
        <Separator className="border-t border-dashed" />
      </div>
      <div className="space-y-2">
        <p className="text-sm font-medium">Gradient separator</p>
        <Separator className="h-0.5 bg-gradient-to-r from-transparent via-gray-400 to-transparent" />
      </div>
    </div>
  ),
};

// Both orientations
export const BothOrientations: Story = {
  render: () => (
    <div className="space-y-8">
      <div>
        <p className="text-sm font-medium mb-4">Horizontal Separator</p>
        <div className="w-80 space-y-2">
          <p className="text-sm">Content above</p>
          <Separator />
          <p className="text-sm">Content below</p>
        </div>
      </div>
      <div>
        <p className="text-sm font-medium mb-4">Vertical Separator</p>
        <div className="flex h-20 items-center gap-4">
          <div className="text-sm">Left content</div>
          <Separator orientation="vertical" />
          <div className="text-sm">Right content</div>
        </div>
      </div>
    </div>
  ),
};
