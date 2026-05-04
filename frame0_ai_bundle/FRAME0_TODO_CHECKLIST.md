# FRAME0 完全TODOチェックリスト

Version: 0.1
Date: 2026-05-03
Audience: 開発者、AIエージェント、プロジェクト管理者

このTODOは単なる作業リストではない。FRAME0を「CLI制御可能なリアルタイムメディア創作インフラ」として成立させるための検査表である。

## 0. Project Foundation

* [ ] Repository名を決める
* [ ] Licenseを決める
* [ ] READMEの一文定義を書く
* [ ] Architecture Decision Recordの置き場を作る
* [ ] Issue templateを作る
* [ ] Pull request templateを作る
* [ ] Coding standardを決める
* [ ] Formatting toolを決める
* [ ] CIを作る
* [ ] macOS runner方針を決める
* [ ] nightly build方針を決める
* [ ] release tag規約を決める
* [ ] versioning規約を決める
* [ ] plugin API versioning規約を決める
* [ ] schema versioning規約を決める

## 1. Architecture Decisions

* [ ] Core Runtime言語を決める
* [ ] CLI実装言語を決める
* [ ] macOS bridge実装方式を決める
* [ ] C++ SDK Adapter方針を決める
* [ ] C ABIを公開境界にすることを決定する
* [ ] C++ ABIを公開しないことを明文化する
* [ ] Plugin isolationを標準にすることを決定する
* [ ] RuntimeとPlugin Hostの責務分担を書く
* [ ] Resource Registryの責務を書く
* [ ] Timebaseをrender loopから独立させることを決定する
* [ ] GUIをv0.1対象外にすることを決定する
* [ ] Camera ExtensionをCoreではなくOutput Adapterにすることを決定する
* [ ] Native SDKをCoreへ直結しないことを決定する

## 2. Schema

* [ ] Scene manifest schemaを作る
* [ ] Plugin manifest schemaを作る
* [ ] Resource schemaを作る
* [ ] Device schemaを作る
* [ ] Capability schemaを作る
* [ ] FramePacket schemaを作る
* [ ] AudioPacket schemaを作る
* [ ] EventPacket schemaを作る
* [ ] Error schemaを作る
* [ ] Runtime snapshot schemaを作る
* [ ] Graph schemaを作る
* [ ] Parameter schemaを作る
* [ ] Permission schemaを作る
* [ ] JSON Schemaとしてexportできるようにする
* [ ] schema validation testを書く
* [ ] invalid sceneのtestを書く
* [ ] schema migration方針を書く

## 3. CLI Core

* [ ] `frame0 --version`を実装する
* [ ] `frame0 new`を実装する
* [ ] `frame0 inspect`を実装する
* [ ] `frame0 graph`を実装する
* [ ] `frame0 run`を実装する
* [ ] `frame0 run --dry-run`を実装する
* [ ] `frame0 devices list`を実装する
* [ ] `frame0 devices modes`を実装する
* [ ] `frame0 plugins list`を実装する
* [ ] `frame0 plugins inspect`を実装する
* [ ] `frame0 resources list`を実装する
* [ ] `frame0 resource get`を実装する
* [ ] `frame0 resource set`を実装する
* [ ] `frame0 doctor`を実装する
* [ ] `frame0 schema export`を実装する
* [ ] 全コマンドへ`--json`を追加する
* [ ] exit code規約を作る
* [ ] stderrとstdoutの出し分けを決める
* [ ] human readable outputとJSON outputを分離する
* [ ] CLI snapshot testを書く

## 4. AI Operation Support

* [ ] AI向けCLI利用規約を書く
* [ ] JSON only modeを作る
* [ ] NDJSON event streamを作る
* [ ] `frame0 snapshot runtime --json`を作る
* [ ] `frame0 explain error --json`を作る
* [ ] `frame0 graph diff --json`を作る
* [ ] `frame0 scene patch`を作る
* [ ] `frame0 suggest fix`のstubを作る
* [ ] error codeからsuggestionを返す仕組みを作る
* [ ] AIが読むべき最小状態を定義する
* [ ] AIが触ってはいけない状態を定義する
* [ ] AI用のsample taskを作る
* [ ] AIが生成したsceneのvalidation testを作る

## 5. Runtime Supervisor

