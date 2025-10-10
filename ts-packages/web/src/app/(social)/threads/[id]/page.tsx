import { SSRHydration } from '@/lib/query-utils';
import { getQueryClient } from '@/providers/getQueryClient';
import Header from './_components/header';
import ThreadPost from './_components/thread';
import ThreadComment from './_components/comment';

// import { requestFeedByID } from '../../_hooks/feed';
import striptags from 'striptags';
import { Suspense } from 'react';
import { logger } from '@/lib/logger';
import { getOption as getFeedByIdOption } from '@/hooks/feeds/use-feed-by-id';
import { getPost } from '@/lib/api/ratel/posts.v3';
import { useParams } from 'react-router';
import { useThreadController } from './_components/use-thread-controller';

// export async function generateMetadata({
//   params,
// }: {
//   params: Promise<{ id: string }>;
// }): Promise<Metadata> {
//   const { id } = await params;
//
//   let title = 'Ratel Thread';
//   let description = 'Ratel Thread';
//   let image = '';
//   try {
//     const { post } = await getPost(id);
//     title = post.title;
//     description = striptags(post.html_contents);
//     if (post.urls.length > 0) {
//       image = post.urls[0];
//     }
//   } catch (error) {
//     logger.error(`Failed to generate metadata for post ${id}:`, error);
//   }
//
//   return {
//     title,
//     description,
//     openGraph: {
//       title,
//       description,
//       images: [
//         {
//           url: image,
//         },
//       ],
//     },
//   };
// }

export default function ThreadPage() {
  const ctrl = useThreadController();

  return (
    <>
      <div className="flex flex-col gap-6 w-full max-tablet:mr-[20px]">
        <Header postId={ctrl.postId} />
        <ThreadPost postId={ctrl.postId} />
        <ThreadComment ctrl={ctrl} />
      </div>
    </>
  );
}
