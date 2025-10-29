import { logger } from '@/lib/logger';
import { editorTheme } from './editor-theme';
import { TableCellNode, TableNode, TableRowNode } from '@lexical/table';
import { ListNode, ListItemNode } from '@lexical/list';

export const editorConfig = {
  namespace: 'CreatePostEditor',
  theme: editorTheme,
  nodes: [TableNode, TableCellNode, TableRowNode, ListNode, ListItemNode],
  onError(error: Error) {
    logger.error('Lexical error:', error);
  },
};
