---
sidebar_position: 4
title: 팀 생성
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 팀 생성

## 왜 팀인가요?

팀은 Ratel 에서 여러 사람이 함께 글을 쓰고, 의사결정을 하고, 보상을 나눠 받는 단위입니다. 팀은 자체 핸들과 아레나 페이지, 자체 트레저리를 가지므로, 함께 만든 결과물이 개인이 아닌 **팀의 이름**으로 남습니다.

가까운 미래에는 모든 팀이 통합된 **Essence House** — 팀이 무엇을 지향하고 어떤 지식을 쌓아왔는지 보여주는 공유 공간 — 를 가지게 됩니다. 지금 단계에서는 팀을 "거버넌스가 기본 내장된 협업 단위"라고 생각하시면 됩니다.

## 팀 만들기

사이드바 하단의 프로필 아바타를 눌러 드롭다운을 열고 <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="20" y1="8" x2="20" y2="14"/><line x1="23" y1="11" x2="17" y2="11"/></svg> **Create Team (팀 생성)** 항목을 누르면 새 팀을 만들 수 있습니다. 짧은 팝업 한 번이면 기본 정보를 모두 입력할 수 있습니다.

- **팀 핸들** — 팀의 URL 이 됩니다: `/your-handle`. 짧고 안정적인 이름을 골라 주세요. 핸들은 나중에 변경하기 어렵습니다.
- **표시 이름(Display name)** — 플랫폼 곳곳에서 보이는 친근한 이름입니다.
- **배너 & 소개(Bio)** — 팀의 분위기를 한눈에 보여 주는 요소입니다.
- **초기 멤버** — 사용자명이나 이메일로 동료를 초대합니다. 초대받은 사람은 수락 또는 거절할 수 있습니다.

팀을 만들면 바로 팀 홈 `/your-handle/home` 으로 이동합니다.

## 팀 아레나 — 팀의 홈

모든 팀은 개인 페이지와는 별도로 자체 **아레나 레이아웃**을 가집니다. 아레나 상단의 HUD 탭은 다음과 같습니다.

| 탭 | URL | 아이콘 |
|---|---|---|
| **Home (홈)** | `/your-handle/home` | <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> |
| **Members (멤버)** | `/your-handle/members` | <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> |
| **Drafts (드래프트)** | `/your-handle/team-drafts` | <img src={useBaseUrl('/img/icons/file-edit.svg')} width="18" height="18" alt="File edit" style={{verticalAlign: 'middle'}} /> |
| **DAO** | `/your-handle/dao` | <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="3" y1="22" x2="21" y2="22"/><line x1="6" y1="18" x2="6" y2="11"/><line x1="10" y1="18" x2="10" y2="11"/><line x1="14" y1="18" x2="14" y2="11"/><line x1="18" y1="18" x2="18" y2="11"/><polygon points="12 2 20 7 4 7"/></svg> |
| **Rewards (보상)** | `/your-handle/team-rewards` | <img src={useBaseUrl('/img/icons/award.svg')} width="18" height="18" alt="Award" style={{verticalAlign: 'middle'}} /> |
| **Memberships (멤버십)** | `/your-handle/team-memberships` | <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="5" width="20" height="14" rx="2"/><line x1="2" y1="10" x2="22" y2="10"/></svg> |
| **Settings (설정)** | `/your-handle/team-settings` | <img src={useBaseUrl('/img/icons/settings.svg')} width="18" height="18" alt="Settings" style={{verticalAlign: 'middle'}} /> |

**Settings** 안에는:

- **멤버 설정** — `/your-handle/team-settings/members`
- **구독 설정** — `/your-handle/team-settings/subscription`

`/your-handle/home` 을 즐겨찾기에 등록해 두세요. 팀에서 일어나는 모든 활동의 기준점입니다.

## 멤버와 역할

멤버 탭 (`/your-handle/members`) 에는 현재 팀에 속한 모두와 수락 대기 중인 초대가 표시됩니다.

새 멤버를 들이려면 **설정 → 멤버**에서 초대를 보내세요. 초대받은 사람은 인박스에서 수락하거나 거절할 수 있고, 수락하는 즉시 공개 멤버 목록에 등장합니다.

크게 두 가지 역할이 있습니다.

- **팀 관리자(Team admin)** — 팀 설정 변경, 멤버 초대 및 제거, 구독 관리, DAO 결과 반영 등 팀의 정체성·과금에 영향을 미치는 작업을 수행합니다.
- **팀 멤버(Team member)** — 게시글 공동 작성, 드래프트 기여, DAO 투표, 보상 수령 등 일상적인 활동을 함께 합니다.

