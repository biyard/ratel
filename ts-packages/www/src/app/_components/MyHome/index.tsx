import MyHomeLayout from './layout';

export default function MyHome() {
  return (
    <MyHomeLayout>
      <div className="min-h-screen">
        <main className="p-8">
          <div className="text-white text-center">
            <h1 className="text-4xl font-bold mb-4">MyHome</h1>
            <p className="text-neutral-400">
              (logged in)Your political information platform
            </p>
            {/* Example usage: <LogoIcon className="w-8 h-8 mx-auto mt-4" /> */}
          </div>
        </main>
      </div>
    </MyHomeLayout>
  );
}
