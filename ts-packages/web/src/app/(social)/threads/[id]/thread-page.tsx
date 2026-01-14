import { useThreadController } from './use-thread-controller';
import ThreadHeader from './_components/thread-header';
import ThreadComment from './_components/comment';
import ThreadPost from './_components/thread';
import { useEffect } from 'react';
import { useLocation } from 'react-router';

export default function ThreadPage() {
  const ctrl = useThreadController();
  const location = useLocation();

  useEffect(() => {
    if (location.hash === '#comments') {
      setTimeout(() => {
        const element = document.getElementById('comments');
        element?.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }, 100);
    }
  }, [location.hash]);

  return (
    <>
      <div className="flex flex-col gap-6 w-full max-tablet:mr-[20px]">
        <ThreadHeader {...ctrl} />
        <ThreadPost {...ctrl} />
        <ThreadComment {...ctrl} />
      </div>
    </>
  );
}
