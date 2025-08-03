// 'use client';
// import React, { useState, useEffect } from 'react';
// import { Col } from './ui/col';
// import { Row } from './ui/row';
// import { CommentIcon, Rewards, Shares, ThumbUp } from './icons';
// import { convertNumberToString } from '@/lib/number-utils';
// import TimeAgo from './time-ago';
// import DOMPurify from 'dompurify';
// import { Button } from './ui/button';
// import { useRouter } from 'next/navigation';
// import { useApiCall } from '@/lib/api/use-send';
// import { ratelApi } from '@/lib/api/ratel_api';
// import { UserType } from '@/lib/api/models/user';
// import Image from 'next/image';
// import { route } from '@/route';
// import { SpaceType } from '@/lib/api/models/spaces';
// import {
//   DropdownMenuContent,
//   DropdownMenuTrigger,
//   DropdownMenu,
//   DropdownMenuItem,
// } from './ui/dropdown-menu';
// import { Edit1 } from './icons';
// import { useRepostDraft } from '@/app/(social)/_components/create-repost2';

// export interface FeedCardProps {
//   id: number;
//   industry: string;
//   title: string;
//   contents: string;
//   author_profile_url: string;
//   author_name: string;
//   author_type: UserType;
//   url?: string;
//   created_at: number;

//   likes: number;
//   is_liked: boolean;
//   comments: number;
//   rewards: number;
//   shares: number;

//   space_id?: number;
//   space_type?: SpaceType;
//   author_id: number;
//   user_id: number;
//   onboard: boolean;
//   onRepostThought?: () => void;
//   onRepost?: () => void;

//   onLikeClick?: (value: boolean) => void;
//   refetch?: () => void;
//   isLikeProcessing?: boolean;
// }

// export default function FeedCard(props: FeedCardProps) {
//   const router = useRouter();
//   const { post } = useApiCall();

//   const [localLikes, setLocalLikes] = useState(props.likes);
//   const [localIsLiked, setLocalIsLiked] = useState(props.is_liked);
//   const [isProcessing, setIsProcessing] = useState(false);

//   const [showRepostModal, setShowRepostModal] = useState(false);

//   const { newDraft, setFeedContent } = useRepostDraft();

//   // Sync with props when they change
//   useEffect(() => {
//     setLocalLikes(props.likes);
//     setLocalIsLiked(props.is_liked);
//   }, [props.likes, props.is_liked]);

//   const handleLike = async (value: boolean) => {
//     if (isProcessing) return; // Prevent multiple clicks

//     // Set processing state and optimistic update
//     setIsProcessing(true);
//     setLocalIsLiked(value);
//     setLocalLikes((prev) => (value ? prev + 1 : prev - 1));

//     try {
//       await post(ratelApi.feeds.likePost(props.id), {
//         like: { value },
//       });

//       // Success - trigger callbacks
//       props.onLikeClick?.(value);
//       props.refetch?.();
//     } catch (error) {
//       // Revert optimistic update on error
//       setLocalIsLiked(props.is_liked);
//       setLocalLikes(props.likes);
//       console.error('Failed to update like:', error);
//     } finally {
//       setIsProcessing(false);
//     }
//   };

//   // repost logic

//   const handleRepost = async () => {
//     try {
//       // await post(ratelApi.feeds.repost(props.id), {
//       //   parent_id: props.id,
//       //   user_id: props.user_id,
//       //   html_contents: props.contents,
//       //   quote_feed_id: null,
//       // });
//       // props.refetch?.();
//     } catch (error) {}
//   };

//   const handleRepostThought = () => {
//     newDraft();
//     setFeedContent(props.contents); // Add this line
//   };

//   return (
//     <Col
//       className="cursor-pointer bg-component-bg rounded-[10px]"
//       onClick={() => {
//         router.push(route.threadByFeedId(props.id));
//       }}
//     >
//       <FeedBody {...props} />

//       <FeedFooter
//         {...props}
//         likes={localLikes}
//         is_liked={localIsLiked}
//         isLikeProcessing={isProcessing}
//         onRepostThought={handleRepostThought}
//         onRepost={handleRepost}
//       />
//     </Col>
//   );
// }

