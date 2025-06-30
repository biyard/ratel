import { route } from '@/route';
import Link from 'next/link';

export default function NotFound() {
  return (
    <div className="flex flex-col items-center w-full">
      <h1 className="text-4xl font-bold mb-4">Thread Not Found</h1>
      <p className="text-gray-600 mb-8">This thread does not exist.</p>
      <div className="flex gap-4">
        <Link href={route.home()} className="text-primary hover:underline">
          Go Home
        </Link>
      </div>
    </div>
  );
}
