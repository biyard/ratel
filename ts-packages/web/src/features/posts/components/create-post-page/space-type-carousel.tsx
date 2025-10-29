import { useRef, useState, useEffect } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { Row } from '@/components/ui/row';

interface SpaceTypeCarouselProps {
  children: React.ReactNode;
}

export function SpaceTypeCarousel({ children }: SpaceTypeCarouselProps) {
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);

  const checkScrollability = () => {
    const container = scrollContainerRef.current;
    if (!container) return;

    setCanScrollLeft(container.scrollLeft > 0);
    setCanScrollRight(
      container.scrollLeft < container.scrollWidth - container.clientWidth - 1,
    );
  };

  useEffect(() => {
    checkScrollability();
    const container = scrollContainerRef.current;
    if (!container) return;

    const handleScroll = () => checkScrollability();
    container.addEventListener('scroll', handleScroll);
    window.addEventListener('resize', checkScrollability);

    return () => {
      container.removeEventListener('scroll', handleScroll);
      window.removeEventListener('resize', checkScrollability);
    };
  }, [children]);

  const scroll = (direction: 'left' | 'right') => {
    const container = scrollContainerRef.current;
    if (!container) return;

    const scrollAmount = 300;
    const newScrollLeft =
      direction === 'left'
        ? container.scrollLeft - scrollAmount
        : container.scrollLeft + scrollAmount;

    container.scrollTo({
      left: newScrollLeft,
      behavior: 'smooth',
    });
  };

  return (
    <div className="relative">
      {/* Left scroll button */}
      {canScrollLeft && (
        <button
          onClick={() => scroll('left')}
          className="absolute left-0 top-1/2 -translate-y-1/2 z-10 w-8 h-8 flex items-center justify-center bg-component-bg border border-input-box-border rounded-full shadow-lg hover:bg-input-box-bg transition-colors"
          aria-label="Scroll left"
        >
          <ChevronLeft size={20} className="text-text-primary" />
        </button>
      )}

      {/* Scrollable container */}
      <div
        ref={scrollContainerRef}
        className="overflow-x-scroll scrollbar-hide"
        style={{
          scrollbarWidth: 'none',
          msOverflowStyle: 'none',
        }}
      >
        <Row className="w-fit">{children}</Row>
      </div>

      {/* Right scroll button */}
      {canScrollRight && (
        <button
          onClick={() => scroll('right')}
          className="absolute right-0 top-1/2 -translate-y-1/2 z-10 w-8 h-8 flex items-center justify-center bg-component-bg border border-input-box-border rounded-full shadow-lg hover:bg-input-box-bg transition-colors"
          aria-label="Scroll right"
        >
          <ChevronRight size={20} className="text-text-primary" />
        </button>
      )}
    </div>
  );
}
