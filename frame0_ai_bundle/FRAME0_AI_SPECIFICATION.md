# FRAME0 AI仕様書

Version: 0.1
Date: 2026-05-03
Audience: AIエージェント、開発者、プラグイン実装者、ランタイム設計者

## 0. 一文定義

FRAME0は、リアルタイム映像、音声、センサー、C++ SDK、OS Extension、AIエージェントを同じ創作ランタイム資源として扱う、CLIファーストのクリエイティブ実行環境である。

ProcessingやopenFrameworksの単なる再実装ではない。描画APIではなく、時間、入出力、GPU共有、拡張、検査、再現性を扱う創作インフラである。

## 1. AIが守るべき基本方針

AIはFRAME0を「コード生成対象」ではなく「検査可能な実行環境」として扱う。

AIの主な役割は次の通り。

* 作品のscene定義を書く
* Node Graphを組む
* CLIを使って実行、検査、録画、プロファイルを行う
* Plugin manifestを生成する
* C++ SDK Adapterの雛形を生成する
* エラー、ログ、device capabilityから修正案を作る
* Runtimeの状態をJSONで読み、差分パッチとして変更する

AIがやってはいけないこと。

* OSドライバ層に作品ロジックを書くこと
* ベンダーSDKの型をFRAME0 Core APIへ直接漏らすこと
* C++ ABIを安定APIとして公開すること
* UI前提でしか動かない作品を生成すること
* 実行結果を検査せずに「動くはず」と判断すること
* カメラ、マイク、ネットワーク、外部デバイスの権限を暗黙に要求すること

## 2. FRAME0の中核思想

FRAME0の核は次の5つである。

1. CLI first
2. Inspectable runtime
3. Device capability model
4. GPU native media graph
5. AI controllable execution

つまり、FRAME0の最小単位はアプリ画面ではない。最小単位は「実行可能で検査可能なscene」である。

```text
scene
  inputs
  nodes
  parameters
  outputs
  clocks
  resources
  permissions
```

## 3. 全体アーキテクチャ

```text
frame0 CLI
  ↓
Runtime Supervisor
  ↓
Core Runtime
  ↓
Graph Engine
  ↓
Timebase System
  ↓
Resource Registry
  ↓
Render Engine / Media Engine / Device Engine
  ↓
Output Adapters
```

プロセス分離は次の構造を基本にする。

```text
frame0 command
  ↓
frame0 runtime process
  ↓
plugin host process
  ↓
native SDK adapter process
  ↓
OS extension process when required
```

Coreにすべてを詰め込まない。C++ SDK、OS Extension、AI Worker、録画、外部デバイス制御は壊れる可能性がある。壊れる可能性があるものは分離する。

## 4. OS前提

FRAME0はmacOSでは次の基盤を優先する。

* Metal
* AVFoundation
* Core Media
* Core Video
* IOSurface
* XPC
* Core Media I/O Camera Extension
* Audio Unit v3
* DriverKit
* System Extensions

Core Media I/O Camera ExtensionはmacOS 12.3以降のCamera Extensionを前提にし、旧DAL Pluginを主軸にしない。AppleはCore Media I/O extensionsをlegacy DAL Pluginの置き換えとして説明している。

MetalはGPU直アクセスとcomputeを担当する。AVFoundationは時間ベースの映像音声メディアを扱う。IOSurfaceはプロセス境界を越えたframebuffer共有に使う。XPCはプロセス分離とPlugin Host連携に使う。DriverKitは本当に必要なハードウェアドライバに限定する。

## 5. Resource Model

FRAME0ではすべてをResourceとして扱う。

```text
Resource
  Device
  Stream
  Buffer
  Texture
  AudioBus
  Timebase
  Node
  Scene
  Plugin
  Extension
  Command
```

Resourceは必ずID、型、capability、状態、権限、所有processを持つ。

```json
{
  "id": "device.camera.decklink.0",
  "type": "device.video_input",
  "vendor": "example_vendor",
  "capabilities": [
    "video.input",
    "audio.input",
    "timecode.input",
    "format.autodetect"
  ],
  "status": "available",
  "process": "plugin_host.blackmagic",
  "permissions": [
    "camera",
    "microphone"
  ]
}
```

AIはResourceを名前で推測してはいけない。必ず`frame0 inspect resources --json`か`frame0 devices list --json`で確認する。

## 6. Capability Model

ベンダーSDKごとの固有性はcapabilityとして表現する。

代表capability。

