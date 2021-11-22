import { InjectionKey } from "@vue/runtime-core";

export const UpdateRawStudentKey: InjectionKey<() => {}> = Symbol('update_raw_student');
export const UpdateRefStudentKey: InjectionKey<() => {}> = Symbol('update_ref_student');
export const AllUpdateRefStudentKey: InjectionKey<() => {}> = Symbol('all_update_ref_student');

export const CreateTeacherKey: InjectionKey<() => {}> = Symbol('create_teacher');
export const UpdateTeachertKey: InjectionKey<() => {}> = Symbol('update_teacher');