// export function FeedBody({
//   industry,
//   title,
//   contents,
//   author_name,
//   author_profile_url,
//   url,
//   created_at,
//   author_type,
//   // user_id,
//   // author_id,
//   space_type,
//   space_id,
//   onboard,
// }: FeedCardProps) {
//   const router = useRouter();
//   return (
//     <Col className="pt-5 pb-2.5">
//       <Row className="justify-between px-5">
//         <Row>
//           <IndustryTag industry={industry} />
//           {onboard && <OnboradingTag />}
//         </Row>
//         {/* {user_id === author_id && !space_id && (
//           <Button
//             variant="rounded_primary"
//             className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
//           >
//             Create a Space
//           </Button>
//         )} */}

//         {space_id && space_type ? (
//           <Button
//             variant="rounded_primary"
//             className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
//             onClick={(e) => {
//               e.stopPropagation();
//               if (space_type === SpaceType.Committee) {
//                 router.push(route.commiteeSpaceById(space_id));
//               } else {
//                 router.push(route.deliberationSpaceById(space_id));
//               }
//             }}
//           >
//             Join
//           </Button>
//         ) : (
//           <div />
//         )}
//       </Row>
//       <h2 className="w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-white px-5">
//         {title}
//       </h2>
//       <Row className="justify-between items-center px-5">
//         <UserBadge
//           profile_url={author_profile_url}
//           name={author_name}
//           author_type={author_type}
//         />
//         <TimeAgo timestamp={created_at} />
//       </Row>
//       <Row className="justify-between px-5"></Row>
//       <FeedContents contents={contents} url={url} />
//     </Col>
//   );
// }

// export function FeedContents({
//   contents,
//   url,
// }: {
//   contents: string;
//   url?: string;
// }) {
//   const html =
//     typeof window !== 'undefined' ? DOMPurify.sanitize(contents) : contents;

//   return (
//     <Col className="text-white">
//       <div
//         className="feed-content font-normal text-[15px]/[24px] align-middle tracking-[0.5px] text-c-wg-30 px-5"
//         dangerouslySetInnerHTML={{ __html: html }}
//       />

//       {url && (
//         <div className="px-5">
//           <div className="relative w-full max-h-80 aspect-video">
//             <Image
//               src={url}
//               alt="Uploaded image"
//               fill
//               className="object-cover rounded-[8px]"
//               sizes="100vw"
//             />
//           </div>
//         </div>
//       )}
//     </Col>
//   );
// }

// export function IconText({
//   children,
//   className,
//   ...props
// }: React.HTMLAttributes<HTMLDivElement> & { children: React.ReactNode }) {
//   return (
//     <Row
//       className={`justify-center items-center gap-1.25 text-white font-normal text-[15px] px-4 py-5 ${className || ''}`}
//       {...props}
//     >
//       {children}
//     </Row>
//   );
// }

// export function UserBadge({
//   author_type,
//   profile_url,
//   name,
// }: {
//   author_type: UserType;
//   profile_url: string;
//   name: string;
// }) {
//   return (
//     <Row className="w-fit items-center med-16 text-white">
//       <Image
//         src={profile_url}
//         alt="User Profile"
//         width={24}
//         height={24}
//         className={
//           author_type == UserType.Team
//             ? 'w-6 h-6 rounded-sm object-cover'
//             : 'w-6 h-6 rounded-full object-cover'
//         }
//       />
//       <span>{name}</span>
//     </Row>
//   );
// }

// export function IndustryTag({ industry }: { industry: string }) {
//   return (
//     <span className="rounded-sm border border-c-wg-70 px-2 text-xs/[25px] font-semibold align-middle uppercase">
//       {industry}
//     </span>
//   );
// }

// export function OnboradingTag() {
//   return (
//     <span className="rounded-sm bg-primary text-white px-2 text-xs/[25px] font-semibold align-middle uppercase">
//       Onboard
//     </span>
//   );
// }

// // Modify the FeedFooter component props:
// interface FeedFooterProps extends FeedCardProps {
//   setFeedContent?: (content: string) => void;
// }

