# 词格酱 · CigeChan

按作曲提供的词格摆出空格子，逐字填词的中文歌词写作工具。

名字是「词格」+「酱」——后者取日语 ちゃん 的亲昵后缀。

前端是 `src/index.html` 一个文件，没有构建步骤。用浏览器直接打开它也能用，
只是保存文件走的是浏览器下载，而不是系统的保存对话框。

## 功能

- 按词格摆格子逐字填词，中文输入法直接连打
- 点哪个格子就写在哪个格子，可以先落句尾几个字、前面留空慢慢填
- 句内停顿分组（如 `2/2/3`），加减格只作用于光标所在的分句
- 每句可存多个备选版本，随时切换，切换前自动保住未存的写法
- 每句可加备注，不占格子
- 自定义背景：图片 / 纯色 / 两套底色，蒙版与模糊可调
- 导入 txt 词格或现成歌词反推词格；导出歌词（可选带全部备选与备注）、词格、工程 json

## 本地开发

需要先装好 [Rust](https://rustup.rs) 和 [Node](https://nodejs.org)，
以及 Tauri 的[系统依赖](https://v2.tauri.app/start/prerequisites/)。

```bash
npm install
npm run dev      # 开发模式，改完 src/index.html 刷新即可
npm run build    # 出本平台的安装包，产物在 src-tauri/target/release/bundle/
```

## 出三平台的安装包

推到 GitHub 后，打一个 `v` 开头的 tag 就会自动构建：

```bash
git tag v1.3.0
git push origin v1.3.0
```

Actions 跑完会创建一个**草稿 Release**，里面挂着：

| 平台 | 产物 |
|---|---|
| Windows | `.msi` 和 `.exe`（NSIS） |
| macOS | `.dmg` 两份，Apple Silicon 和 Intel 各一 |
| Linux | `.AppImage` 和 `.deb` |

确认无误后在 GitHub 上把草稿 Release 点成正式发布即可。
也可以在 Actions 页面手动触发（workflow_dispatch），先试跑一次再打 tag。

**仓库必须是公开的。** 公开仓库用 GitHub 托管的 runner 不限量免费；
私有仓库的 macOS 构建按 10 倍速度消耗免费额度，很快就会用完。

## 关于签名

这些包都没有做付费签名，用户首次打开时系统会拦一下：

- **Windows**：点「更多信息」→「仍要运行」
- **macOS**：右键点应用 →「打开」，只需做一次
- **Linux**：无障碍

macOS 的 arm64 二进制会由构建工具链自动做 ad-hoc 签名（这是 Apple Silicon 上
能运行的最低要求），但它不等于公证，Gatekeeper 仍会提示一次。

## 几个已知的坑

- **应用名用的是 `CigeChan`，窗口标题才是「词格酱」。**
  Linux 的 .deb 包名要求小写 ASCII，所以 `productName` 不能写中文。

- **`dragDropEnabled` 设成了 `false`。** Tauri 默认会拦截文件拖放并转成自己的事件，
  那样网页里的 HTML5 拖放就收不到文件了。关掉之后，导入词格和拖图片换背景才能正常工作。

- **背景图存在 localStorage 里**，受约 5MB 配额限制。
  以后可以改成存到应用数据目录，就没有大小限制了。

- **移动端还没适配。** Tauri 2 支持 iOS / Android，但当前界面是桌面交互：
  工具按钮靠 hover 出现、依赖快捷键、底栏会和虚拟键盘打架，
  透明输入框叠格子的方案在移动输入法下也需要重测。

## 目录

```
src/index.html          程序本体，唯一的前端文件
src-tauri/              Rust 外壳
  src/lib.rs            两个命令：save_text / open_text，负责系统文件对话框
  tauri.conf.json       窗口、打包、图标配置
  capabilities/         权限
  icons/                各平台图标
.github/workflows/      三平台自动构建
示例/                   示例词格和 12 个导入测试用例
```

---

GitHub [@yukinsnow](https://github.com/yukinsnow) · 哔哩哔哩 [@純白](https://space.bilibili.com/123372)
