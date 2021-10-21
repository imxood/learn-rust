<script setup lang="ts">
import { getCurrent } from '@tauri-apps/api/window'
import { ref, onMounted } from 'vue'
import { GetRequest, GetResponse } from '../protos/example'
import axios from 'axios';
import { JsonValue } from "@protobuf-ts/runtime";

defineProps<{ msg: string }>()

onMounted(() => {

  let ws = new WebSocket("ws://localhost:8080/ws")

  ws.onopen = async () => {
    console.log("websocket opened");

    let window = getCurrent();
    for (var i = 0; i < 3; i++) {
      await window.emit("init", "hello, the world");
    }

    let req = GetRequest.create();
    req.name = "maxu";
    let binary = GetRequest.toBinary(req);
    ws.send(binary);
  }

  ws.onclose = () => {
    console.log("websocket closed");
  }

  ws.onmessage = async (e) => {
    var buffer = await e.data.arrayBuffer();
    let data = new Uint8Array(buffer);
    console.log("recv msg", data);
    let obj = GetResponse.fromBinary(data);
    console.log("new obj", obj);
  }
})

const count_clicked = async (e: Event) => {
  count.value++;

  let req = GetRequest.create();
  req.name = "maxu";
  req.age = 18;

  let binary = GetRequest.toBinary(req);
  console.log("send binary", binary);

  axios.post("http://127.0.0.1:8080/proto", binary, { responseType: "arraybuffer" }).then((res) => {
    console.log("------------ proto ------------");
    console.log("recv binary", res.data);
    console.log("res.data type", typeof res.data);
    let bin = new Uint8Array(res.data as ArrayBuffer);
    console.log("recv binary", bin);
    console.log("recv req", GetRequest.fromBinary(bin));
  }).catch((error) => {
    console.log(error);
  });

  console.log("send binary", binary);

  axios.post("http://127.0.0.1:8080/proto_json", binary).then((res) => {
    console.log("------------ proto json ------------");
    console.log("recv json", GetRequest.fromJson(res.data as JsonValue));
  }).catch((error) => {
    console.log(error);
  });

  // let ret = await new HttpRequest("http://127.0.0.1:8080/proto").data(binary).post()
}

const count = ref(0)
</script>

<template>
  <h1>{{ msg }}</h1>

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VSCode</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>
    See
    <code>README.md</code> for more information.
  </p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank">Vite Docs</a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Docs</a>
  </p>

  <button type="button" @click="count_clicked">count is: {{ count }}</button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
