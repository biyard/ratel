'use client';

import type React from 'react';

import type { ReactNode } from 'react';
import { useState, forwardRef, useEffect } from 'react';
import { X } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import Search from '@/assets/icons/search.svg';
interface Tag {
  id: string;
  label: string;
  icon?: ReactNode; // Optional icon for the tag
}

interface SearchInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  onTagRemove?: (tagId: string) => void;
  tags?: Tag[];
}

const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>(
  (
    {
      placeholder = 'Search',
      value,
      onChange,
      onTagRemove,
      tags = [],
      className,
      ...props
    },
    ref,
  ) => {
    const [isFocused, setIsFocused] = useState(false);
    const [internalValue, setInternalValue] = useState<string>(
      (value as string) || '',
    );

    // Use internal state if value prop is not controlled
    useEffect(() => {
      if (value !== undefined) {
        setInternalValue(value as string);
      }
    }, [value]);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = e.target.value;
      setInternalValue(newValue);
      onChange?.(e); // Pass the event back if onChange is provided
    };

    const handleTagRemove = (tagId: string) => {
      onTagRemove?.(tagId);
    };

    const hasContent = internalValue.length > 0 || tags.length > 0;

    return (
      <div className={cn('relative w-full', className)}>
        <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 text-[#737373] w-5 h-5 z-10" />

        <Input
          ref={ref}
          type="text"
          value={internalValue}
          onChange={handleChange}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          placeholder={hasContent ? '' : placeholder}
          className={cn(
            'w-full h-14 pl-12 pr-6 bg-[#171717] border-2 rounded-full text-[#ffffff] placeholder:text-[#737373]',
            'focus-visible:ring-0 focus-visible:ring-offset-0 focus-visible:outline-none', // shadcn/ui focus-visible 스타일 재정의
            isFocused || tags.length > 0
              ? 'border-[#fcb300]'
              : 'border-transparent',
            props.readOnly && 'cursor-default', // readOnly prop이 있을 경우 커서 변경
          )}
          {...props} // Pass through any other standard input props
        />

        {/* Tags */}
        {tags.length > 0 && (
          <div className="absolute left-12 top-1/2 transform -translate-y-1/2 flex items-center gap-2">
            {tags.map((tag) => (
              <Badge
                key={tag.id}
                className="bg-[#737373] text-[#ffffff] hover:bg-[#737373] rounded-md px-3 py-1 text-sm font-normal flex items-center gap-2"
              >
                {tag.icon && (
                  <div className="w-3 h-3 bg-[#737373] rounded-sm"></div>
                )}
                {tag.label}
                <button
                  type="button" // Prevent form submission if inside a form
                  onClick={() => handleTagRemove(tag.id)}
                  className="hover:bg-[#262626] rounded-full p-0.5 transition-colors"
                >
                  <X className="w-3 h-3" />
                </button>
              </Badge>
            ))}
          </div>
        )}
      </div>
    );
  },
);

SearchInput.displayName = 'SearchInput';

export default SearchInput;
