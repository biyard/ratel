export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="absolute w-screen h-screen left-0 top-0 bg-bg">
      {children}
    </div>
  );
}
