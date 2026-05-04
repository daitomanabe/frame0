# FRAME0 開発プラン

Version: 0.1
Date: 2026-05-03
Audience: Core開発者、AIエージェント、SDK Adapter実装者、ランタイム設計者

## 0. 開発方針

FRAME0は最初から巨大に作ると失敗する。失敗パターンは明確である。

* GUIから始める
* Processing風APIから始める
* ベンダーSDKから始める
* Camera Extensionから始める
* Shader playgroundだけを作る
* Plugin systemを後回しにする
* 検査可能性を後回しにする

正しい順番は逆である。

1. CLI
2. Runtime inspection
3. Timebase
4. Resource registry
5. Mock device
6. Metal render graph
7. AVFoundationとCoreAudio
8. Native SDK Adapter
9. Plugin isolation
10. OS Extension output

FRAME0は作品環境ではなく、まず運用可能な実行環境として作る。

## 1. 成功条件

v0.1の成功条件。

* AIがCLIだけでsceneを生成、検査、実行、修正できる
* deviceとpluginをJSONで列挙できる
* mock deviceだけでCIが通る
* macOSでMetal previewとheadless renderが動く
* AVFoundation camera inputがMetal textureへ入る
* audio inputのFFTがrender nodeへ入る
* Native Pluginをout of processで読み込める
* C++ SDK Adapterのサンプルが1つ動く
* event streamがNDJSONで読める
* crashしたpluginがRuntime全体を落とさない

## 2. 失敗条件

次が起きたら設計を止めて戻す。

* Core RuntimeがベンダーSDKのヘッダをincludeし始める
* PluginがCoreの内部型へ依存し始める
* CLIなしでしか操作できない機能が増える
* JSON inspectが古い情報を返す
* Timebaseがrender loopに従属する
* Camera Extensionの中に作品ロジックが入る
* demoは動くがtestがない
* UIでしかデバッグできない

## 3. Repository構成案

```text
frame0/
  apps/
    frame0_cli/
    frame0_runtime/
    frame0_plugin_host/
    frame0_preview_app/
  crates_or_packages/
    frame0_core/
    frame0_graph/
    frame0_time/
    frame0_render/
    frame0_media/
    frame0_audio/
    frame0_device/
    frame0_plugin_api/
    frame0_schema/
    frame0_ai_tools/
  native/
    frame0_plugin_c_api/
    frame0_cpp_sdk/
    adapters/
      mock_video/
      example_vendor/
  extensions/
    camera_extension/
    audio_unit_extension/
  examples/
    hello_shader/
    camera_to_shader/
    audio_reactive/
    native_sdk_input/
  tests/
    unit/
    integration/
    performance/
    plugin_crash/
  docs/
```

言語選定は次の方針。

* Core RuntimeはRustかSwiftのどちらかを選ぶ
* C++ SDK AdapterはC++で書く
* 公開plugin境界はC ABIにする
* Scene APIはTypeScript、Lua、Python、WASMのいずれかを後で追加する
* 最初のv0.1ではScript APIよりmanifest実行を優先する

個人的にはCore RuntimeはRustが強い。理由はC ABI、プロセス分離、CLI、schema、並行処理、memory safetyとの相性が良いからである。ただしApple Frameworkとの接続はSwiftが楽なので、macOS bridgeはSwiftまたはObjective C++を併用する。

## 4. Phase 0 仕様固定

期間目安: 1から2週間

目的: 作り始める前に境界を決める。

Deliverables。

* Architecture Decision Record一式
* Resource schema v0
* Scene manifest schema v0
* Plugin manifest schema v0
* FramePacket schema v0
* AudioPacket schema v0
* Error schema v0
* CLI command map v0
* Test strategy v0

Decision points。

* Core言語
* IPC方式
* Plugin ABI方式
* macOS bridge方式
* JSON Schema管理方法
* CI環境

この段階でGUIの話をしない。GUIの話は逃げ道になる。

## 5. Phase 1 CLIとSchema

期間目安: 2週間

目的: AIと人間が同じCLIで扱える土台を作る。

Deliverables。

```bash
frame0 --version
frame0 new
frame0 inspect
frame0 graph
frame0 devices list
frame0 plugins list
frame0 doctor
```

必須仕様。

* すべてのコマンドが`--json`を持つ
* schema validationを実装する
* errorを構造化する
* dry runを実装する
* NDJSON event streamの雛形を作る

Acceptance。

```bash
frame0 inspect examples/hello_shader/scene.yaml --json
frame0 graph examples/hello_shader/scene.yaml --json
frame0 doctor --json
```

これがCIで通る。

## 6. Phase 2 Core RuntimeとResource Registry

期間目安: 3週間

目的: Runtimeを検査可能なResource集合として動かす。

Deliverables。