```text
video.input
video.output
audio.input
audio.output
timecode.input
timecode.output
genlock.input
midi.input
osc.input
dmx.output
artnet.output
hid.input
serial.io
camera.control
lens.control
record.control
storage.media
gpu.texture.export
gpu.texture.import
os.camera_extension
os.audio_unit
os.driverkit
```

ベンダー固有フィールドは`vendor_properties`へ逃がす。Core APIへ直書きしない。

```json
{
  "capabilities": ["video.input", "timecode.input"],
  "vendor_properties": {
    "supports_sub_devices": true,
    "supports_internal_keying": true
  }
}
```

## 7. C++ SDK Adapter仕様

### 7.1 原則

C++ SDKはFRAME0 Coreから直接呼ばない。必ずNative SDK Adapterで包む。

```text
Vendor C++ SDK
  ↓
Native Adapter
  ↓
Stable C ABI
  ↓
FRAME0 Plugin Host
  ↓
Resource Registry
  ↓
Runtime Graph
```

C++ ABIを公開境界にしない。C++コンパイラ、標準ライブラリ、例外、RTTI、メモリ管理の違いで壊れる。公開境界はC ABIにする。

### 7.2 Adapterの責務

Native SDK Adapterは次を担当する。

* ベンダーSDKの初期化と終了処理
* デバイス列挙
* mode列挙
* capture開始と停止
* callbackからFramePacketへの変換
* audio packet変換
* timecode変換
* エラー正規化
* thread境界の整理
* メモリ所有権の明示
* vendor specific controlのJSON RPC化

### 7.3 Adapterがやってはいけないこと

* scene logicを持つ
* shaderを勝手に適用する
* UIを出す
* runtime clockを勝手に決める
* Core Runtimeを直接操作する
* 他PluginのResourceを直接参照する

### 7.4 C ABIの基本形

```c
#define FRAME0_PLUGIN_API_VERSION 1

typedef struct frame0_plugin_context frame0_plugin_context;
typedef struct frame0_device_handle frame0_device_handle;
typedef struct frame0_stream_handle frame0_stream_handle;

typedef enum frame0_result {
    FRAME0_OK = 0,
    FRAME0_ERROR_UNKNOWN = 1,
    FRAME0_ERROR_UNSUPPORTED = 2,
    FRAME0_ERROR_DEVICE_BUSY = 3,
    FRAME0_ERROR_PERMISSION_DENIED = 4,
    FRAME0_ERROR_INVALID_ARGUMENT = 5
} frame0_result;

typedef struct frame0_plugin_descriptor {
    uint32_t api_version;
    const char* plugin_id;
    const char* plugin_name;
    const char* plugin_version;
} frame0_plugin_descriptor;

FRAME0_EXPORT frame0_result frame0_plugin_get_descriptor(frame0_plugin_descriptor* out_descriptor);
FRAME0_EXPORT frame0_result frame0_plugin_initialize(frame0_plugin_context* context);
FRAME0_EXPORT frame0_result frame0_plugin_shutdown(void);
```

C++ wrapperはこのC ABIの上に作る。

```cpp
class VideoInputAdapter {
public:
    virtual Result open(const DeviceId& id) = 0;
    virtual Result start(const CaptureConfig& config) = 0;
    virtual Result stop() = 0;
    virtual Result close() = 0;
    virtual ~VideoInputAdapter() = default;
};
```

## 8. FramePacket仕様

```json
{
  "type": "video.frame",
  "stream_id": "stream.camera.main.video",
  "pts_ns": 1234567890,
  "duration_ns": 16666667,
  "frame_index": 120,
  "width": 1920,
  "height": 1080,
  "pixel_format": "uyvy422",
  "color_space": "rec709",
  "transfer_function": "gamma24",
  "range": "video",
  "field_order": "progressive",
  "storage": {
    "type": "iosurface",
    "handle": "opaque"
  }
}
```

FramePacketは映像本体とmetadataを混ぜない。映像本体はBuffer、CVPixelBuffer、IOSurface、Metal Texture、外部handleのいずれかで保持する。

## 9. Storage Model

```text
CPUBuffer
CVPixelBuffer
IOSurface
MetalTexture
ExternalHandle
CompressedPacket
```

優先順位。

1. IOSurfaceまたはCVPixelBufferからMetal Textureへ変換できる経路
2. GPU uploadが1回で済む経路
3. CPU copyが必要な経路
4. format変換が必要な経路

FRAME0はzero copyを宗教化しない。現場で壊れるのは過剰なzero copy信仰である。性能と安定性の両方を測る。

## 10. Timebase仕様

FRAME0は描画フレームワークではなく時間制御フレームワークである。

