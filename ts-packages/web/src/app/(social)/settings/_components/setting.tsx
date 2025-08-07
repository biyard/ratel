'use client';
import React from 'react';
import { useSettingsContext } from '../providers.client';
import MyInfo from './tab/my-info';
import Setting from './tab/setting';
import { config } from '@/config';

export default function SettingPage() {
  const { activeTab, handleActiveTab, tabItems } = useSettingsContext();
  const renderTab = () => {
    switch (activeTab) {
      case 'info':
        return <MyInfo />;
      case 'setting':
        return <Setting />;
      default:
        return null;
    }
  };
  return (
    <div className="w-full max-w-desktop mx-auto text-white px-4 py-6">
      {config.experiment && (
        <div className="flex justify-around mb-5">
          {tabItems.map((tab) => (
            <button
              key={tab.key}
              onClick={() => handleActiveTab(tab.key)}
              className="flex flex-col items-center gap-2"
            >
              <span
                className={`font-bold text-sm/[20px] ${
                  activeTab === tab.key ? 'text-white' : 'text-neutral-400'
                }`}
              >
                {tab.label}
              </span>
              {activeTab === tab.key && (
                <span className="bg-yellow-400 w-5 h-0.5 rounded-full" />
              )}
            </button>
          ))}
        </div>
      )}

      {renderTab()}
    </div>
  );
}
