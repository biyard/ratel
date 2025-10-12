import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import SwitchButton from './index';

const meta = {
  title: 'Components/SwitchButton',
  component: SwitchButton,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    value: {
      control: 'boolean',
      description: 'Whether the switch is on or off',
    },
    color: {
      control: 'color',
      description: 'Color when the switch is on',
    },
    onChange: {
      action: 'changed',
      description: 'Callback when the switch value changes',
    },
  },
} satisfies Meta<typeof SwitchButton>;

export default meta;
type Story = StoryObj<typeof meta>;

// On state
export const On: Story = {
  args: {
    value: true,
    onChange: () => {},
    color: 'bg-[#fcb300]',
  },
};

// Off state
export const Off: Story = {
  args: {
    value: false,
    onChange: () => {},
    color: 'bg-[#fcb300]',
  },
};

// Interactive
export const Interactive: Story = {
  render: () => {
    const [isOn, setIsOn] = useState(false);
    return (
      <div className="flex items-center gap-3">
        <SwitchButton
          value={isOn}
          onChange={setIsOn}
          color="bg-[#fcb300]"
        />
        <span className="text-sm">{isOn ? 'On' : 'Off'}</span>
      </div>
    );
  },
};

// Different colors
export const DifferentColors: Story = {
  render: () => {
    const [blue, setBlue] = useState(true);
    const [green, setGreen] = useState(true);
    const [red, setRed] = useState(true);
    const [purple, setPurple] = useState(true);
    const [yellow, setYellow] = useState(true);

    return (
      <div className="flex flex-col gap-4">
        <div className="flex items-center gap-3">
          <SwitchButton value={blue} onChange={setBlue} color="bg-blue-500" />
          <span className="text-sm">Blue</span>
        </div>
        <div className="flex items-center gap-3">
          <SwitchButton
            value={green}
            onChange={setGreen}
            color="bg-green-500"
          />
          <span className="text-sm">Green</span>
        </div>
        <div className="flex items-center gap-3">
          <SwitchButton value={red} onChange={setRed} color="bg-red-500" />
          <span className="text-sm">Red</span>
        </div>
        <div className="flex items-center gap-3">
          <SwitchButton
            value={purple}
            onChange={setPurple}
            color="bg-purple-500"
          />
          <span className="text-sm">Purple</span>
        </div>
        <div className="flex items-center gap-3">
          <SwitchButton
            value={yellow}
            onChange={setYellow}
            color="bg-[#fcb300]"
          />
          <span className="text-sm">Yellow (Default)</span>
        </div>
      </div>
    );
  },
};

// Settings panel example
export const SettingsPanel: Story = {
  render: () => {
    const [notifications, setNotifications] = useState(true);
    const [darkMode, setDarkMode] = useState(false);
    const [autoSave, setAutoSave] = useState(true);
    const [analytics, setAnalytics] = useState(false);

    return (
      <div className="max-w-md border rounded-lg p-6">
        <h3 className="font-semibold text-lg mb-4">Settings</h3>
        <div className="flex flex-col gap-4">
          <div className="flex items-center justify-between">
            <div>
              <div className="font-medium text-sm">Notifications</div>
              <div className="text-xs text-gray-600">
                Receive push notifications
              </div>
            </div>
            <SwitchButton
              value={notifications}
              onChange={setNotifications}
              color="bg-[#fcb300]"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="font-medium text-sm">Dark Mode</div>
              <div className="text-xs text-gray-600">
                Use dark theme throughout the app
              </div>
            </div>
            <SwitchButton
              value={darkMode}
              onChange={setDarkMode}
              color="bg-[#fcb300]"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="font-medium text-sm">Auto Save</div>
              <div className="text-xs text-gray-600">
                Automatically save changes
              </div>
            </div>
            <SwitchButton
              value={autoSave}
              onChange={setAutoSave}
              color="bg-[#fcb300]"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="font-medium text-sm">Analytics</div>
              <div className="text-xs text-gray-600">
                Share usage data to improve the app
              </div>
            </div>
            <SwitchButton
              value={analytics}
              onChange={setAnalytics}
              color="bg-[#fcb300]"
            />
          </div>
        </div>
      </div>
    );
  },
};

// Grouped switches
export const GroupedSwitches: Story = {
  render: () => {
    const [emailNotif, setEmailNotif] = useState(true);
    const [pushNotif, setPushNotif] = useState(false);
    const [smsNotif, setSmsNotif] = useState(false);

    return (
      <div className="max-w-md border rounded-lg">
        <div className="p-4 border-b">
          <h3 className="font-semibold text-sm">Notification Preferences</h3>
        </div>
        <div className="divide-y">
          <div className="flex items-center justify-between p-4">
            <span className="text-sm">Email Notifications</span>
            <SwitchButton
              value={emailNotif}
              onChange={setEmailNotif}
              color="bg-green-500"
            />
          </div>
          <div className="flex items-center justify-between p-4">
            <span className="text-sm">Push Notifications</span>
            <SwitchButton
              value={pushNotif}
              onChange={setPushNotif}
              color="bg-green-500"
            />
          </div>
          <div className="flex items-center justify-between p-4">
            <span className="text-sm">SMS Notifications</span>
            <SwitchButton
              value={smsNotif}
              onChange={setSmsNotif}
              color="bg-green-500"
            />
          </div>
        </div>
      </div>
    );
  },
};

// Form example
export const FormExample: Story = {
  render: () => {
    const [agreeToTerms, setAgreeToTerms] = useState(false);
    const [subscribeNewsletter, setSubscribeNewsletter] = useState(true);
    const [makePublic, setMakePublic] = useState(false);

    return (
      <form className="max-w-md flex flex-col gap-6">
        <div>
          <h3 className="font-semibold mb-4">Create Account</h3>
          <input
            type="text"
            placeholder="Username"
            className="w-full px-3 py-2 border rounded mb-3"
          />
          <input
            type="email"
            placeholder="Email"
            className="w-full px-3 py-2 border rounded mb-3"
          />
          <input
            type="password"
            placeholder="Password"
            className="w-full px-3 py-2 border rounded"
          />
        </div>

        <div className="flex flex-col gap-4">
          <div className="flex items-center justify-between">
            <span className="text-sm">Agree to Terms & Conditions</span>
            <SwitchButton
              value={agreeToTerms}
              onChange={setAgreeToTerms}
              color="bg-[#fcb300]"
            />
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm">Subscribe to Newsletter</span>
            <SwitchButton
              value={subscribeNewsletter}
              onChange={setSubscribeNewsletter}
              color="bg-[#fcb300]"
            />
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm">Make Profile Public</span>
            <SwitchButton
              value={makePublic}
              onChange={setMakePublic}
              color="bg-[#fcb300]"
            />
          </div>
        </div>

        <button
          type="submit"
          disabled={!agreeToTerms}
          className={`py-2 px-4 rounded ${
            agreeToTerms
              ? 'bg-blue-500 text-white hover:bg-blue-600'
              : 'bg-gray-300 text-gray-500 cursor-not-allowed'
          }`}
          onClick={(e) => e.preventDefault()}
        >
          Create Account
        </button>
      </form>
    );
  },
};