* Resource Registry
* Runtime Supervisor
* Node lifecycle
* Plugin lifecycle stub
* Event bus
* Structured logging
* Runtime state snapshot

Resource state。

```text
created
resolved
opening
active
paused
failed
closed
```

Acceptance。

* sceneをloadできる
* resource dependencyを解決できる
* lifecycle eventがNDJSONで出る
* failed resourceが原因つきで出る
* AIがJSONだけで状態を読める

## 7. Phase 3 Timebase

期間目安: 2週間

目的: FRAME0を描画loop依存にしない。

Deliverables。

* Monotonic clock
* Manual clock
* Audio clock stub
* Frame scheduler
* Timestamp conversion
* Drift reporting
* Fixed timestep simulation mode

Acceptance。

* headlessで1000 frameをdeterministicにrenderできる
* manual clockで同じ結果が再現する
* dropped frameとlate frameをevent streamに出せる

## 8. Phase 4 Metal Render Graph

期間目安: 4週間

目的: Metalを中心にGPU graphを成立させる。

Deliverables。

* Metal device management
* Render graph
* Texture pool
* Shader compilation
* Fullscreen pass
* Compute pass
* Offscreen target
* Window preview
* Headless render
* GPU timing

Acceptance。

* shader nodeが動く
* texture poolが再利用される
* GPU frame timeが取れる
* 1920x1080 60fpsの基本sceneが動く
* headlessでmovieまたはimage sequenceへ書き出せる準備がある

この段階では凝ったcreative APIはいらない。必要なのはrender graphの堅さである。

## 9. Phase 5 Media Foundation

期間目安: 4週間

目的: AVFoundation、Core Video、CoreAudioをResource化する。

Deliverables。

* AVFoundation camera input
* CVPixelBuffer to Metal Texture bridge
* basic movie input
* basic movie outputまたはimage sequence output
* CoreAudio input
* FFT node
* audio level node
* audio clock接続

Acceptance。

* camera inputをMetal shaderへ渡せる
* audio FFT値をshader parameterに渡せる
* `frame0 devices list --json`でAVFoundation deviceが出る
* format negotiation failureが構造化errorになる

## 10. Phase 6 Mock SDK Adapter

期間目安: 2週間

目的: C++ SDK Adapter構造を実機なしで固定する。

Deliverables。

* C ABI header
* C++ adapter base class
* mock video SDK
* mock audio SDK
* plugin hostからのload
* plugin crash test
* device discovery
* stream start and stop

Acceptance。

* Native Pluginをout of processで読み込める
* mock video frameがRuntimeへ入る
* plugin crashでRuntimeが落ちない
* plugin restart policyが動く
* memory leak testの土台がある

このphaseがないと、BlackmagicやAJAなどの実SDKで設計が歪む。

## 11. Phase 7 Real C++ SDK Adapter Sample

期間目安: 3から5週間

目的: 現実のC++ SDKを1つ接続し、抽象化の弱点を潰す。

サンプル候補。

* Blackmagic DeckLink SDK
* Intel RealSense SDK
* NDI SDK
* AJA SDK
* libfreenect系

選定基準。

* callback型のstreamを持つ
* device discoveryがある
* format negotiationがある
* metadataがある
* failure modeが多い

Blackmagicを使う場合の扱い。

```text
DeckLink SDK
  ↓
frame0 native adapter
  ↓
C ABI
  ↓
Plugin Host
  ↓
VideoFrame Resource
  ↓
Metal Texture
```

Acceptance。

* device listが出る
* mode listが出る
* capture開始停止ができる
* frame timestampが出る
* format mismatch errorが出る
* hot unplug相当のエラーを扱える

## 12. Phase 8 Plugin IsolationとIPC

期間目安: 4週間

目的: Pluginを安全に壊せる構造にする。

Deliverables。

* Plugin Host process
* IPC protocol
* shared memoryまたはIOSurface handle transport
* lifecycle supervisor
* crash recovery
* permission model
* signature check placeholder

macOSではXPCを優先候補にする。AppleのXPCは軽量なプロセス間通信として使える。Pluginが不安定な場合、process分離は安定性と権限分離に効く。

Acceptance。

* Plugin HostをkillしてもRuntimeは落ちない
* Runtimeがplugin failed eventを出す
* 再起動policyが動く
* Resource ownershipが壊れない

## 13. Phase 9 Output Adapters

期間目安: 4週間

目的: FRAME0の出力を現場へ流せるようにする。

Deliverables。

* screen output
* file output
* image sequence output
* Syphon output候補
* NDI output候補
* Core Media I/O Camera Extension prototype

優先順位。

1. screen
2. fileまたはimage sequence
3. Syphon
4. NDI
5. Core Media I/O Camera Extension

