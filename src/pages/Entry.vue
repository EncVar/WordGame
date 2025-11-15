<script setup lang="ts">
import { ref } from 'vue';
import { router } from '../router';
import Button from '../components/Button.vue';
import Input from '../components/Input.vue';
import { app } from '../main';

let group = ref("");

if (app.$cookies.isKey("group"))
    router.replace(`/group/${app.$cookies.get("group")}`);

if (app.$cookies.isKey("admin"))
    router.replace("/admin");

function join() {
    if (group.value === "judge") {
        router.replace("/judge");
        return;
    }
    if (Number.isInteger(Number(group.value)) && Number(group.value) >= 1 && Number(group.value) <= 7) {
        router.replace(`/group/${group.value}`);
        app.$cookies.set("group", group.value, "1d");
        return;
    }
    if (group.value.toLowerCase() === "admin") {
        router.replace("/admin");
        app.$cookies.set("admin", "1", "1d");
        return;
    }
}

function onFocusOut() {
    disabled.value = (
        group.value === "" || (
            group.value.toLowerCase() !== "judge" &&
            group.value.toLowerCase() !== "admin" &&
            !Number.isInteger(Number(group.value)) && 
            Number(group.value) < 1 || Number(group.value) > 7 
        )
    );
}

let disabled = ref(
    group.value === "" || (
        group.value.toLowerCase() !== "judge" &&
        group.value.toLowerCase() !== "admin" &&
        !Number.isInteger(Number(group.value)) && 
        Number(group.value) < 1 || Number(group.value) > 7 
    )
);
</script>

<template>
    <div class="flex flex-row select-none">
        <h1 class="mt-90 text-6xl ml-auto">Join as group </h1>
        <Input v-model="group" class="mt-90 ml-5 mr-auto" @on-enter="join()" @focusout="onFocusOut()"/>
    </div>
    <Button class="ml-auto mr-auto mt-10" @click="join()" :disabled="disabled">
       <p class="mt-auto mb-auto ml-auto mr-auto relative items-center justify-center">Enter</p> 
    </Button>
</template>