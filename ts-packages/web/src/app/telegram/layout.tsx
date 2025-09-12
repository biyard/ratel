import Provider from './providers';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <Provider>
      <div className="absolute w-screen h-screen left-0 top-0 bg-black">
        {children}
      </div>
    </Provider>
  );
}