* [ ] Runtime processを作る
* [ ] Runtime lifecycleを定義する
* [ ] Runtime configを読む
* [ ] Sceneをloadする
* [ ] Sceneをvalidateする
* [ ] Resource dependencyを解決する
* [ ] Resource Registryを初期化する
* [ ] Event busを初期化する
* [ ] Structured loggerを実装する
* [ ] Runtime snapshotを生成する
* [ ] Graceful shutdownを実装する
* [ ] Signal handlingを実装する
* [ ] Panicまたはcrash時のreportを作る

## 6. Resource Registry

* [ ] Resource ID規約を決める
* [ ] Resource typeを定義する
* [ ] Resource state machineを実装する
* [ ] Resource owner processを記録する
* [ ] Resource capabilityを記録する
* [ ] Resource permissionを記録する
* [ ] Resource dependency graphを作る
* [ ] Resource query APIを作る
* [ ] Resource update eventを出す
* [ ] Resource failure eventを出す
* [ ] Resource cleanupを実装する
* [ ] Resource leak検出を作る

## 7. Timebase

* [ ] Monotonic clockを実装する
* [ ] Manual clockを実装する
* [ ] Fixed timestep clockを実装する
* [ ] Audio clock stubを実装する
* [ ] Video clock stubを実装する
* [ ] Timecode clock stubを実装する
* [ ] Beat clock stubを実装する
* [ ] Clock selection policyを作る
* [ ] Clock fallback policyを作る
* [ ] Timestamp conversionを実装する
* [ ] Drift detectionを実装する
* [ ] Late frame detectionを実装する
* [ ] Dropped frame counterを作る
* [ ] Deterministic replay testを作る

## 8. Graph Engine

* [ ] Node interfaceを定義する
* [ ] Node inputとoutputを定義する
* [ ] Node parameterを定義する
* [ ] Graph validationを実装する
* [ ] Graph topological sortを実装する
* [ ] Graph cycle detectionを実装する
* [ ] Graph execution schedulerを実装する
* [ ] Node lifecycleを実装する
* [ ] Node error propagationを実装する
* [ ] Graph snapshotを生成する
* [ ] Graph diffを生成する
* [ ] Graph hot reloadの方針を書く

## 9. Metal Render Engine

* [ ] Metal device初期化を実装する
* [ ] Command queue管理を実装する
* [ ] Render pass管理を実装する
* [ ] Compute pass管理を実装する
* [ ] Shader compiler連携を作る
* [ ] Pipeline cacheを作る
* [ ] Texture poolを作る
* [ ] Buffer poolを作る
* [ ] Offscreen render targetを作る
* [ ] Window previewを作る
* [ ] Headless render pathを作る
* [ ] GPU timingを取得する
* [ ] GPU error reportingを作る
* [ ] Resource lifetime trackingを実装する
* [ ] Texture leak testを作る
* [ ] 1080p60 benchmarkを作る
* [ ] 4K60 benchmarkを作る

## 10. Core Video and IOSurface Bridge

* [ ] CVPixelBuffer受け取りを実装する
* [ ] CVPixelBuffer metadata取得を実装する
* [ ] CVMetalTextureCache連携を実装する
* [ ] IOSurface handle管理を実装する
* [ ] IOSurface metadata方針を決める
* [ ] Pixel format mapping tableを作る
* [ ] Color space mapping tableを作る
* [ ] Alpha mode mapping tableを作る
* [ ] YUV to RGB shaderを作る
* [ ] HDR handling方針を書く
* [ ] CPU copy pathを実装する
* [ ] GPU pathを実装する
* [ ] Performance比較testを作る

## 11. AVFoundation Input

* [ ] Camera device discoveryを実装する
* [ ] Camera mode discoveryを実装する
* [ ] Camera permission checkを実装する
* [ ] Capture sessionを実装する
* [ ] Frame callbackを実装する
* [ ] CVPixelBuffer to FramePacket変換を実装する
* [ ] Timestamp変換を実装する
* [ ] Format negotiationを実装する
* [ ] Capture start and stopを実装する
* [ ] Device busy errorを実装する
* [ ] Permission denied errorを実装する
* [ ] Camera disconnect handlingを実装する
* [ ] AVFoundation camera exampleを作る

## 12. Audio Foundation

* [ ] Audio device discoveryを実装する
* [ ] Audio permission checkを実装する
* [ ] Audio input streamを実装する
* [ ] AudioPacket schemaに変換する
* [ ] Audio clockを接続する
* [ ] Level meter nodeを作る
* [ ] FFT nodeを作る
* [ ] Beat detection stubを作る
* [ ] Audio xrun detectionを作る
* [ ] Audio device change handlingを作る
* [ ] Audio reactive shader exampleを作る

