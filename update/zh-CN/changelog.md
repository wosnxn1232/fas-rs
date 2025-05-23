# v4.8.0 (2025-03-23)

* refactor: 优化webui体验 ([2a4787f](https://github.com/shadow3aaa/fas-rs/commit/2a4787f))
* refactor: 当重负载线程亲和都一样时使用此亲和而不是退化为全部核心 ([1ada2ed](https://github.com/shadow3aaa/fas-rs/commit/1ada2ed))
* refactor(webui): 去掉最外层的容器，节省屏幕空间 ([5636def](https://github.com/shadow3aaa/fas-rs/commit/5636def))
* refactor(webui): 移除黑色指示条 ([3061512](https://github.com/shadow3aaa/fas-rs/commit/3061512))
* refactor(webui): 调整webui文本描述 ([5845bc7](https://github.com/shadow3aaa/fas-rs/commit/5845bc7))
* feat: webui ([0ba2f49](https://github.com/shadow3aaa/fas-rs/commit/0ba2f49))
* feat: 根据线程亲和自动选择是否控制最小频率，修复忽略cpu集簇时频率验证失败的bug ([3d6133e](https://github.com/shadow3aaa/fas-rs/commit/3d6133e))
* feat(webui): 即时保存更改 ([e010866](https://github.com/shadow3aaa/fas-rs/commit/e010866))
* feat(webui): 添加切换动画 ([42c65fd](https://github.com/shadow3aaa/fas-rs/commit/42c65fd))
* fix: make eslint happy ([b56d726](https://github.com/shadow3aaa/fas-rs/commit/b56d726))
* fix: 添加webui后ci构建失败 ([2f2afb5](https://github.com/shadow3aaa/fas-rs/commit/2f2afb5))
* fix: 部分设备没有预料的温控设备导致panic ([63c9a31](https://github.com/shadow3aaa/fas-rs/commit/63c9a31))
* fix: 验证频率警告更有意义 ([d089e07](https://github.com/shadow3aaa/fas-rs/commit/d089e07))
* doc: 更新webui README描述 ([82c00d0](https://github.com/shadow3aaa/fas-rs/commit/82c00d0))
* build(deps): bump anyhow from 1.0.96 to 1.0.97 ([1a59e72](https://github.com/shadow3aaa/fas-rs/commit/1a59e72))
* build(deps): bump clap from 4.5.31 to 4.5.32 ([d0c3f40](https://github.com/shadow3aaa/fas-rs/commit/d0c3f40))
* build(deps): bump libc from 0.2.170 to 0.2.171 ([202dd51](https://github.com/shadow3aaa/fas-rs/commit/202dd51))
* build(deps): bump mimalloc from 0.1.43 to 0.1.44 ([da7aebe](https://github.com/shadow3aaa/fas-rs/commit/da7aebe))
* build(deps): bump serde from 1.0.218 to 1.0.219 ([02dd11f](https://github.com/shadow3aaa/fas-rs/commit/02dd11f))
* build(deps): bump serde_json from 1.0.139 to 1.0.140 ([5295af2](https://github.com/shadow3aaa/fas-rs/commit/5295af2))
* build(deps): bump thiserror from 2.0.11 to 2.0.12 ([a1620b5](https://github.com/shadow3aaa/fas-rs/commit/a1620b5))
* build(deps): bump zip from 2.2.3 to 2.3.0 ([55a36e8](https://github.com/shadow3aaa/fas-rs/commit/55a36e8))
* build(deps): bump zip from 2.3.0 to 2.4.1 ([116f167](https://github.com/shadow3aaa/fas-rs/commit/116f167))
* build(deps): bump zip from 2.4.1 to 2.4.2 ([862effa](https://github.com/shadow3aaa/fas-rs/commit/862effa))
