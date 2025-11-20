import { ReactNode, useCallback, useRef } from 'react';

interface ScrollableProps extends React.HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  onReachBottom: () => void;
}

export default function Scrollable({
  onReachBottom,
  children,
  ...props
}: ScrollableProps) {
  const observer = useRef<IntersectionObserver | null>(null);

  const lastItemRef = useCallback(
    (node: HTMLDivElement) => {
      if (observer.current) observer.current.disconnect();

      observer.current = new IntersectionObserver((entries) => {
        if (entries[0]?.isIntersecting) {
          onReachBottom();
        }
      });

      if (node) observer.current.observe(node);
    },
    [onReachBottom],
  );

  return (
    <div {...props}>
      {children}
      <div ref={lastItemRef} />
    </div>
  );
}