## 13. Native Plugin C ABI

* [ ] `frame0_plugin_api.h`を作る
* [ ] API version定数を作る
* [ ] Descriptor取得関数を作る
* [ ] Initialize関数を作る
* [ ] Shutdown関数を作る
* [ ] Device enumerate関数を作る
* [ ] Device open関数を作る
* [ ] Device close関数を作る
* [ ] Stream start関数を作る
* [ ] Stream stop関数を作る
* [ ] Callback登録関数を作る
* [ ] Error取得関数を作る
* [ ] Memory ownership規約を書く
* [ ] Threading規約を書く
* [ ] C ABI compatibility testを書く
* [ ] Header lintを作る

## 14. C++ SDK Adapter Framework

* [ ] C++ base interfaceを作る
* [ ] Device discovery interfaceを作る
* [ ] Video input interfaceを作る
* [ ] Audio input interfaceを作る
* [ ] Control plane interfaceを作る
* [ ] Timecode interfaceを作る
* [ ] Metadata interfaceを作る
* [ ] Vendor property APIを作る
* [ ] Adapter to C ABI bridgeを作る
* [ ] RAII wrapperを作る
* [ ] Exception boundaryを実装する
* [ ] Callback thread handoffを実装する
* [ ] Lock orderを文書化する
* [ ] Adapter templateを作る

## 15. Mock SDK Adapter

* [ ] Mock video SDKを作る
* [ ] Mock audio SDKを作る
* [ ] Mock timecode generatorを作る
* [ ] Mock device enumerationを作る
* [ ] Mock format negotiationを作る
* [ ] Mock frame generatorを作る
* [ ] Mock dropped frame modeを作る
* [ ] Mock slow callback modeを作る
* [ ] Mock crash modeを作る
* [ ] Mock hot unplug modeを作る
* [ ] Mock pluginをC ABIでexportする
* [ ] Mock plugin integration testを書く

## 16. Real SDK Adapter Sample

* [ ] サンプル対象SDKを選ぶ
* [ ] SDK license確認を行う
* [ ] SDK include path方針を決める
* [ ] SDK binary配布方針を決める
* [ ] Adapter manifestを書く
* [ ] Device enumerationを実装する
* [ ] Mode enumerationを実装する
* [ ] Capture start and stopを実装する
* [ ] FramePacket変換を実装する
* [ ] AudioPacket変換を実装する
* [ ] Timecode変換を実装する
* [ ] Error mappingを実装する
* [ ] Hot unplug handlingを実装する
* [ ] Resource status reportingを実装する
* [ ] SDK adapter example sceneを作る
* [ ] 実機なしfallback testを作る

## 17. Plugin Host

* [ ] Plugin Host executableを作る
* [ ] Plugin manifest loaderを作る
* [ ] Dynamic library loaderを作る
* [ ] C ABI symbol resolverを作る
* [ ] Plugin API version checkを作る
* [ ] Plugin initializationを実装する
* [ ] Plugin shutdownを実装する
* [ ] Plugin process supervisionを実装する
* [ ] Plugin crash detectionを実装する
* [ ] Plugin restart policyを実装する
* [ ] Plugin log forwardingを実装する
* [ ] Plugin event forwardingを実装する
* [ ] Plugin resource registrationを実装する
* [ ] Plugin permission validationを実装する

## 18. IPC

* [ ] IPC方式を決める
* [ ] Command message schemaを作る
* [ ] Event message schemaを作る
* [ ] Resource message schemaを作る
* [ ] Frame transport方式を決める
* [ ] Audio transport方式を決める
* [ ] Shared memory pathを作る
* [ ] IOSurface handle pathを作る
* [ ] Backpressure handlingを作る
* [ ] Timeout handlingを作る
* [ ] IPC reconnectを作る
* [ ] IPC fuzz testを作る
* [ ] IPC load testを作る

## 19. Permissions and Security

* [ ] Permission schemaを作る
* [ ] Plugin permission manifestを実装する
* [ ] Camera permission checkを作る
* [ ] Microphone permission checkを作る
* [ ] Network permission flagを作る
* [ ] File read permission flagを作る
* [ ] File write permission flagを作る
* [ ] Unknown plugin warningを作る
* [ ] Plugin signature方針を決める
* [ ] Signed plugin verify stubを作る
* [ ] Runtime sandbox方針を書く
* [ ] Secret handling方針を書く
* [ ] AI external access policyを書く

## 20. Output Adapters

