import { logger } from '@/lib/logger';
import { editorTheme } from './editor-theme';
import { TableCellNode, TableNode, TableRowNode } from '@lexical/table';

export const editorConfig = {
  namespace: 'CreatePostEditor',
  theme: editorTheme,
  nodes: [TableNode, TableCellNode, TableRowNode],
  onError(error: Error) {
    logger.error('Lexical error:', error);
  },
};
