import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import Scrollable from './index';

const meta: Meta<typeof Scrollable> = {
  title: 'Components/Scrollable',
  component: Scrollable,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;
type Story = StoryObj<typeof Scrollable>;

// Basic infinite scroll example
export const Basic: Story = {
  render: () => {
    const [items, setItems] = useState(
      Array.from({ length: 20 }, (_, i) => i + 1),
    );
    const [loading, setLoading] = useState(false);

    const loadMore = () => {
      if (loading || items.length >= 40) return;

      setLoading(true);
      console.log('Loading more items...');
      setTimeout(() => {
        setItems((prev) => [
          ...prev,
          ...Array.from({ length: 10 }, (_, i) => prev.length + i + 1),
        ]);
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-md h-[600px] overflow-auto border border-gray-300 rounded bg-white shadow-lg"
        >
          <div className="p-4">
            <h2 className="text-xl font-bold mb-4">Infinite Scroll Demo</h2>
            {items.map((item) => (
              <div
                key={item}
                className="p-4 mb-2 bg-blue-100 rounded hover:bg-blue-200 transition-colors"
              >
                Item {item}
              </div>
            ))}
            {loading && (
              <div className="p-4 text-center text-gray-500">
                Loading more...
              </div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};

// With trigger counter
export const WithTriggerCounter: Story = {
  render: () => {
    const [items, setItems] = useState(
      Array.from({ length: 15 }, (_, i) => i + 1),
    );
    const [loading, setLoading] = useState(false);
    const [triggerCount, setTriggerCount] = useState(0);

    const loadMore = () => {
      if (loading) return;

      setLoading(true);
      setTriggerCount((prev) => prev + 1);
      console.log('onReachBottom triggered:', triggerCount + 1);

      setTimeout(() => {
        setItems((prev) => [
          ...prev,
          ...Array.from({ length: 10 }, (_, i) => prev.length + i + 1),
        ]);
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-md h-[600px] overflow-auto border border-gray-300 rounded bg-white shadow-lg"
        >
          <div className="p-4">
            <div className="mb-4 p-3 bg-purple-100 rounded-lg">
              <div className="text-sm font-semibold text-purple-900">
                Trigger Count: {triggerCount}
              </div>
              <div className="text-xs text-purple-700 mt-1">
                Scroll to bottom to load more
              </div>
            </div>
            {items.map((item) => (
              <div
                key={item}
                className="p-4 mb-2 bg-purple-50 rounded hover:bg-purple-100 transition-colors"
              >
                Item {item}
              </div>
            ))}
            {loading && (
              <div className="p-4 text-center text-gray-500">
                Loading more...
              </div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};

// Empty state
export const EmptyState: Story = {
  render: () => {
    const [items, setItems] = useState<number[]>([]);
    const [loading, setLoading] = useState(false);

    const loadMore = () => {
      if (loading || items.length > 0) return;

      setLoading(true);
      setTimeout(() => {
        setItems(Array.from({ length: 10 }, (_, i) => i + 1));
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-md h-[600px] overflow-auto border border-gray-300 rounded bg-white shadow-lg"
        >
          <div className="p-4">
            <h2 className="text-xl font-bold mb-4">Empty State Demo</h2>
            {items.length === 0 && !loading && (
              <div className="p-8 text-center text-gray-400">
                <div className="text-6xl mb-4">📭</div>
                <div className="text-lg font-medium">No items yet</div>
                <div className="text-sm mt-2">
                  Scroll down to load initial items
                </div>
              </div>
            )}
            {items.map((item) => (
              <div
                key={item}
                className="p-4 mb-2 bg-blue-100 rounded hover:bg-blue-200 transition-colors"
              >
                Item {item}
              </div>
            ))}
            {loading && (
              <div className="p-4 text-center text-gray-500">Loading...</div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};

// Grid layout example
export const GridLayout: Story = {
  render: () => {
    const [items, setItems] = useState(
      Array.from({ length: 30 }, (_, i) => i + 1),
    );
    const [loading, setLoading] = useState(false);

    const loadMore = () => {
      if (loading) return;

      setLoading(true);
      setTimeout(() => {
        setItems((prev) => [
          ...prev,
          ...Array.from({ length: 15 }, (_, i) => prev.length + i + 1),
        ]);
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-3xl h-[600px] overflow-auto border border-gray-300 rounded bg-white shadow-lg"
        >
          <div className="p-4">
            <h2 className="text-xl font-bold mb-4">Grid Layout Demo</h2>
            <div className="grid grid-cols-3 gap-4">
              {items.map((item) => (
                <div
                  key={item}
                  className="aspect-square bg-indigo-100 rounded flex items-center justify-center hover:bg-indigo-200 transition-colors font-bold text-lg"
                >
                  {item}
                </div>
              ))}
            </div>
            {loading && (
              <div className="p-4 text-center text-gray-500 mt-4">
                Loading more...
              </div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};

// With custom styling
export const WithCustomStyling: Story = {
  render: () => {
    const [items, setItems] = useState(
      Array.from({ length: 15 }, (_, i) => i + 1),
    );
    const [loading, setLoading] = useState(false);

    const loadMore = () => {
      if (loading) return;

      setLoading(true);
      setTimeout(() => {
        setItems((prev) => [
          ...prev,
          ...Array.from({ length: 10 }, (_, i) => prev.length + i + 1),
        ]);
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-linear-to-br from-pink-100 to-purple-100 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-md h-[600px] overflow-auto border-2 border-pink-300 rounded-lg bg-linear-to-b from-pink-50 to-purple-50 shadow-2xl"
        >
          <div className="p-4">
            <h2 className="text-xl font-bold mb-4 text-purple-900">
              Custom Styled Demo
            </h2>
            {items.map((item) => (
              <div
                key={item}
                className="p-4 mb-2 bg-white/80 backdrop-blur rounded-lg shadow-sm hover:shadow-md transition-all"
              >
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-linear-to-br from-pink-400 to-purple-400 rounded-full flex items-center justify-center text-white font-bold">
                    {item}
                  </div>
                  <div>
                    <div className="font-semibold">Item {item}</div>
                    <div className="text-sm text-gray-500">
                      Custom styled card
                    </div>
                  </div>
                </div>
              </div>
            ))}
            {loading && (
              <div className="p-4 text-center text-purple-600 font-medium">
                Loading more items...
              </div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};

// Long content items
export const LongContentItems: Story = {
  render: () => {
    const [items, setItems] = useState(
      Array.from({ length: 10 }, (_, i) => i + 1),
    );
    const [loading, setLoading] = useState(false);

    const loadMore = () => {
      if (loading) return;

      setLoading(true);
      setTimeout(() => {
        setItems((prev) => [
          ...prev,
          ...Array.from({ length: 5 }, (_, i) => prev.length + i + 1),
        ]);
        setLoading(false);
      }, 1000);
    };

    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-50 p-8">
        <Scrollable
          onReachBottom={loadMore}
          className="w-full max-w-2xl h-[600px] overflow-auto border border-gray-300 rounded bg-white shadow-lg"
        >
          <div className="p-6">
            <h2 className="text-2xl font-bold mb-6">Article List Demo</h2>
            {items.map((item) => (
              <article
                key={item}
                className="p-6 mb-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow"
              >
                <h3 className="text-lg font-bold mb-2">Article Title {item}</h3>
                <p className="text-gray-600 mb-3">
                  Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed
                  do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                  Ut enim ad minim veniam, quis nostrud exercitation ullamco
                  laboris.
                </p>
                <div className="flex gap-2 text-sm text-gray-500">
                  <span>👤 Author Name</span>
                  <span>•</span>
                  <span>📅 Jan {item}, 2024</span>
                  <span>•</span>
                  <span>⏱️ 5 min read</span>
                </div>
              </article>
            ))}
            {loading && (
              <div className="p-4 text-center text-gray-500">
                Loading more articles...
              </div>
            )}
          </div>
        </Scrollable>
      </div>
    );
  },
};
