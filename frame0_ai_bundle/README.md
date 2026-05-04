# FRAME0 AI Bundle

Date: 2026-05-03

このbundleは、FRAME0をAIエージェントと開発者が扱えるCLIファーストのリアルタイムメディア創作環境として設計するための初期仕様一式である。

## Files

* `FRAME0_AI_SPECIFICATION.md`

  AIエージェント、Runtime、Plugin、C++ SDK Adapter、CLI、Resource Model、Error Model、Security Contractの仕様。

* `FRAME0_DEVELOPMENT_PLAN.md`

  Phase 0からPublic Alphaまでの開発計画。何を先に作り、何を後回しにするかを明示。

* `FRAME0_TODO_CHECKLIST.md`

  実装作業の完全チェックリスト。CLI、schema、runtime、Metal、media IO、plugin host、C++ SDK adapter、AI diagnostics、packagingまでを含む。

## Core Direction

FRAME0はProcessingやopenFrameworksの焼き直しではない。

FRAME0は次のものを一体化する。

* CLI
* Runtime inspection
* Metal render graph
* AVFoundation media IO
* CoreAudio analysis
* C++ SDK Adapter
* Plugin Host
* OS Extension output
* AI controllable execution

最初の実装でGUIを作るべきではない。まずCLI、schema、Resource Registry、Timebase、Mock Device、Metal、Media IO、Plugin Hostを作る。

## Immediate Next Action

最初の作業は`FRAME0_TODO_CHECKLIST.md`の`Brutal Priority List`を上から実行すること。
