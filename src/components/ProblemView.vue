<script setup lang="ts">
import { ref, watch } from 'vue';
import { Problem, ProblemList, ProblemStatus } from '../problem';
import { getGroupStatus, finish, reveal as revealProblem, getProblemList } from '../core';
import { app } from '../main';
import Button from './Button.vue';

const props = defineProps<{
    problemStatusList: Array<ProblemStatus>,
    problemList: ProblemList,
    group: number
}>();

const progress = ref(0);
const reveal = ref(false);
const end = ref(false);
const timeout = ref(Number.MAX_VALUE);
const currentProblem = ref({
    id: -1,
    question: "There is a problem here.",
    answer: "none",
    limit: 600,
    score: 100
} as Problem);

async function sleep(duration: number) {
    await new Promise(resolve => setTimeout(resolve, duration));
    return;
}

if (app.$cookies.isKey("revealed"))
    reveal.value = true;

watch(reveal, () => {
    if (reveal.value)
        app.$cookies.set("revealed", true, "1d");
    else if (app.$cookies.isKey("revealed"))
        app.$cookies.remove("removed");
})

setInterval(async () => {
    //console.log(props.problemList)
    let groupStatus = await getGroupStatus();
    for (let status of groupStatus) {
        if (status.id == props.group) {
            if (status.end)
                timeout.value = status.end - Math.round(Date.now() / 1000);
            if (timeout.value < 0)
                end.value = true;
        }
    }
    for (let item of props.problemList) {
        if (item.group == props.group && item.status == "answering") {
            // console.log(groupStatus);
            for (let status of groupStatus) {
                if (status.id == props.group) {
                    if (status.end)
                        timeout.value = status.end - Math.round(Date.now() / 1000);
                    if (timeout.value < 0)
                        end.value = true;
                    if (item.problem)
                        currentProblem.value = item.problem;
                    if (timeout.value <= 0) {
                        await sleep(1000);
                        await next();
                        timeout.value = Number.MAX_VALUE;
                    }
                    return;
                }
            }
        }
    }
}, 200)

setInterval(async () => {
    for (let groupStatus of await getGroupStatus()) {
        if (groupStatus.id == props.group)
            setTimeout(() => {
                progress.value = groupStatus.progress;
            }, 200);
    };
}, 1000)

async function start() {
    if (progress.value == 0) 
        await finish(props.group);
    await revealProblem(props.group);
    let problemList = await getProblemList();
    for (let item of problemList) {
        if (item.group == props.group && item.status == "answering") {
            if (item.problem)
                currentProblem.value = item.problem;
            if (timeout.value <= 0) {
                next();
                timeout.value = Number.MAX_VALUE;
            }
            reveal.value = true;
            return;
        }
    }
    reveal.value = true;
    return;
}

async function next() {
    await finish(props.group);
    for (let groupStatus of await getGroupStatus()) {
        if (groupStatus.id == props.group)
            setTimeout(() => {
                progress.value = groupStatus.progress;
            }, 200);
    };
    reveal.value = false;
}
</script>

<template>
    <div v-if="!end" class="transition duration-300 h-[90%] ml-0 mr-auto mt-auto mb-auto border rounded-4xl border-gray-300 w-[40%]" :class="{ collapse: end }">
        <div class="grid grid-rows-10 grid-cols-1 h-[92.5%] gap-0">
            <div class="row-span-1 flex flex-row">
                <div class="ml-3 mt-3 w-40 h-10 rounded-3xl bg-gray-300 flex items-center justify-center">
                    <h1 class="text-xl">Progress: {{ progress }}</h1>
                </div>
                <h1 v-if="timeout !== Number.MAX_VALUE" class="text-2xl mt-4.25 ml-3 wrap-break-word overflow-hidden text-gray-600">Timeout: {{ Math.round(timeout).toFixed(0) }}s</h1>
            </div>
            
            <div v-if="progress === 0 || !reveal" class="row-span-9 transition duration-300 mt-3 mb-3 h-[96%] w-auto ml-3 mr-3 border border-gray-300 rounded-[20px] flex items-center justify-center hover:shadow-md hover:active:scale-99" @click="start()">
                <h1 class="text-3xl">Click to start</h1>
            </div>
            <div v-if="progress !== 0 && reveal" class="row-span-9 transition duration-300 mt-3 mb-3 h-[96%] w-auto ml-3 mr-3 pl-2 pr-2">
                <h1 class="text-3xl wrap-break-word overflow-hidden mt-5">{{ currentProblem.question }}</h1>
            </div>
        </div>
        <Button class="mb-3 ml-3 mt-auto" v-if="progress !== 0 && reveal"> 
            <p class="mt-auto mb-auto ml-auto mr-auto relative items-center justify-center" @click="next">Next ></p> 
        </Button>
    </div>
</template>