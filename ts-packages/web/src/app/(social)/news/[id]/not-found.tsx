import { route } from '@/route';
import { NavLink } from 'react-router';

export default function NotFound() {
  return (
    <div className="flex flex-col items-center w-full">
      <h1 className="text-4xl font-bold mb-4">News Not Found</h1>
      <p className="text-gray-600 mb-8">This news does not exist.</p>
      <div className="flex gap-4">
        <NavLink to={route.home()} className="text-primary hover:underline">
          Go Home
        </NavLink>
      </div>
    </div>
  );
}