대부분의 일상 활동은 모든 멤버에게 열려 있고, 팀의 정체성이나 결제와 관련된 작업만 관리자 권한이 필요합니다.

## 게시글 · 드래프트 · 보상 · 멤버십

팀은 개인 사용자처럼 완전한 발행 주체로 동작합니다.

### 게시글
팀 핸들로 발행된 글은 팀 홈과 팀 피드에 노출됩니다. 모든 멤버가 팀 이름으로 글을 발행할 수 있습니다.

### 드래프트
드래프트 탭 (`/your-handle/team-drafts`) 은 공유 작업 공간입니다. 다른 팀원이 시작한 글을 이어서 쓰거나, 동시에 함께 다듬은 뒤 발행할 수 있습니다.

### 보상
보상 탭 (`/your-handle/team-rewards`) 에는 팀이 운영한 스페이스, 보상받은 게시글, 거버넌스 참여 등으로 누적된 보상이 표시됩니다. 보상은 팀 트레저리에 둘 수도 있고 멤버에게 분배할 수도 있습니다.

### 멤버십
팀은 서포터를 대상으로 **유료 멤버십**(`/your-handle/team-memberships`) 을 운영할 수 있습니다. 서포터는 팀을 구독하고 멤버 전용 콘텐츠나 혜택에 접근할 수 있습니다. 이는 아래의 **팀 구독**과는 별개입니다.

## 팀 구독

`/your-handle/team-settings/subscription` 에서 팀 자체의 요금제를 관리합니다.

이 구독은 팀 단위 기능 (보상 스페이스용 월간 Credit, Trusted Creator 뱃지, 상위 등급의 참여자 원본 데이터 열람) 을 활성화합니다. 결제는 PortOne 을 통한 오프체인 방식이라 익숙한 결제 수단을 그대로 쓸 수 있어요. 전체 내역은 [팀 설정 → 팀 구독](./team-settings.md#-팀-구독) 을 참고하세요.

## DAO — 집단 거버넌스

모든 팀에는 `/your-handle/dao` 에 자체 DAO 가 내장되어 있습니다.

DAO 는 한 사람의 결정이 아닌 팀의 결정으로 일을 진행하는 공간입니다. 멤버 누구나 **제안(Proposal)** 을 열 수 있습니다 — 예: "트레저리의 일부를 캠페인에 배정", "특정 서브팀 신청 승인", "팀 소개 변경" 등. 멤버 투표 결과는 팀의 공식 입장으로 기록됩니다.

제안 유형은 실제 팀 활동과 연결됩니다. 예산 배정은 보상 트레저리에, 서브팀 결정은 팀 구조에 반영되고, 거버넌스 결과는 팀 타임라인에 남아 누구나 이력을 검증할 수 있습니다.

## 서브팀(Sub-teams)

팀은 그 아래에 **서브팀**을 둘 수 있습니다. 부모 팀(parent) 아래에서 자체 정관(Bylaws)·문서·공지를 가지는 작은 그룹입니다.

서브팀 관련 흐름은 팀 아레나 안에서 전체 라이프사이클을 다룹니다.

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="m9 11 3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg> **서브팀 신청** — `/your-handle/sub-teams/apply`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg> **신청 상태 확인** — `/your-handle/sub-teams/application`
- <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> **하위 서브팀 관리** — `/your-handle/sub-teams/manage`
- <img src={useBaseUrl('/img/icons/file.svg')} width="18" height="18" alt="File" style={{verticalAlign: 'middle'}} /> **서브팀 상세** — `/your-handle/sub-teams/:sub-team-id` (해제(deregister) 흐름 포함)
- <img src={useBaseUrl('/img/icons/file-edit.svg')} width="18" height="18" alt="File edit" style={{verticalAlign: 'middle'}} /> **서브팀 문서** — `/your-handle/sub-teams/docs/...` 에서 공동 문서 작성·편집
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="m3 11 18-5v12L3 14v-3z"/><path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"/></svg> **서브팀 공지** — `/your-handle/sub-teams/announcements/...` 에서 멤버에게 공지 전송
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9 12l2 2 4-4"/><path d="M21 12c0 4.97-4.03 9-9 9s-9-4.03-9-9 4.03-9 9-9c2.39 0 4.68.94 6.36 2.64"/></svg> **정관(Bylaws)** — `/your-handle/bylaws`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg> **부모 팀 탈퇴** — `/your-handle/parent/leave`

전체 흐름 — 신청, 관리, 문서, 공지, 학칙, 등록 해제, 상위팀 이탈 — 을 URL 단위로 다룬 [하위팀](./sub-teams.md) 챕터를 참고하세요.
