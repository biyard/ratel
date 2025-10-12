import type { Meta, StoryObj } from '@storybook/react';
import { useState, useEffect } from 'react';
import { Modal } from './modal';

// Ensure modal-root exists for Storybook
if (typeof document !== 'undefined' && !document.getElementById('modal-root')) {
  const modalRoot = document.createElement('div');
  modalRoot.id = 'modal-root';
  document.body.appendChild(modalRoot);
}

const meta = {
  title: 'Components/Modal',
  component: Modal,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    title: {
      control: 'text',
      description: 'Modal title',
    },
    isOpen: {
      control: 'boolean',
      description: 'Whether the modal is open',
    },
    onClose: {
      action: 'closed',
      description: 'Callback when modal is closed',
    },
  },
} satisfies Meta<typeof Modal>;

export default meta;
type Story = StoryObj<typeof meta>;

// Basic modal
export const Basic: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);

    return (
      <>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Open Modal
        </button>
        <Modal title="Basic Modal" isOpen={isOpen} onClose={() => setIsOpen(false)}>
          <p className="text-white">This is a basic modal with some content.</p>
        </Modal>
      </>
    );
  },
};

// With form
export const WithForm: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);
    const [name, setName] = useState('');
    const [email, setEmail] = useState('');

    const handleSubmit = () => {
      alert(`Submitted: ${name}, ${email}`);
      setIsOpen(false);
      setName('');
      setEmail('');
    };

    return (
      <>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Open Form Modal
        </button>
        <Modal title="Contact Form" isOpen={isOpen} onClose={() => setIsOpen(false)}>
          <div className="flex flex-col gap-4">
            <input
              type="text"
              placeholder="Name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="px-3 py-2 bg-[#2a2a2a] text-white border border-gray-600 rounded"
            />
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="px-3 py-2 bg-[#2a2a2a] text-white border border-gray-600 rounded"
            />
            <button
              onClick={handleSubmit}
              disabled={!name || !email}
              className={`px-4 py-2 rounded ${
                name && email
                  ? 'bg-blue-500 text-white hover:bg-blue-600'
                  : 'bg-gray-600 text-gray-400 cursor-not-allowed'
              }`}
            >
              Submit
            </button>
          </div>
        </Modal>
      </>
    );
  },
};

// Confirmation dialog
export const ConfirmationDialog: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);
    const [result, setResult] = useState('');

    const handleConfirm = () => {
      setResult('Confirmed!');
      setIsOpen(false);
      setTimeout(() => setResult(''), 2000);
    };

    const handleCancel = () => {
      setResult('Cancelled');
      setIsOpen(false);
      setTimeout(() => setResult(''), 2000);
    };

    return (
      <>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Delete Item
        </button>
        {result && (
          <div className="mt-2 text-sm text-gray-600">{result}</div>
        )}
        <Modal
          title="Confirm Deletion"
          isOpen={isOpen}
          onClose={handleCancel}
        >
          <div className="flex flex-col gap-4">
            <p className="text-white">
              Are you sure you want to delete this item? This action cannot be
              undone.
            </p>
            <div className="flex gap-3">
              <button
                onClick={handleCancel}
                className="flex-1 px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                className="flex-1 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
              >
                Delete
              </button>
            </div>
          </div>
        </Modal>
      </>
    );
  },
};

// With long content
export const WithLongContent: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);

    return (
      <>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Open Long Content Modal
        </button>
        <Modal
          title="Terms and Conditions"
          isOpen={isOpen}
          onClose={() => setIsOpen(false)}
        >
          <div className="flex flex-col gap-4 max-h-[60vh] overflow-y-auto text-white">
            <p>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua.
            </p>
            <p>
              Ut enim ad minim veniam, quis nostrud exercitation ullamco
              laboris nisi ut aliquip ex ea commodo consequat.
            </p>
            <p>
              Duis aute irure dolor in reprehenderit in voluptate velit esse
              cillum dolore eu fugiat nulla pariatur.
            </p>
            <p>
              Excepteur sint occaecat cupidatat non proident, sunt in culpa qui
              officia deserunt mollit anim id est laborum.
            </p>
            <p>
              Sed ut perspiciatis unde omnis iste natus error sit voluptatem
              accusantium doloremque laudantium.
            </p>
            <p>
              Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et
              quasi architecto beatae vitae dicta sunt explicabo.
            </p>
            <button
              onClick={() => setIsOpen(false)}
              className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              I Agree
            </button>
          </div>
        </Modal>
      </>
    );
  },
};

// Success message
export const SuccessMessage: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);

    return (
      <>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          Show Success
        </button>
        <Modal
          title="Success!"
          isOpen={isOpen}
          onClose={() => setIsOpen(false)}
        >
          <div className="flex flex-col items-center gap-4 py-4">
            <div className="w-16 h-16 bg-green-500 rounded-full flex items-center justify-center">
              <svg
                className="w-10 h-10 text-white"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <p className="text-white text-center">
              Your action has been completed successfully!
            </p>
            <button
              onClick={() => setIsOpen(false)}
              className="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600"
            >
              OK
            </button>
          </div>
        </Modal>
      </>
    );
  },
};

// Auto-open on mount
export const AutoOpen: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(true);

    useEffect(() => {
      setIsOpen(true);
    }, []);

    return (
      <>
        <p className="text-sm text-gray-600 mb-4">
          This modal opens automatically
        </p>
        <button
          onClick={() => setIsOpen(true)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Reopen Modal
        </button>
        <Modal
          title="Welcome!"
          isOpen={isOpen}
          onClose={() => setIsOpen(false)}
        >
          <div className="flex flex-col gap-4">
            <p className="text-white">
              Welcome to our application! This modal opened automatically when
              you loaded this story.
            </p>
            <button
              onClick={() => setIsOpen(false)}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              Get Started
            </button>
          </div>
        </Modal>
      </>
    );
  },
};

// Multiple modals
export const MultipleModals: Story = {
  render: () => {
    const [modal1Open, setModal1Open] = useState(false);
    const [modal2Open, setModal2Open] = useState(false);

    return (
      <>
        <div className="flex gap-3">
          <button
            onClick={() => setModal1Open(true)}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Open Modal 1
          </button>
          <button
            onClick={() => setModal2Open(true)}
            className="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600"
          >
            Open Modal 2
          </button>
        </div>
        <Modal
          title="First Modal"
          isOpen={modal1Open}
          onClose={() => setModal1Open(false)}
        >
          <p className="text-white">This is the first modal.</p>
        </Modal>
        <Modal
          title="Second Modal"
          isOpen={modal2Open}
          onClose={() => setModal2Open(false)}
        >
          <p className="text-white">This is the second modal.</p>
        </Modal>
      </>
    );
  },
};
