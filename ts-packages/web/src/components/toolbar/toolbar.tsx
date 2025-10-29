'use client';

import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useCallback, useEffect, useState } from 'react';
import {
  $getSelection,
  $isRangeSelection,
  FORMAT_TEXT_COMMAND,
  TextFormatType,
  COMMAND_PRIORITY_LOW,
  SELECTION_CHANGE_COMMAND,
} from 'lexical';
import {
  $getTableCellNodeFromLexicalNode,
  $isTableCellNode,
  $isTableNode,
  INSERT_TABLE_COMMAND,
  TableCellNode,
  $insertTableRow__EXPERIMENTAL,
  $deleteTableRow__EXPERIMENTAL,
  $insertTableRowAtSelection,
  $deleteTableRowAtSelection,
  $insertTableColumnAtSelection,
  $deleteTableColumnAtSelection,
} from '@lexical/table';
import {
  INSERT_ORDERED_LIST_COMMAND,
  INSERT_UNORDERED_LIST_COMMAND,
  REMOVE_LIST_COMMAND,
  $isListNode,
  ListNode,
} from '@lexical/list';
import {
  $getNearestNodeOfType,
  $findMatchingParent,
} from '@lexical/utils';
import {
  ImagePlus,
  Bold,
  Italic,
  Underline,
  Strikethrough,
  Table,
  Plus,
  Minus,
  Trash2,
  List,
  ListOrdered,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import FileUploader from '@/features/spaces/files/components/file-uploader';
import {
  EditorDeleteCol,
  EditorDeleteRow,
  EditorInsertAboveRow,
  EditorInsertBelowRow,
  EditorInsertLeftCol,
  EditorInsertRightCol,
} from '../icons';

export default function ToolbarPlugin({
  onImageUpload,
  enableImage = true,
}: {
  onImageUpload: (url: string) => void;
  enableImage?: boolean;
}) {
  const [editor] = useLexicalComposerContext();
  const [activeEditor, setActiveEditor] = useState(editor);
  const [isBold, setIsBold] = useState(false);
  const [isItalic, setIsItalic] = useState(false);
  const [isUnderline, setIsUnderline] = useState(false);
  const [isStrikethrough, setIsStrikethrough] = useState(false);
  const [isInTable, setIsInTable] = useState(false);
  const [isBulletList, setIsBulletList] = useState(false);
  const [isNumberedList, setIsNumberedList] = useState(false);

  const updateToolbar = useCallback(() => {
    const selection = $getSelection();
    if ($isRangeSelection(selection)) {
      setIsBold(selection.hasFormat('bold'));
      setIsItalic(selection.hasFormat('italic'));
      setIsUnderline(selection.hasFormat('underline'));
      setIsStrikethrough(selection.hasFormat('strikethrough'));

      // Check if selection is in a table
      const anchorNode = selection.anchor.getNode();
      const tableCell = $getTableCellNodeFromLexicalNode(anchorNode);
      setIsInTable(tableCell !== null);

      // Check if selection is in a list
      const element = anchorNode.getKey() === 'root'
        ? anchorNode
        : $findMatchingParent(anchorNode, (e) => {
            const parent = e.getParent();
            return parent !== null && $isListNode(parent);
          });

      if ($isListNode(element)) {
        const parentList = element.getParent();
        const type = parentList ? (parentList as ListNode).getListType() : element.getListType();
        setIsBulletList(type === 'bullet');
        setIsNumberedList(type === 'number');
      } else {
        setIsBulletList(false);
        setIsNumberedList(false);
      }
    }
  }, []);

  useEffect(() => {
    return editor.registerCommand(
      SELECTION_CHANGE_COMMAND,
      (_payload, newEditor) => {
        updateToolbar();
        setActiveEditor(newEditor);
        return false;
      },
      COMMAND_PRIORITY_LOW,
    );
  }, [editor, updateToolbar]);

  useEffect(() => {
    return activeEditor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        updateToolbar();
      });
    });
  }, [activeEditor, updateToolbar]);

  const formatText = (format: TextFormatType) => {
    activeEditor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  };

  const insertTable = () => {
    activeEditor.dispatchCommand(INSERT_TABLE_COMMAND, {
      columns: '3',
      rows: '3',
      includeHeaders: false,
    });
  };

  const insertTableRowAbove = () => {
    activeEditor.update(() => {
      $insertTableRowAtSelection(false);
    });
  };

  const insertTableRowBelow = () => {
    activeEditor.update(() => {
      $insertTableRowAtSelection(true);
    });
  };

  const insertTableColLeft = () => {
    activeEditor.update(() => {
      $insertTableColumnAtSelection(false);
    });
  };

  const insertTableColRight = () => {
    activeEditor.update(() => {
      $insertTableColumnAtSelection(true);
    });
  };

  const deleteTableRow = () => {
    activeEditor.update(() => {
      $deleteTableRowAtSelection();
    });
  };

  const deleteTableCol = () => {
    activeEditor.update(() => {
      $deleteTableColumnAtSelection();
    });
  };

  const toggleBulletList = () => {
    if (isBulletList) {
      activeEditor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
    } else {
      activeEditor.dispatchCommand(INSERT_UNORDERED_LIST_COMMAND, undefined);
    }
  };

  const toggleNumberedList = () => {
    if (isNumberedList) {
      activeEditor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
    } else {
      activeEditor.dispatchCommand(INSERT_ORDERED_LIST_COMMAND, undefined);
    }
  };

  return (
    <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-hover">
      <button
        onClick={() => formatText('bold')}
        className={cn(isBold && 'bg-neutral-600 text-white')}
        aria-label="Format text as bold"
      >
        <Bold />
      </button>
      <button
        onClick={() => formatText('italic')}
        className={cn(isItalic && 'bg-neutral-600 text-white')}
        aria-label="Format text as italics"
      >
        <Italic />
      </button>
      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 text-white')}
        aria-label="Format text to underlined"
      >
        <Underline />
      </button>
      <button
        onClick={() => formatText('strikethrough')}
        className={cn(isStrikethrough && 'bg-neutral-600 text-white')}
        aria-label="Format text with a strikethrough"
      >
        <Strikethrough />
      </button>

      {/* List Tools */}
      <div className="w-px h-4 bg-neutral-600" />

      <button
        onClick={toggleBulletList}
        className={cn(isBulletList && 'bg-neutral-600 text-white')}
        aria-label="Bullet list"
        title="Bullet list"
      >
        <List />
      </button>
      <button
        onClick={toggleNumberedList}
        className={cn(isNumberedList && 'bg-neutral-600 text-white')}
        aria-label="Numbered list"
        title="Numbered list"
      >
        <ListOrdered />
      </button>

      {enableImage ? (
        <FileUploader onUploadSuccess={onImageUpload}>
          <ImagePlus />
        </FileUploader>
      ) : (
        <></>
      )}

      {/* Table Tools */}
      <div className="w-px h-4 bg-neutral-600" />

      {!isInTable && (
        <button
          onClick={insertTable}
          className="flex gap-1 items-center"
          aria-label="Insert table"
          title="Insert 3x3 table"
        >
          <Table size={16} />
        </button>
      )}

      {isInTable && (
        <>
          <button
            onClick={insertTableRowAbove}
            className="flex items-center w-[30px] h-[30px]"
            aria-label="Insert row above"
            title="Insert row above"
          >
            <EditorInsertAboveRow width={30} height={30} />
          </button>
          <button
            onClick={insertTableRowBelow}
            className="flex items-center"
            aria-label="Insert row below"
            title="Insert row below"
          >
            <EditorInsertBelowRow width={30} height={30} />
          </button>
          <button
            onClick={deleteTableRow}
            className="flex items-center text-red-500 hover:text-red-600"
            aria-label="Delete row"
            title="Delete current row"
          >
            <EditorDeleteRow width={30} height={30} />
          </button>

          <button
            onClick={insertTableColLeft}
            className="flex items-center"
            aria-label="Insert col left"
            title="Insert col left"
          >
            <EditorInsertLeftCol width={30} height={30} />
          </button>
          <button
            onClick={insertTableColRight}
            className="flex items-center"
            aria-label="Insert col right"
            title="Insert col right"
          >
            <EditorInsertRightCol width={30} height={30} />
          </button>
          <button
            onClick={deleteTableCol}
            className="flex items-center text-red-500 hover:text-red-600"
            aria-label="Delete col"
            title="Delete current col"
          >
            <EditorDeleteCol width={30} height={30} />
          </button>
        </>
      )}
    </div>
  );
}
