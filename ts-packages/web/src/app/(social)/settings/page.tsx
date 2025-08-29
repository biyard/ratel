'use client';

import React from 'react';
import MyInfo from './_components/tab/my-info';
import MySettings from './_components/tab/my-settings';

export default function MyProfilePage() {
  return (
    <div>

      {/* tab active state indicator */}
      <div className="w-10 h-[2px] bg-primary rounded-md"></div>

      {/* Top tabs section */}
      <div className= "flex flex-row justify-between text-sm font-bold  text-neutral-400 w-[500px] mx-auto">
        <p>My Info</p>
        <p>My Inventory</p>
        <p>Settings</p>
      </div>

      {/* Tab content */}
        
    </div>
  );
}