* [ ] Screen outputを作る
* [ ] Headless outputを作る
* [ ] Image sequence outputを作る
* [ ] Movie output方針を決める
* [ ] AVFoundation writer outputを実装する
* [ ] Syphon output方針を決める
* [ ] NDI output方針を決める
* [ ] Core Media I/O Camera Extension prototype方針を書く
* [ ] Output format negotiationを実装する
* [ ] Multi output graphを実装する
* [ ] Output timing testを書く

## 21. Core Media I/O Camera Extension Adapter

* [ ] Camera ExtensionをOutput Adapterとして定義する
* [ ] ExtensionとRuntimeの通信方式を決める
* [ ] CMSampleBuffer生成pathを作る
* [ ] Pixel formatを決める
* [ ] Clock同期方針を書く
* [ ] Extension entitlement確認を行う
* [ ] Packaging方針を決める
* [ ] Install and uninstall flowを作る
* [ ] Extension only test sceneを作る
* [ ] Legacy DALに依存しないことを確認する

## 22. Audio Unit v3 Adapter

* [ ] AUv3をInputまたはControl Adapterとして位置づける
* [ ] AU parameter mappingを設計する
* [ ] MIDI event mappingを設計する
* [ ] Audio buffer input pathを設計する
* [ ] Host synchronization方針を書く
* [ ] AUv3 prototypeを作る
* [ ] DAWからparameter制御するtestを作る

## 23. DriverKit and System Extension Policy

* [ ] DriverKitをCore対象外にする方針を書く
* [ ] DriverKitが必要な条件を定義する
* [ ] Hardware driver Adapterの境界を書く
* [ ] DriverKit Extension packaging方針を書く
* [ ] Driver communication Resourceを定義する
* [ ] DriverKit sample調査を行う
* [ ] DriverKitをv0.1対象外にするか決める

## 24. Scene Examples

* [ ] hello_shader exampleを作る
* [ ] headless_render exampleを作る
* [ ] camera_to_shader exampleを作る
* [ ] audio_reactive exampleを作る
* [ ] mock_sdk_input exampleを作る
* [ ] multi_output exampleを作る
* [ ] device_error exampleを作る
* [ ] plugin_crash_recovery exampleを作る
* [ ] AI_scene_patch exampleを作る
* [ ] native_adapter_template exampleを作る

## 25. Testing

* [ ] Unit test frameworkを設定する
* [ ] Integration test frameworkを設定する
* [ ] CLI snapshot testを作る
* [ ] Schema validation testを作る
* [ ] Resource lifecycle testを作る
* [ ] Timebase deterministic testを作る
* [ ] Graph validation testを作る
* [ ] Metal render testを作る
* [ ] Texture lifetime testを作る
* [ ] AVFoundation mock testを作る
* [ ] Audio mock testを作る
* [ ] Plugin load testを作る
* [ ] Plugin crash testを作る
* [ ] IPC load testを作る
* [ ] Error suggestion testを作る
* [ ] Performance benchmark testを作る
* [ ] 10 minute soak testを作る

## 26. Diagnostics

* [ ] `frame0 doctor`を強化する
* [ ] GPU availability checkを作る
* [ ] Camera permission checkを作る
* [ ] Microphone permission checkを作る
* [ ] Plugin path checkを作る
* [ ] SDK dependency checkを作る
* [ ] Extension install checkを作る
* [ ] Runtime health checkを作る
* [ ] Device health checkを作る
* [ ] Performance health checkを作る
* [ ] JSON diagnostic reportを出力する
* [ ] Human readable diagnostic reportを出力する

## 27. Performance

* [ ] FPS measurementを実装する
* [ ] GPU frame time measurementを実装する
* [ ] CPU frame time measurementを実装する
* [ ] Audio xrun counterを実装する
* [ ] Dropped frame counterを実装する
* [ ] Memory usage trackingを実装する
* [ ] Texture allocation counterを実装する
* [ ] Plugin latency measurementを実装する
* [ ] IPC latency measurementを実装する
* [ ] Benchmark sceneを作る
* [ ] Benchmark JSON reportを作る
* [ ] Performance regression CIを作る

## 28. Documentation

* [ ] FRAME0 overviewを書く
* [ ] AI仕様書を書く
* [ ] 開発プランを書く
* [ ] TODO checklistを書く
* [ ] CLI referenceを書く
* [ ] Scene manifest referenceを書く
* [ ] Plugin manifest referenceを書く
* [ ] C ABI referenceを書く
* [ ] C++ Adapter guideを書く
* [ ] Resource model guideを書く
* [ ] Timebase guideを書く
* [ ] Render graph guideを書く
* [ ] Media input guideを書く
* [ ] Plugin isolation guideを書く
* [ ] Troubleshooting guideを書く
* [ ] Examples guideを書く

