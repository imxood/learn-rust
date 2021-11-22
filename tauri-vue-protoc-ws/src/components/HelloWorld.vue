<script setup lang="ts">
import { getCurrent } from '@tauri-apps/api/window'
import { ref, inject, onMounted } from 'vue'
import { GetRequest, GetResponse } from '../protos/example'
import axios from 'axios';
import { JsonValue } from "@protobuf-ts/runtime";
import {
  UpdateRawStudentKey,
  UpdateRefStudentKey,
  CreateTeacherKey,
  UpdateTeachertKey,
  AllUpdateRefStudentKey
} from '../types';

defineProps<{ msg: string }>()

const update_raw_student = inject(UpdateRawStudentKey)
const update_ref_student = inject(UpdateRefStudentKey)
const create_teacher = inject(CreateTeacherKey)
const update_teacher = inject(UpdateTeachertKey)
const all_update_ref_student = inject(AllUpdateRefStudentKey)

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

const test_proto_clicked = async (e: Event) => {
  let req = GetRequest.create();
  req.name = "maxu";
  req.age = 18;

  let binary = GetRequest.toBinary(req);
  console.log("send binary", binary);

  axios.post("http://127.0.0.1:8080/test_json").then((res) => {
    console.log("------------ proto ------------");
    console.log("recv binary", res.data);
    console.log("res.data type", typeof res.data);
    let bin = new Uint8Array(res.data as ArrayBuffer);
    console.log("recv binary", bin);
    console.log("recv req", GetRequest.fromBinary(bin));
  }).catch((error) => {
    console.log(error);
  });

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
}
</script>

<template>
  <h1>{{ msg }}</h1>
  <div>
    <button type="button" @click="test_proto_clicked">test_proto</button>
  </div>
  <div>
    <button type="button" @click="update_raw_student">update_raw_student</button>
    <button type="button" @click="update_ref_student">update_ref_student</button>
    <button type="button" @click="all_update_ref_student">all_update_ref_student</button>
    <button type="button" @click="create_teacher">create_teacher</button>
    <button type="button" @click="update_teacher">update_teacher</button>
  </div>
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