Timebaseは次を扱う。

```text
audio clock
video clock
external clock
wall clock
beat clock
timecode clock
manual clock
simulation clock
```

すべてのFramePacket、AudioPacket、EventPacketはtimestampを持つ。

```json
{
  "clock": "audio.input.0",
  "pts_ns": 2400000000,
  "duration_ns": 5333333,
  "rate": 48000
}
```

AIは「60fpsだから16.67ms」と固定してはいけない。必ずsceneのclock policyを読む。

## 11. Scene Manifest仕様

```yaml
name: sample_live_scene
version: 0.1
runtime: frame0

permissions:
  camera: true
  microphone: true
  network: true

clock:
  primary: audio.input.main
  fallback: monotonic

inputs:
  camera_a:
    type: device.video_input
    selector:
      capability: video.input
      vendor: any
    mode: 1080p5994
    pixel_format: auto

  audio_main:
    type: device.audio_input
    selector:
      capability: audio.input

nodes:
  camera_texture:
    type: media.texture_from_video
    input: camera_a.video

  reactive_shader:
    type: render.shader
    shader: shaders/reactive.msl
    inputs:
      image: camera_texture.texture
      audio: audio_main.fft

outputs:
  preview:
    type: screen
    input: reactive_shader.output

  virtual_camera:
    type: os.camera_extension
    input: reactive_shader.output
```

## 12. CLI仕様

CLIは人間用とAI用を分けない。すべてのコマンドは`--json`を持つ。

基本コマンド。

```bash
frame0 new my_scene
frame0 run scene.yaml
frame0 render scene.yaml --duration 60 --out movie.mov
frame0 inspect scene.yaml --json
frame0 graph scene.yaml --json
frame0 devices list --json
frame0 devices modes device.camera.0 --json
frame0 plugins list --json
frame0 plugins inspect io.frame0.example --json
frame0 doctor --json
frame0 benchmark scene.yaml --json
frame0 record start --scene scene.yaml --out take.mov
frame0 resource get resource_id --json
frame0 resource set node.reactive_shader.params.intensity 0.8
```

AI向けにはNDJSON event streamを用意する。

```bash
frame0 run scene.yaml --events ndjson
```

出力例。

```json
{"event":"runtime.started","time":"2026-05-03T00:00:00Z"}
{"event":"device.opened","id":"device.camera.0"}
{"event":"frame.rendered","frame":1,"gpu_ms":2.4}
{"event":"warning","code":"FRAME_DROP","count":3}
```

## 13. AI Tool Contract

AIはFRAME0に対して次の順で操作する。

1. 目的をscene manifestへ落とす
2. `frame0 inspect`で静的検査
3. `frame0 devices list --json`で環境検査
4. 必要Resourceが存在するか確認
5. `frame0 run --dry-run --json`で解決検査
6. 実行
7. event streamを読み、エラーを分類
8. 変更はpatchとして適用

AIが実行時に見るべき最重要情報。

```text
resource status
capability mismatch
permission missing
format negotiation failure
timebase drift
dropped frame count
gpu frame time
audio xrun count
plugin crash count
memory pressure
```

## 14. Error Model

エラーは人間向け文字列だけで返さない。必ず構造化する。

```json
{
  "error": {
    "code": "FORMAT_NEGOTIATION_FAILED",
    "severity": "error",
    "resource": "device.camera.0",
    "message": "Requested 2160p5994 is not supported by this device",
    "suggestions": [
      "Run frame0 devices modes device.camera.0 --json",
      "Use mode 1080p5994"
    ]
  }
}
```

AIは`message`だけで判断しない。`code`と`resource`を使う。

## 15. Plugin Manifest仕様

```yaml
plugin:
  id: io.frame0.vendor.sample
  version: 0.1.0
  type: native
  entry:
    macos_arm64: libframe0_vendor_sample.dylib
  api_version: 1
  capabilities:
    * video.input
    * audio.input
    * timecode.input
  isolation:
    process: separate
    restart_policy: on_crash
  permissions:
    camera: true
    microphone: true
  vendor:
    sdk_name: Example Vendor SDK
    sdk_version: unknown
```

注意: YAMLの`*`は実際のYAMLではalias記号として扱われるため、実装時は通常の配列表記を使う。ここではMarkdown上の概念表現である。

実際のmanifest例。

```yaml
plugin:
  id: io.frame0.vendor.sample
  version: 0.1.0
  type: native
  entry:
    macos_arm64: libframe0_vendor_sample.dylib
  api_version: 1
  capabilities:
    - video.input
    - audio.input
    - timecode.input
  isolation:
    process: separate
    restart_policy: on_crash
  permissions:
    camera: true
    microphone: true
```