## 29. Developer Experience

* [ ] `frame0 new` templateを作る
* [ ] `frame0 new plugin` templateを作る
* [ ] `frame0 new adapter` templateを作る
* [ ] `frame0 examples list`を作る
* [ ] `frame0 examples run`を作る
* [ ] `frame0 plugins verify`を作る
* [ ] `frame0 plugins package`を作る
* [ ] `frame0 benchmark`を作る
* [ ] `frame0 logs`を作る
* [ ] Error messageを実用的にする
* [ ] AI向けsuggestionを出す
* [ ] Docsとschemaを同期する

## 30. Packaging and Release

* [ ] macOS buildを作る
* [ ] universal binary方針を決める
* [ ] installer方針を決める
* [ ] plugin package formatを決める
* [ ] release artifactを作る
* [ ] checksumを出す
* [ ] signature方針を決める
* [ ] notarization方針を決める
* [ ] crash report方針を決める
* [ ] telemetry方針を決める
* [ ] opt in analytics方針を書く
* [ ] public alpha criteriaを定義する

## 31. V0.1 Exit Criteria

* [ ] CLIでsceneを作れる
* [ ] CLIでsceneを検査できる
* [ ] CLIでsceneを実行できる
* [ ] JSONでgraphを取得できる
* [ ] JSONでdeviceを取得できる
* [ ] JSONでpluginを取得できる
* [ ] Runtime snapshotを取得できる
* [ ] NDJSON event streamを取得できる
* [ ] Metal previewが動く
* [ ] Headless renderが動く
* [ ] AVFoundation camera inputが動く
* [ ] CoreAudio inputが動く
* [ ] FFT nodeが動く
* [ ] Mock native pluginが動く
* [ ] Real SDK Adapter sampleが動く
* [ ] Plugin crashでRuntimeが落ちない
* [ ] Error suggestionが出る
* [ ] 主要exampleがCIで通る
* [ ] 10 minute soak testが通る
* [ ] AIがscene errorを読んで修正できる

## 32. Stop Doing List

* [ ] GUI editorを先に作らない
* [ ] ベンダーSDKをCoreへ直結しない
* [ ] Camera Extensionを最初に作らない
* [ ] Processing風APIを最初に作らない
* [ ] OS Extension内に作品ロジックを書かない
* [ ] C++ ABIを公開しない
* [ ] testなしでdemoを増やさない
* [ ] JSON inspectなしの機能を作らない
* [ ] Mock deviceなしで実機対応へ進まない
* [ ] Timebaseをrender loopに従属させない
* [ ] zero copyを実測なしで信じない
* [ ] AIが読めないログを主診断にしない

## 33. Brutal Priority List

本当に最初にやる順番。

* [ ] 1. CLI skeleton
* [ ] 2. JSON schema
* [ ] 3. Resource Registry
* [ ] 4. NDJSON event stream
* [ ] 5. Timebase
* [ ] 6. Metal graph
* [ ] 7. Mock device
* [ ] 8. AVFoundation camera
* [ ] 9. CoreAudio FFT
* [ ] 10. C ABI Plugin
* [ ] 11. Plugin Host
* [ ] 12. Mock SDK Adapter
* [ ] 13. Real SDK Adapter sample
* [ ] 14. AI diagnostics
* [ ] 15. Output adapters

これ以外に先に手を出したくなったら、だいたい逃避である。

## 34. 参考資料

* Apple Core Media I/O Camera Extension: https://developer.apple.com/documentation/coremediaio/creating-a-camera-extension-with-core-media-i-o
* Apple WWDC22 Core Media IO Camera Extensions: https://developer.apple.com/videos/play/wwdc2022/10022/
* Apple Metal: https://developer.apple.com/documentation/metal
* Apple AVFoundation: https://developer.apple.com/documentation/avfoundation/
* Apple Audio Unit v3 Plug Ins: https://developer.apple.com/documentation/audiotoolbox/audio-unit-v3-plug-ins
* Apple IOSurface: https://developer.apple.com/documentation/iosurface
* Apple XPC: https://developer.apple.com/documentation/xpc
* Apple DriverKit: https://developer.apple.com/documentation/driverkit
* Blackmagic Camera REST API developer page: https://www.blackmagicdesign.com/developer/products/camera/sdk-and-software
