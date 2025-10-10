import {
  BackButton,
  EditButton,
  MakePublicButton,
  PublishSpaceButton,
  SaveButton,
} from '@/components/post-header/buttons';
import { useSpaceHeaderContext } from './context';

export function SpaceModifySection() {
  // Context로부터 필요한 모든 상태와 함수를 가져옵니다.
  const {
    space,
    isEditing,
    isDraft,
    isPublic,
    isModified,
    hasEditPermission,
    onGoBack,
    onSave,
    onEdit,
  } = useSpaceHeaderContext();

  return (
    <div className="flex flex-col w-full gap-2.5">
      <SpaceModifySection
        title={isEdit ? commonData?.title ?? '' : space.title}
        isDraft={isDraft}
        isPublic={isPublic}
        authorId={space.author[0]?.id}
        authorName={space.author[0]?.username}
        spaceId={spaceId}
        onEdit={handleStartEdit}
        onDelete={handleDelete}
      />
      <PostInfoSection
        likes={feed.post.likes}
        shares={feed.post.shares}
        comments={feed.post.comments}
        rewards={feed.post.rewards ?? 0}
        isDraft={isDraft}
        isPublic={isPublic}
      />
      <TitleSection
        isEdit={isEdit}
        title={isEdit ? commonData?.title ?? '' : space.title}
        setTitle={(newTitle) => updateCommonData({ title: newTitle })}
        handleShare={handleShare}
      />
      <AuthorSection
        type={author.user_type}
        profileImage={author.profile_url}
        name={author.nickname}
        isCertified={true}
        createdAt={space.created_at}
      />
    </div>
  );
}
