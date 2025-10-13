import { config } from '@/config';

export function StorybookPage() {
  return (
    <iframe
      className="w-full h-[calc(100vh-100px)]"
      src={config.storybookUrl}
    />
  );
}
