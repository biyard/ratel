import type { AttachedFile, Comment } from '@/types/thread';

export const attachedFiles: AttachedFile[] = [
  { id: '1', name: '학업용품.pdf', type: 'pdf', size: '2.4 MB' },
  { id: '2', name: '학업용품.pdf', type: 'pdf', size: '1.8 MB' },
  { id: '3', name: '학업용품.pdf', type: 'pdf', size: '3.2 MB' },
  { id: '4', name: '학업용품.pdf', type: 'pdf', size: '1.5 MB' },
];

export const externalLinks = [
  { url: '#', text: '[dao.title] This is link for d109231' },
  { url: '#', text: '[dao.title] This is link for d109231' },
  { url: '#', text: '[dao.title] This is link for d109231' },
];

export const initialComments: Comment[] = [
  {
    id: '1',
    author: {
      name: 'zf4EWAe',
      avatar: '/placeholder.svg?height=32&width=32',
    },
    content: '',
    timestamp: '12hrs ago',
    likes: 0,
    replies: [
      {
        id: '2',
        author: {
          name: 'DW001',
          avatar: '/placeholder.svg?height=32&width=32',
        },
        content:
          'I agree that maintenance is important, and we are actively considering mechanisms to ensure it. However, as stated in the proposal, "the program does not guarantee placement within the DAO." Ultimately, it is up to each fellow to find their role within the DAO, but we will be there to provide guidance and support.\n\nThank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.\n\nWhat I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under "Other."',
        timestamp: '12hrs ago',
        likes: 481,
        replies: [],
      },
    ],
  },
  {
    id: '3',
    author: {
      name: 'DW001',
      avatar: '/placeholder.svg?height=32&width=32',
    },
    content:
      'I agree that maintenance is important, and we are actively considering mechanisms to ensure it. However, as stated in the proposal, "the program does not guarantee placement within the DAO." Ultimately, it is up to each fellow to find their role within the DAO, but we will be there to provide guidance and support.',
    timestamp: '12hrs ago',
    likes: 0,
    percentage: 80,
    replies: [],
  },
  {
    id: '4',
    author: {
      name: 'zf4EWAe',
      avatar: '/placeholder.svg?height=32&width=32',
    },
    content: '',
    timestamp: '12hrs ago',
    likes: 0,
    replies: [
      {
        id: '5',
        author: {
          name: 'DW001',
          avatar: '/placeholder.svg?height=32&width=32',
        },
        content:
          'I agree that maintenance is important, and we are actively considering mechanisms to ensure it. However, as stated in the proposal, "the program does not guarantee placement within the DAO." Ultimately, it is up to each fellow to find their role within the DAO, but we will be there to provide guidance and support.\n\nThank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.\n\nWhat I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under "Other."',
        timestamp: '12hrs ago',
        likes: 481,
        replies: [],
      },
    ],
  },
];