Camera Extensionは早く触りたくなるが、最初にやると署名、配布、権限、OS更新で時間を失う。Core Runtimeが固まってから出力adapterとして作る。

Acceptance。

* Render outputを複数出力へ同時に流せる
* output format変換がgraph上で見える
* virtual cameraはCoreに依存しないadapterとして動く

## 14. Phase 10 AI Integration

期間目安: 3週間

目的: AIがFRAME0を操作しやすい状態にする。

Deliverables。

* AI command guide
* JSON Schema export
* `frame0 explain error`
* `frame0 suggest fix`
* Runtime snapshot
* graph diff
* scene patch
* parameter patch
* event summarizer

Commands。

```bash
frame0 schema export --format json
frame0 explain error error.json --json
frame0 snapshot runtime --json
frame0 graph diff before.json after.json --json
frame0 scene patch scene.yaml patch.json
```

Acceptance。

* AIがsceneを生成してinspectできる
* errorから修正候補を返せる
* graph差分を出せる
* runtime snapshotから原因分析できる

## 15. Phase 11 Creative API

期間目安: 4週間以降

目的: Processing的な使いやすさを後から追加する。

候補。

* TypeScript scene API
* Lua quick scripting
* Python control API
* MSL shader library
* node template system
* live reload

注意点。

Creative APIを先に作ると、基盤が弱いまま表層だけ良くなる。FRAME0の価値は表層の書き味ではなく、時間、IO、拡張、AI制御の一貫性にある。

## 16. Phase 12 DistributionとDeveloper Experience

期間目安: 継続

Deliverables。

* installer
* signed plugins
* plugin registry
* docs site
* examples
* templates
* CI release
* crash report
* benchmark report

重要なDX。

```bash
frame0 doctor
frame0 plugins verify
frame0 examples run camera_to_shader
frame0 benchmark examples/audio_reactive
```

## 17. Performance Budget

初期budget。

```text
1080p60 basic shader scene
  CPU main thread under 4ms
  GPU under 8ms
  frame drop under 1 percent
  audio xrun zero during 10 minute test

4K60 media shader scene
  GPU under 14ms
  texture allocation stable after warmup
  no unbounded memory growth
```

測らない最適化はやらない。特にzero copyは実測して採用する。

## 18. Risk Register

| Risk | Impact | Countermeasure |
| --- | --- | --- |
| C++ SDKごとの癖でCoreが汚れる | High | C ABIとAdapter層で隔離 |
| OS Extension対応が重い | High | Output adapterとして後回し |
| GUI開発に時間を吸われる | High | CLIとheadlessを優先 |
| AIが状態を読めない | High | JSONとNDJSONを最初から実装 |
| Timebaseが破綻する | High | Phase 3で独立実装 |
| GPU resource leak | High | Texture poolとlifetime tracking |
| Plugin crashでRuntimeが落ちる | High | out of process host |
| 実機依存でCI不能 | High | mock devices必須 |
| Creative APIが早すぎる | Medium | manifest実行を先に作る |

## 19. Milestone Map

```text
M0 Specification locked
M1 CLI and schema running
M2 Resource registry running
M3 Deterministic timebase running
M4 Metal graph running
M5 Camera and audio input running
M6 Mock native plugin running
M7 Real SDK adapter running
M8 Plugin isolation stable
M9 Output adapters running
M10 AI integration usable
M11 Creative API preview
M12 Public alpha
```

## 20. 最初の90日で作るもの

90日で作るべき範囲。

* CLI
* schema
* runtime registry
* timebase
* Metal graph
* AVFoundation camera
* CoreAudio FFT
* mock native plugin
* C ABI
* event stream
* JSON inspect
* 3つのexample

90日で作らないもの。

* GUI editor
* marketplace
* 完全なCamera Extension
* すべてのSDK対応
* ノードエディタ
* AI自律制作機能
* Web UI

ここを間違えると、また中途半端なcreative toolになる。

## 21. 参考資料

* Apple Core Media I/O Camera Extension: https://developer.apple.com/documentation/coremediaio/creating-a-camera-extension-with-core-media-i-o
* Apple WWDC22 Core Media IO Camera Extensions: https://developer.apple.com/videos/play/wwdc2022/10022/
* Apple Metal: https://developer.apple.com/documentation/metal
* Apple AVFoundation: https://developer.apple.com/documentation/avfoundation/
* Apple Audio Unit v3 Plug Ins: https://developer.apple.com/documentation/audiotoolbox/audio-unit-v3-plug-ins
* Apple IOSurface: https://developer.apple.com/documentation/iosurface
* Apple XPC: https://developer.apple.com/documentation/xpc
* Apple DriverKit: https://developer.apple.com/documentation/driverkit
* Blackmagic Camera REST API developer page: https://www.blackmagicdesign.com/developer/products/camera/sdk-and-software
