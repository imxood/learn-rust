# Vue 3 + Typescript + Vite

This template should help get you started developing with Vue 3 and Typescript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=johnsoncodehk.volar)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's `.vue` type support plugin by running `Volar: Switch TS Plugin on/off` from VSCode command palette.


## 项目构建

    // vue3-vuex-ts

### 添加 tauri

    yarn add -D @tauri-apps/cli

    yarn add @tauri-apps/api

    初始化 tauri 项目, 记得执行前设置代理:

        $env:HTTP_PROXY="http://127.0.0.1:1080"
        $env:HTTPS_PROXY="http://127.0.0.1:1080"

    yarn tauri init

## 使用 vuex 管理状态

    yarn add vuex@next --save

### 添加 websocket

    yarn add vue-native-websocket-vue3

### protobuf-ts

    yarn add @protobuf-ts/plugin

    // yarn add @protobuf-ts/protoc
    // yarn add google-protobuf

    默认会下载 https://github.com/protocolbuffers/protobuf/releases
    
    记得在 package.json 中添加, 以避免每次运行的时候, 检查最新版本(由于网络问题):
    "config": {
        "protocVersion": "3.18.1"
    },

    yarn protoc --ts_out src/protos --proto_path protos protos/example.proto

### rust 库

    cargo add tokio --features "rt-multi-thread time net"
    
    cargo add protobuf --features "with-bytes"
    cargo add protobuf-codegen-pure -B
    
    cargo add poem --features websocket
    cargo add futures_util

    cargo add log flexi_logger anyhow