export const TEAM_NAME = '입법처 실증 준비 팀';
export const TEAM_ID = (timestamp: number) => `iitp-poc-1-${timestamp}`;
export const TEAM_DESCRIPTION =
  '국회입법조사처와 함께 진행하는 공론조사를 준비하는 팀입니다.';

export const POST_TITLE =
  '리워드 스페이스의 보상 방식, 참여도 중심이 맞을까 품질 중심이 맞을까?';
export const POST_CONTENT = `1️⃣ 배경 설명
Ratel은 사용자의 참여를 보상하는 RewardSpace 기능을 제공합니다.
현재 대부분의 스페이스에서는 활동 횟수(참여도) 를 기준으로 포인트가 분배되고 있습니다.
하지만 최근 일부 크리에이터들은 "참여의 품질을 더 중시해야 한다"고 주장하고 있습니다.

2️⃣ 쟁점
A안: 참여도 중심 분배
B안: 품질 중심 분배

3️⃣ 질문
Q1. 당신은 어떤 보상 기준이 더 적절하다고 생각하나요?
Q2. 품질 평가를 도입한다면, 누가 평가하는 것이 적절할까요?`;

export const YOUTUBE_LINK = 'https://www.youtube.com/watch?v=R2X4BJ1KNM4';

// Survey questions
export const SURVEY_QUESTIONS = [
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title:
      'Ratel과 같은 온라인 공론장(토론·투표 플랫폼)에 참여해본 적이 있습니까?',
    options: [
      '자주 참여한다',
      '가끔 참여한다',
      '이름은 들어봤지만 참여한 적은 없다',
      '전혀 참여해본 적이 없다',
    ],
  },
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title: '공론조사에 참여할 때 가장 중요하다고 생각하는 요소는 무엇입니까?',
    options: [
      '다양한 의견이 공정하게 반영되는 구조',
      '토론 주제의 공익성',
      '참여에 따른 보상',
      '편리한 참여 환경 (UI, 시간 등)',
      '익명성과 개인정보 보호',
    ],
  },
  {
    type: 'multiple_choice',
    displayName: 'Multiple Choice',
    required: true,
    title:
      '당신이 공론조사에 적극적으로 참여하게 되는 이유를 모두 선택해주세요.',
    options: [
      '사회적 의사결정에 영향을 미칠 수 있어서',
      '자신의 의견이 기록으로 남는 것이 좋아서',
      '토론을 통해 새로운 시각을 얻을 수 있어서',
      '포인트·리워드 등 보상이 있어서',
      '친구나 커뮤니티의 추천으로',
    ],
  },
  {
    type: 'short_answer',
    displayName: 'Short Answer',
    required: true,
    title:
      '온라인 공론조사나 토론 플랫폼을 신뢰하기 위해 가장 필요한 요소는 무엇이라고 생각하나요?',
  },
  {
    type: 'subjective',
    displayName: 'Subjective',
    required: true,
    title:
      '당신이 직접 공론조사 주제를 제안할 수 있다면, 어떤 주제를 제안하고 싶나요?',
  },
];

// Board posts
export const BOARD_POSTS = [
  {
    category: '보상 기준의 공정성과 효율성',
    title: '활동량 기준 보상이 공정하다는 이유',
    content:
      "RewardSpace는 '참여' 그 자체를 장려하기 위해 설계된 시스템입니다.",
  },
  {
    category: '보상 기준의 공정성과 효율성',
    title: '품질 중심 보상이 커뮤니티를 성숙하게 만든다',
    content: '양적인 참여보다 중요한 것은 기여의 깊이입니다.',
  },
  {
    category: 'AI 평가와 사용자 자율성의 균형',
    title: 'AI 평가 도입, 공정성 향상의 첫걸음',
    content: 'AI는 감정이나 사적 이해관계가 없습니다.',
  },
  {
    category: 'AI 평가와 사용자 자율성의 균형',
    title: 'AI 평가가 자율성과 창의성을 제한할 수도 있다',
    content:
      "AI가 모든 참여를 수치화하고 등급을 매기기 시작하면, 사람들은 '잘 보이기 위한 발언'만 하게 될 위험이 있습니다.",
  },
];
