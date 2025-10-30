import { Node, mergeAttributes } from '@tiptap/core';

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    video: {
      setVideo: (attrs: {
        src: string;
        controls?: boolean;
        autoplay?: boolean;
        loop?: boolean;
        muted?: boolean;
        poster?: string | null;
        style?: string | null;
      }) => ReturnType;
    };
  }
}

const Video = Node.create({
  name: 'video',
  group: 'block',
  atom: true,
  selectable: true,
  draggable: true,

  addAttributes() {
    return {
      src: { default: null },
      controls: { default: true },
      autoplay: { default: false },
      loop: { default: false },
      muted: { default: false },
      poster: { default: null },
      style: {
        default:
          'max-width:100%;height:auto;display:block;margin:1rem auto;border-radius:0.5rem;',
      },
    };
  },

  parseHTML() {
    return [{ tag: 'video' }];
  },

  renderHTML({ HTMLAttributes }) {
    return ['video', mergeAttributes(HTMLAttributes)];
  },

  addCommands() {
    return {
      setVideo:
        (attrs) =>
        ({ chain }) =>
          chain()
            .insertContent({
              type: this.name,
              attrs,
            })
            .run(),
    };
  },
});

export default Video;