// export function FeedFooter({
//   likes,
//   comments,
//   rewards,
//   shares,
//   is_liked,
//   onLikeClick,
//   isLikeProcessing,
//   onRepostThought,
//   onRepost,
//   contents
// }: FeedFooterProps) {
//   const { newDraft, setFeedContent } = useRepostDraft();
//   return (
//     <Row className="items-center justify-around border-t w-full border-neutral-800">
//       <IconText
//         onClick={(evt) => {
//           evt.stopPropagation();
//           if (!isLikeProcessing) {
//             onLikeClick?.(!is_liked);
//           }
//         }}
//         className={
//           isLikeProcessing ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'
//         }
//       >
//         <ThumbUp
//           className={
//             is_liked
//               ? '[&>path]:fill-primary [&>path]:stroke-primary'
//               : undefined
//           }
//         />
//         {convertNumberToString(likes)}
//       </IconText>
//       <IconText>
//         <CommentIcon />
//         {convertNumberToString(comments)}
//       </IconText>
//       <IconText>
//         <Rewards />
//         {convertNumberToString(rewards)}
//       </IconText>
//       <IconText>
//         {/* <button>
//           <Shares />
//           {convertNumberToString(shares)}
//         </button> */}

//         {
//           <DropdownMenu modal={false}>
//             <DropdownMenuTrigger asChild>
//               <button onClick={(e) => e.stopPropagation()}>
//                 <Shares />
//                 {convertNumberToString(shares)}
//               </button>
//             </DropdownMenuTrigger>

//             <DropdownMenuContent
//               align="end"
//               className="w-84 border-0 transition ease-out duration-100 py-4 px-2 "
//             >
//               <DropdownMenuItem asChild>
//                 <button
//                   onClick={(e) => {
//                     e.stopPropagation();
//                     onRepostThought?.();
//                     // newDraft();
//                     // setFeedContent?.(contents);
//                   }}
//                   className="flex  items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700 transition-colors text-white text-lg font-semibold"
//                 >
//                   <Edit1 className="" />
//                   Repost with your thoughts
//                 </button>
//               </DropdownMenuItem>

//               <DropdownMenuItem asChild>
//                 <button
//                   onClick={(e) => {
//                     newDraft();
//                     e.stopPropagation();
//                     // onRepost?.();
//                   }}
//                   className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700  text-white text-sm mt-1 font-semibold"
//                 >
//                   <Shares />
//                   Repost
//                 </button>
//               </DropdownMenuItem>
//             </DropdownMenuContent>
//           </DropdownMenu>
//         }
//       </IconText>
//     </Row>
//   );
// }

'use client';
import React, { useState, useEffect } from 'react';
import { Col } from './ui/col';
import { Row } from './ui/row';
import { CommentIcon, Rewards, Shares, ThumbUp } from './icons';
import { convertNumberToString } from '@/lib/number-utils';
import TimeAgo from './time-ago';
import DOMPurify from 'dompurify';
import { Button } from './ui/button';
import { useRouter } from 'next/navigation';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { route } from '@/route';
import { SpaceType } from '@/lib/api/models/spaces';
import {
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenu,
  DropdownMenuItem,
} from './ui/dropdown-menu';
import { Edit1 } from './icons';
import { useRepostDraft } from '@/app/(social)/_components/create-repost2';

export interface FeedCardProps {
  id: number;
  industry: string;
  title: string;
  contents: string;
  author_profile_url: string;
  author_name: string;
  author_type: UserType;
  url?: string;
  created_at: number;
  likes: number;
  is_liked: boolean;
  comments: number;
  rewards: number;
  shares: number;
  space_id?: number;
  space_type?: SpaceType;
  author_id: number;
  user_id: number;
  onboard: boolean;
  onRepostThought?: () => void;
  onRepost?: () => void;
  onLikeClick?: (value: boolean) => void;
  refetch?: () => void;
  isLikeProcessing?: boolean;
}

