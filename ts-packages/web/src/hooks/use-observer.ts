import { useEffect, useRef } from 'react';

/**
 * A custom hook that uses IntersectionObserver.
 * Executes a callback function when a specific element enters the viewport.
 *
 * @param onIntersect - The callback function to be executed when the element enters the viewport.
 * @param options - Options to be passed to the IntersectionObserver (e.g., threshold, rootMargin).
 * @returns - The ref object to be attached to the element to be observed.
 */
export function useObserver<T extends HTMLElement>(
  onIntersect: () => void,
  options?: IntersectionObserverInit,
) {
  const ref = useRef<T | null>(null);

  useEffect(() => {
    const element = ref.current;

    // Do not run if the element or callback function is not present
    if (!element || !onIntersect) {
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        // Execute the callback when isIntersecting is true (i.e., when the element is visible in the viewport)
        if (entries[0].isIntersecting) {
          onIntersect();
        }
      },
      { ...options },
    );

    // Start observing the element
    observer.observe(element);

    // Stop observing when the component unmounts
    return () => {
      observer.unobserve(element);
    };
  }, [ref, onIntersect, options]);

  return ref;
}

/**
 * Usage Example:
 *
 * function MyComponent() {
 *   const { data, fetchNextPage, hasNextPage } = useInfiniteQuery(...);
 *
 *   const handleIntersect = () => {
 *     if (hasNextPage) {
 *       fetchNextPage();
 *     }
 *   };
 *
 *   const observerRef = useObserver(handleIntersect, { threshold: 0.5 });
 *
 *   return (
 *     <div>
 *       {data.pages.map(page => ...)}
 *       <div ref={observerRef} style={{ height: '1px' }} /> // The target element to observe
 *     </div>
 *   );
 * }
 */
