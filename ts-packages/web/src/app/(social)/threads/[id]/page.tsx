import { useThreadController } from './use-thread-controller';
import ThreadHeader from './_components/thread-header';
import ThreadComment from './_components/comment';
import ThreadPost from './_components/thread';

export default function ThreadPage() {
  const ctrl = useThreadController();

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