export default function FeedCard(props: FeedCardProps) {
  const router = useRouter();
  const { post } = useApiCall();

  const [localLikes, setLocalLikes] = useState(props.likes);
  const [localIsLiked, setLocalIsLiked] = useState(props.is_liked);
  const [isProcessing, setIsProcessing] = useState(false);

  const {
    setFeedContent,
    setFeedImageUrl,
    newDraft,
    setOriginalFeedId,
    setTitle,
    setContent,
    publishPost,
    setAuthorName,
    setAuthorProfileUrl,
    setIndustry,
  } = useRepostDraft();

  // Sync with props when they change
  useEffect(() => {
    setLocalLikes(props.likes);
    setLocalIsLiked(props.is_liked);
  }, [props.likes, props.is_liked]);

  const handleLike = async (value: boolean) => {
    if (isProcessing) return;

    setIsProcessing(true);
    setLocalIsLiked(value);
    setLocalLikes((prev) => (value ? prev + 1 : prev - 1));

    try {
      await post(ratelApi.feeds.likePost(props.id), {
        like: { value },
      });
      props.onLikeClick?.(value);
      props.refetch?.();
    } catch (error) {
      setLocalIsLiked(props.is_liked);
      setLocalLikes(props.likes);
      console.error('Failed to update like:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRepost = async () => {
    try {
      // Your repost implementation
      setOriginalFeedId(props.id);
      setFeedContent(props.contents || '');
      setFeedImageUrl(props.url || '');
      setTitle(''); // No thoughts
      setContent(null); // No content
      setAuthorName(props.author_name);
      setAuthorProfileUrl(props.author_profile_url);
      publishPost();
    } catch (error) {
      console.error('Repost failed:', error);
    }
  };

  const handleRepostThought = () => {
    setAuthorName(props.author_name);
    setIndustry(props.industry)
    setAuthorProfileUrl(props.author_profile_url);
    setFeedContent(props.contents);
    setFeedImageUrl(props.url || null);
    setOriginalFeedId(props.id); // Add this
    newDraft();
  };
  return (
    <Col
      className="cursor-pointer bg-component-bg rounded-[10px]"
      onClick={() => router.push(route.threadByFeedId(props.id))}
    >
      <FeedBody {...props} />
      <FeedFooter
        {...props}
        likes={localLikes}
        is_liked={localIsLiked}
        isLikeProcessing={isProcessing}
        onRepostThought={handleRepostThought}
        onRepost={handleRepost}
      />
    </Col>
  );
}

function FeedBody({
  industry,
  title,
  contents,
  author_name,
  author_profile_url,
  url,
  created_at,
  author_type,
  space_type,
  space_id,
  onboard,
}: FeedCardProps) {
  const router = useRouter();
  return (
    <Col className="pt-5 pb-2.5">
      <Row className="justify-between px-5">
        <Row>
          <IndustryTag industry={industry} />
          {onboard && <OnboradingTag />}
        </Row>
        {space_id && space_type ? (
          <Button
            variant="rounded_primary"
            className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
            onClick={(e) => {
              e.stopPropagation();
              if (space_type === SpaceType.Committee) {
                router.push(route.commiteeSpaceById(space_id));
              } else {
                router.push(route.deliberationSpaceById(space_id));
              }
            }}
          >
            Join
          </Button>
        ) : (
          <div />
        )}
      </Row>
      <h2 className="w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-white px-5">
        {title}
      </h2>
      <Row className="justify-between items-center px-5">
        <UserBadge
          profile_url={author_profile_url}
          name={author_name}
          author_type={author_type}
        />
        <TimeAgo timestamp={created_at} />
      </Row>
      <FeedContents contents={contents} url={url} />
    </Col>
  );
}

function FeedContents({ contents, url }: { contents: string; url?: string }) {
  const html =
    typeof window !== 'undefined' ? DOMPurify.sanitize(contents) : contents;

  return (
    <Col className="text-white">
      <div
        className="feed-content font-normal text-[15px]/[24px] align-middle tracking-[0.5px] text-c-wg-30 px-5"
        dangerouslySetInnerHTML={{ __html: html }}
      />
      {url && (
        <div className="px-5">
          <div className="relative w-full max-h-80 aspect-video">
            <Image
              src={url}
              alt="Uploaded image"
              fill
              className="object-cover rounded-[8px]"
              sizes="100vw"
            />
          </div>
        </div>
      )}
    </Col>
  );
}

function IconText({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement> & { children: React.ReactNode }) {
  return (
    <Row
      className={`justify-center items-center gap-1.25 text-white font-normal text-[15px] px-4 py-5 ${className || ''}`}
      {...props}
    >
      {children}
    </Row>
  );
}

function UserBadge({
  author_type,
  profile_url,
  name,
}: {
  author_type: UserType;
  profile_url: string;
  name: string;
}) {
  return (
    <Row className="w-fit items-center med-16 text-white">
      <Image
        src={profile_url}
        alt="User Profile"
        width={24}
        height={24}
        className={
          author_type == UserType.Team
            ? 'w-6 h-6 rounded-sm object-cover'
            : 'w-6 h-6 rounded-full object-cover'
        }
      />
      <span>{name}</span>
    </Row>
  );
}

function IndustryTag({ industry }: { industry: string }) {
  return (
    <span className="rounded-sm border border-c-wg-70 px-2 text-xs/[25px] font-semibold align-middle uppercase">
      {industry}
    </span>
  );
}

function OnboradingTag() {
  return (
    <span className="rounded-sm bg-primary text-white px-2 text-xs/[25px] font-semibold align-middle uppercase">
      Onboard
    </span>
  );
}

interface FeedFooterProps extends Omit<FeedCardProps, 'onRepostThought'> {
  onRepostThought?: () => void;
  isLikeProcessing?: boolean;
}

function FeedFooter({
  likes,
  comments,
  rewards,
  shares,
  is_liked,
  onLikeClick,
  isLikeProcessing,
  onRepost,
  onRepostThought,
}: FeedFooterProps) {
  const [isReposting, setIsReposting] = useState(false);

  const handleRepostWithThoughts = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsReposting(true);
    try {
      onRepostThought?.();
    } catch (error) {
      console.error('Failed to repost:', error);
    } finally {
      setIsReposting(false);
    }
  };

  const handleRepost = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsReposting(true);

    try {
      onRepost?.();
    } catch (error) {
      console.error('Failed to repost:', error);
    } finally {
      setIsReposting(false);
    }
  };
  return (
    <Row className="items-center justify-around border-t w-full border-neutral-800">
      <IconText
        onClick={(evt) => {
          evt.stopPropagation();
          if (!isLikeProcessing) onLikeClick?.(!is_liked);
        }}
        className={
          isLikeProcessing ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'
        }
      >
        <ThumbUp
          className={
            is_liked
              ? '[&>path]:fill-primary [&>path]:stroke-primary'
              : undefined
          }
        />
        {convertNumberToString(likes)}
      </IconText>

      <IconText>
        <CommentIcon />
        {convertNumberToString(comments)}
      </IconText>

      <IconText>
        <Rewards />
        {convertNumberToString(rewards)}
      </IconText>

      <IconText>
        <DropdownMenu modal={false}>
          <DropdownMenuTrigger asChild>
            <button onClick={(e) => e.stopPropagation()}>
              <Shares />
              {convertNumberToString(shares)}
            </button>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            align="end"
            className="w-84 border-0 transition ease-out duration-100 py-4 px-2"
          >
            <DropdownMenuItem asChild>
              <button
                onClick={handleRepostWithThoughts}
                disabled={isReposting}
                className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700 transition-colors text-white text-lg font-semibold"
              >
                {isReposting ? (
                  <span className="animate-spin">↻</span>
                ) : (
                  <Edit1 />
                )}
                Repost with your thoughts
              </button>
            </DropdownMenuItem>

            <DropdownMenuItem asChild>
              <button
                onClick={handleRepost}
                disabled={isReposting}
                className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700 transition-colors text-white text-lg font-semibold"
              >
                {isReposting ? (
                  <span className="animate-spin">↻</span>
                ) : (
                  <Shares />
                )}
                Repost
              </button>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </IconText>
    </Row>
  );
}
