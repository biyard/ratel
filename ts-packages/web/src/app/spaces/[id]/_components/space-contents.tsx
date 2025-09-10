'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import React from 'react';
import TextEditor from '@/components/text-editor/text-editor';

export interface SpaceContentsProps {
  isEdit?: boolean;
  htmlContents: string;
  setContents?: (htmlContents: string) => void;
}

export default function SpaceContents({
  isEdit = false,
  htmlContents,
  setContents = () => {},
}: SpaceContentsProps) {
  const html = (
    <BlackBox isWhite={true}>
      <div
        className="rich-content"
        dangerouslySetInnerHTML={{ __html: htmlContents }}
      />
      <style jsx global>{`
        .rich-content {
          color: #525252;
          font-size: 15px;
          line-height: 24px;
        }

        html.dark .rich-content,
        html[data-theme='dark'] .rich-content {
          color: #d4d4d4 !important;
        }

        .rich-content h1,
        .rich-content h2,
        .rich-content h3,
        .rich-content h4,
        .rich-content h5,
        .rich-content h6 {
          font-weight: 700;
          margin-bottom: 20px;
        }
        .rich-content h1 {
          font-size: 28px;
        }
        .rich-content h2 {
          font-size: 22px;
        }
        .rich-content h3 {
          font-size: 18px;
        }

        .rich-content p {
          margin-bottom: 20px;
        }
        .rich-content strong {
          font-weight: bold;
        }
        .rich-content em {
          font-style: italic;
        }
        .rich-content u {
          text-decoration: underline;
        }

        .rich-content ul {
          list-style-type: disc;
          margin-left: 20px;
          margin-bottom: 20px;
        }
        .rich-content ol {
          list-style-type: decimal;
          margin-left: 20px;
          margin-bottom: 20px;
        }
        .rich-content li {
          margin-bottom: 5px;
        }
      `}</style>
    </BlackBox>
  );

  return isEdit ? (
    <TextEditor
      content={htmlContents}
      onChange={(text: string) => {
        setContents(text);
      }}
    />
  ) : (
    html
  );
}