## 16. Plugin Isolation

Native pluginは標準ではout of processで動かす。

```text
Core Runtime
  ↓ XPC or local IPC
Plugin Host
  ↓ C ABI
Native Plugin
  ↓
Vendor SDK
```

例外的にin processを許可する条件。

* 署名済み
* API version一致
* crash復旧不要
* 実測でIPC overheadが問題
* デバッグモードではない

初期実装はout of processを基本にする。最初からin process最適化へ行くのは設計逃避である。

## 17. Rendering Contract

Render Nodeは次を満たす。

* 入力Textureのformatを宣言する
* 出力Textureのformatを宣言する
* HDR、SDR、alpha、color spaceを明示する
* GPU resource lifetimeをRuntimeに委譲する
* 自前で無限にtextureを確保しない
* CPU readbackを明示的に要求する

```json
{
  "node": "render.shader",
  "inputs": [
    {"name":"image","type":"texture2d","format":"rgba16float"}
  ],
  "outputs": [
    {"name":"out","type":"texture2d","format":"rgba16float"}
  ],
  "requires": ["metal.compute"]
}
```

## 18. AIが生成するコードの基準

AIがコードを書くときは次を守る。

* 先にmanifestを書く
* 次にCLIで検査できるschemaを書く
* C++はCore APIを直接含めず、adapter境界を明示する
* エラーはenumと構造体で返す
* callbackでは重い処理をしない
* lock順序を明記する
* texture lifetimeを明記する
* unit testを同時に生成する
* mock deviceを先に作る

## 19. SDK Adapterサンプル方針

Blackmagic、AJA、RealSense、NDI、DMX、MIDI、OSCなどは、すべて同じ抽象化で扱う。

```text
Vendor SDK Adapter
  device discovery
  capability extraction
  stream negotiation
  packet conversion
  control plane
  health reporting
```

AIはベンダー名をCore型名に入れない。

悪い例。

```cpp
class BlackmagicFrame0InputDevice;
```

よい例。

```cpp
class NativeVideoInputDevice;
```

ベンダー名はplugin IDとvendor metadataに閉じ込める。

## 20. Security Contract

FRAME0は創作環境だが、カメラ、マイク、ネットワーク、外部デバイスに触る。安全設計を後回しにすると配布時に死ぬ。

必須事項。

* plugin署名
* plugin permission manifest
* runtime permission prompt
* sandbox可能なprocess分離
* network accessの明示
* file accessの明示
* cameraとmicrophone権限の明示
* crashしたpluginの自動隔離
* unknown pluginの警告
* AIが勝手に外部送信しないpolicy

## 21. Testing Contract

最初からmock deviceを作る。

```text
mock.video_input.colorbars
mock.video_input.noise
mock.audio_input.sine
mock.timecode.generator
mock.midi.generator
mock.osc.generator
mock.plugin.crasher
mock.plugin.slow_frame
```

AIは実機なしで大半のテストを走らせるべきである。

## 22. Acceptance Criteria

FRAME0 v0.1は次を満たす。

* CLIでsceneを起動できる
* JSONでdevice、plugin、graph、resourceを検査できる
* Metalでwindow描画できる
* headless renderができる
* AVFoundation camera inputが使える
* audio inputからFFT値を取れる
* Native Pluginをout of processで読み込める
* mock C++ SDK Adapterが動く
* FramePacketがtimestampとformat metadataを持つ
* event streamがNDJSONで出る
* AIがエラーを読んで修正できる構造を持つ

## 23. 参考資料

* Apple Core Media I/O Camera Extension: https://developer.apple.com/documentation/coremediaio/creating-a-camera-extension-with-core-media-i-o
* Apple WWDC22 Core Media IO Camera Extensions: https://developer.apple.com/videos/play/wwdc2022/10022/
* Apple Metal: https://developer.apple.com/documentation/metal
* Apple AVFoundation: https://developer.apple.com/documentation/avfoundation/
* Apple Audio Unit v3 Plug Ins: https://developer.apple.com/documentation/audiotoolbox/audio-unit-v3-plug-ins
* Apple IOSurface: https://developer.apple.com/documentation/iosurface
* Apple XPC: https://developer.apple.com/documentation/xpc
* Apple DriverKit: https://developer.apple.com/documentation/driverkit
* Blackmagic Camera REST API developer page: https://www.blackmagicdesign.com/developer/products/camera/sdk-and-software
