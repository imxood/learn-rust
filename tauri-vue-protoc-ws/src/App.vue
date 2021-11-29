<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup
import HelloWorld from './components/HelloWorld.vue'
import { Student, Teacher } from './protos/example';
import { reactive, readonly, provide } from 'vue';
import {
  UpdateRawStudentKey,
  UpdateRefStudentKey,
  CreateTeacherKey,
  UpdateTeachertKey,
  AllUpdateRefStudentKey
} from './types';

let stu = Student.create();
stu.name = "maxu"
stu.age = 18

const student = reactive(stu)
const student_copy = readonly(student)
const student1 = reactive(student)
console.log(student)

const print_student = (student: Student) => {
  console.log("print student", student);
}

provide(UpdateRawStudentKey, () => {
  stu.age++;
  print_student(student_copy)
  console.log("stu", stu)
})

provide(UpdateRefStudentKey, () => {
  student.age++;
  student1.age += 2;
  print_student(student_copy)
  console.log("stu", stu)
  console.log("student1", student1)
})

provide(AllUpdateRefStudentKey, () => {
  let stu = Student.create();
  stu.name = "xiaoming"
  stu.age = 18
  //
  // 无法覆盖更新!!!
  //
  print_student(student_copy)
  console.log("stu", stu)
})

provide(CreateTeacherKey, () => {
  let teacher = Teacher.create();
  teacher.name = "hello"
  teacher.age = 36
  student.teacher = teacher
  print_student(student_copy)
  console.log("stu", stu)
})

provide(UpdateTeachertKey, () => {
  if (student.teacher) {
    student.teacher.age++;
    print_student(student_copy)
    console.log("stu", stu)
  }
})

</script>

<template>
  <HelloWorld msg="Hello Vue 3 + TypeScript + Vite" />
  <div>
    <div>stydent name: {{ student.name }} age: {{ student.age }}</div>
    <div>teacher name: {{ student.teacher?.name }} age: {{ student.teacher?.age }}</div>
  </div>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
