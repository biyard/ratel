---
sidebar_position: 2
title: Fact or Fold
---

# Fact or Fold

A 3-minute game where four people gather to judge whether a news headline is real or fake. You read the headline and decide if it's true or false. You put chips down on your call and write one line saying why. You read the others' reasoning, debate for about a minute, and then make your final call. After that you check the result.

```
/arcade/games/fact-or-fold
```

## Your first round, start to finish

Here's what the next three minutes look like after you hit **Play**.

### 1. The lobby

The moment you hit Play the matchmaker takes you to a lobby for whichever news headline is currently active. You wait there while the other players show up.

The round starts the moment the fourth seat fills. No countdown. No "ready up" button.

### 2. The news appears — 30 seconds

The headline shows up along with a short body excerpt and the source label. Everyone sees the same screen at the same time. You have 30 seconds to make a gut call.

You can't search. This is a step where you rely on your instinct and what's on the screen.

### 3. The bet — 10 seconds

A small panel opens with two buttons — REAL and FAKE. Between them sits a slider in the **100–1,000 RP** range. Pick a side, stake an amount you can comfortably lose, and submit.

Once you submit both your side and your amount are **locked**. You can't change your mind in the next stage. That comes later in a separate flip slot.

If you don't submit in time the round auto-forfeits your seat. Your stake comes back but you're out of this round. Don't overthink the bet. The 10 seconds are short on purpose.

### 4. Writing your rationale — 30 seconds

A text box opens for you to write your rationale. Type *why* you bet the way you did.

Write this as if you're trying to persuade the other players. It helps to picture explaining your judgment to one friend.

Submit only goes through **once**. You can't rewrite it. If you don't submit, whatever's in the box at the deadline goes in as is.

### 5. Reading what the others wrote — 20 seconds

All four rationales appear at the same time, including yours. The author order is randomized so submitting last doesn't give you an information edge.

Read each one carefully. Does anyone's logic shake your call? If so, hold their name in mind for a second. You may need it in the next stage.

### 6. Debate — 70 seconds

Free chat opens. Each message can be up to 80 characters. Push back, agree, ask questions — all fair game.

**The flip slot opens in the final 10 seconds.** This is your one chance to switch sides. If someone's rationale flipped you from REAL to FAKE (or the other way around) click their card and confirm. You **must** cite a specific player. A flip without a cite is rejected. Your original side stays the way it was.

You only get one flip per round. Use it carefully or don't use it at all.

### 7. Settlement — automatic

The truth is revealed. The screen shows the following.

- Whether the headline was **REAL** or **FAKE**.
- Whether you won or lost.
- A breakdown of your chip delta.

The breakdown is broken out line by line.

| Line item | When you earn it |
|---|---|
| **Base refund** | You won — your stake comes back as is. |
| **Correct-side bonus** | You won — extra chips on top, +60% of your stake by default. |
| **Pool share** | You won — a slice of the losing side's pool, proportional to your stake. |
| **Influence bonus** | Another player cited your rationale when they flipped and they ended up on the winning side. The bigger their stake, the bigger your bonus. |
| **Insider bonus** | You were the round's insider and you bet the truth side. The next section covers this. |

You'll see exactly which line item paid you what. If you lost, your chips_out is 0. You never go negative.

Settlement runs in the background. It lands in your wallet within a few seconds. There's no separate claim button.

## Two flavors of round: regular vs insider

Most of the time you'll be one of three **regular players**. You have to decide REAL or FAKE using only the headline and your own judgment. No private clues.

Every round one of the four players is secretly the insider.

### When you're the insider

A panel only you can see appears at the start of the round. It carries the **truth statement**. The statement tells you whether the news is REAL or FAKE and includes one sentence of context. Something like "BOK actually cut by 0.25%, not 0.5%."

Your role shifts a little.

- The flow of betting, writing, and debating stays the same.
- Your rationale should *nudge the others toward the truth without revealing that you know it*.
- If you bet the truth side and win, you receive a **+50% bonus** on top of your stake refund.
- Keep your insider role to yourself for the entire round.

### When you're a regular player

The other three are all guessing. You don't know which of them is the insider. One of them definitely is. Read the rationales carefully. The insider won't tell you the truth outright. Their reasoning still tends to lean slightly toward the correct answer in a way the other two don't.

## Things to know before you queue

A few notes on what happens when you hit **Play**.

### Only one headline is active at a time

A small team curates the news headlines you'll play on. Each headline has a defined active window. It's usually a few hours wide. When you queue the lobby picks whichever headline is currently inside its window. When that window closes the next headline rotates in automatically.

If no headline is in its window right now, a notice like *"The next round opens at hh:mm"* will appear on the page. This notice is coming in a future update.

### You can only play each headline once

Once you've finished a round on a given headline you can't queue for that same headline again. "Finished" means settled — not just joined. The lobby will pick a different one for you, or tell you to wait if it was the only one available.

Most of the time you won't even notice this rule. Queue, play, queue again later, and you'll usually find a different headline waiting. If you queue back-to-back quickly the page may show *"You already played this round. Try again when the next subject is active."*

### Mid-round queue attempts are blocked

If you're already in a round that hasn't finished and try to queue for another one, a *"You're still in a round"* message blocks the attempt. This often happens after you closed the tab and forgot, or your phone died. Go back to the in-progress round at its URL, or wait for it to settle.

## When things go wrong

- **You drop offline mid-round.** Reconnect within 90 seconds. If you make it back in time you pick up where you left off. After that the round auto-forfeits your seat. Your stake comes back but you're out of any bonuses or pool shares for that round.
- **You want to leave a lobby that hasn't started.** Just hit Leave. Your seat clears and your chips come right back. You can queue again right away.
- **You want to leave a round that has already started.** You can't formally leave a round in progress. Your bet stands and your seat stays. The fastest way out is to let the rest of the stages run and accept the settlement.

## Leaderboard

:::note Coming soon
Two leaderboards are on the way.

- **Arcade leaderboard** — a unified board that aggregates your activity across every game in the Arcade. Details will land on the [Arcade overview](./) once it ships.
- **Fact or Fold leaderboard** — a game-specific board that ranks you by accuracy and chip delta within Fact or Fold itself.

Neither board ships with v1. We'll add details on this page once a release date is set.
:::

## Where to go next

- **[Arcade overview](./)** — the chip wallet and the infrastructure that Arcade games share.
- **[Reward Points](/rewards/points)** — the RP you'll convert into chips before your first round.
