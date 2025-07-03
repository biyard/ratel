export type NotificationItem = {
  id: number;
  type: 'Community' | 'Space' | 'User';
  icon: string;
  title: string;
  message: string;
  description: string;
  timeAgo: string;
};

export type FollowSuggestion = {
  id: number;
  name: string;
  role: string;
  image: string;
  fallback: string;
};

export const notifications: NotificationItem[] = [
  {
    id: 1,
    type: 'Community',
    icon: 'C',
    title: '[Community]',
    message: "We've updated the Community.",
    description: 'User name left a comment in the community.',
    timeAgo: '12hrs ago',
  },
  {
    id: 2,
    type: 'Space',
    icon: 'S',
    title: '[Space]',
    message: "We've updated the space.",
    description: 'User name left a comment in the space.',
    timeAgo: '12hrs ago',
  },
  {
    id: 3,
    type: 'User',
    icon: 'U',
    title: '[User name]',
    message: 'We are planning to launch for our project next.',
    description: 'Replies in [Community name]',
    timeAgo: '12hrs ago',
  },
  {
    id: 4,
    type: 'User',
    icon: 'U',
    title: '[User name]',
    message: 'We are planning to launch for our project next.',
    description: 'Mention in [Space name]',
    timeAgo: '12hrs ago',
  },
  {
    id: 5,
    type: 'Community',
    icon: 'C',
    title: '[Community]',
    message: "We've updated the Community.",
    description: 'User name left a comment in the community.',
    timeAgo: '12hrs ago',
  },
  {
    id: 6,
    type: 'Space',
    icon: 'S',
    title: '[Space]',
    message: "We've updated the space.",
    description: 'User name left a comment in the space.',
    timeAgo: '12hrs ago',
  },
  {
    id: 7,
    type: 'User',
    icon: 'U',
    title: '[User name]',
    message: 'We are planning to launch for our project next.',
    description: 'Replies in [Community name]',
    timeAgo: '12hrs ago',
  },
  {
    id: 8,
    type: 'User',
    icon: 'U',
    title: '[User name]',
    message: 'We are planning to launch for our project next.',
    description: 'Replies in [Space name]',
    timeAgo: '12hrs ago',
  },
];

export type Article = {
  id: number;
  title: string;
  description: string;
};

export const peopleToFollow: FollowSuggestion[] = [
  {
    id: 1,
    name: 'Donald Trump',
    role: 'President of the US',
    image: '/trump.jpg?height=40&width=40',
    fallback: 'DT',
  },
  {
    id: 2,
    name: 'Elon Musk',
    role: 'CEO of Tesla and SpaceX',
    image: '/elon.png?height=40&width=40',
    fallback: 'EM',
  },
  {
    id: 3,
    name: 'Jongseok Park',
    role: 'National Assembly of',
    image: '/jongsook.png?height=40&width=40',
    fallback: 'JP',
  },
];

export const articles: Article[] = [
  {
    id: 1,
    title: `Ratel' Launches Digital Asset Policy Comparison Feature Ahead of 2025 Election`,
    description: `Ratel, a blockchain-based legislative social media platform, has introduced a new feature that allows voters to...`,
  },
  {
    id: 2,
    title: `Legislative Platform 'Ratel' Records Public Opinion on Election Pledges via Blockchain`,
    description: `Ratel, a blockchain-based social media platform, has launched a new feature ahead of South Korea's 2025...`,
  },
  {
    id: 3,
    title: `Decentralized Legislative Platform 'RATEL' Leads Crypto Regulatory Reform in South Korea`,
    description: `RATEL is the world's first decentralized legislative DAO platform empowering communities to propose and monitor...`,
  },
];
