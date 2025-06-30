import React from 'react';

export default function MyHomeLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-row gap-20">
      <div id="myhome-sidemenu" className="flex-1">
        <h1 className="text-2xl font-bold">My Home Layout</h1>
        <p>This is the home layout for my application.</p>
      </div>

      <div className="flex-1">{children}</div>

      <div id="myhome-infomenu" className="flex-1">
        <p>This is the right sidebar content.</p>
      </div>
    </div>
  );
}
