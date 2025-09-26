import { useState } from "react";
import { Search, CheckCircle } from "lucide-react";

interface CommentProps {
  author: string;
  mention: string;
  text: string;
  time: string;
  replies: number;
  avatarGroup: string[];
  status: "done" | "pending";
  highlighted: boolean;
  id:number;
}

export default function SideCommentMenu() {
  const [comments] = useState<CommentProps[]>([
    {
      id: 1,
      author: "Hackartist",
      mention: "@Username",
      text: "Could you check this up real quick.",
      time: "1w ago",
      replies: 3,
      avatarGroup: [
        "https://i.pravatar.cc/40?img=1",
        "https://i.pravatar.cc/40?img=2",
        "https://i.pravatar.cc/40?img=3",
      ],
      status: "done",
      highlighted: false,
    },
    {
      id: 2,
      author: "Hackartist",
      mention: "@Username",
      text: "Could you check this up real quick.",
      time: "1w ago",
      replies: 3,
      avatarGroup: [
        "https://i.pravatar.cc/40?img=1",
        "https://i.pravatar.cc/40?img=2",
        "https://i.pravatar.cc/40?img=3",
      ],
      status: "pending",
      highlighted: true,
    },
    {
      id: 3,
      author: "Hackartist",
      mention: "@Username",
      text: "Could you check this up real quick.",
      time: "1w ago",
      replies: 3,
      avatarGroup: [
        "https://i.pravatar.cc/40?img=1",
        "https://i.pravatar.cc/40?img=2",
        "https://i.pravatar.cc/40?img=3",
      ],
      status: "done",
      highlighted: false,
    },
  ]);

  return (
    <div className="w-80 bg-white shadow-lg rounded-xl flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b">
        <h2 className="font-semibold flex items-center gap-2">
          <span>💬</span> Comments
        </h2>
        <button className="text-gray-500">≡</button>
      </div>

      {/* Search */}
      <div className="px-4 py-2">
        <div className="flex items-center bg-gray-100 rounded-lg px-3 py-2">
          <Search size={16} className="text-gray-400 mr-2" />
          <input
            type="text"
            placeholder="Search"
            className="bg-transparent outline-none text-sm w-full"
          />
        </div>
      </div>

      {/* Comments */}
      <div className="flex-1 overflow-y-auto space-y-2 p-4">
        {comments.map((comment, idx) => (
          <CommentBox
            key={idx}
            id={comment.id}
            author={comment.author}
            mention={comment.mention}
            text={comment.text}
            time={comment.time}
            replies={comment.replies}
            avatarGroup={comment.avatarGroup}
            status={comment.status}
            highlighted={comment.highlighted}
          />
        ))}
      </div>
    </div>
  );
}

function CommentBox({
  author,
  mention,
  text,
  time,
  replies,
  avatarGroup,
  status,
  highlighted,
}: CommentProps) {
  return (
    <div
      className={`p-3 rounded-xl border flex flex-col gap-1 relative ${
        highlighted ? "bg-gray-100" : "bg-white"
      }`}
    >
      {/* Avatars + status */}
      <div className="flex items-center justify-between">
        <div className="flex -space-x-2">
          {avatarGroup.map((src, idx) => (
            <img
              key={idx}
              src={src}
              alt="avatar"
              className="w-6 h-6 rounded-full border-2 border-white"
            />
          ))}
        </div>
        {status === "done" ? (
          <CheckCircle size={18} className="text-gray-400" />
        ) : (
          <CheckCircle size={18} className="text-yellow-500 fill-yellow-500" />
        )}
      </div>

      {/* Text */}
      <div className="text-sm">
        <p className="font-semibold">{author}</p>
        <p>
          <span className="text-yellow-600 font-medium">{mention}</span> {text}
        </p>
      </div>

      {/* Footer */}
      <div className="flex justify-between text-xs text-gray-500 mt-1">
        <span>{time}</span>
        <span className="text-yellow-600 cursor-pointer">{replies} replies</span>
      </div>
    </div>
  );
}