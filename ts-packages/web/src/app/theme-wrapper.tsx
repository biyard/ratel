'use client';

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  return <div className={`bg-bg light:bg-light-bg`}>{children}</div>;
}
