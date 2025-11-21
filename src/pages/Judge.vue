<script setup lang="ts">
import Button from '../components/Button.vue';
import { app } from '../main';
import { ref } from 'vue';
import { getJudgeTask, judge } from '../core';
import { router } from '../router';

function back() {
    router.replace("/");
    app.$cookies.remove("judge");
    return;
}

const answer = ref("Waiting the client to respond...");

setInterval(async () => {
    if (router.currentRoute.value.fullPath === `/judge/${app.$route.params.id}`) {
        let task = await getJudgeTask(Number(app.$route.params.id));
        if (task !== null) {
            answer.value = task.answer;
        }
        return;
    }
}, 200)

async function correct() {
    await judge(Number(app.$route.params.id), true);
    answer.value = "Waiting the client to respond...";
    return;
}

async function incorrect() {
    await judge(Number(app.$route.params.id), false);
    answer.value = "Waiting the client to respond...";
    return;
}
</script>

<template>
    <div class="select-none min-h-screen flex flex-col">
        <!-- Top bar -->
        <div class="flex items-center gap-4 bg-amber-100 px-3 py-2">
            <Button @click="back()" class="px-2 py-1">
                <p class="text-sm sm:text-base">< Back</p>
            </Button>
            <h1 class="text-lg sm:text-xl font-medium">Judging Group {{ $route.params.id }}</h1>
        </div>

        <!-- Answer / content -->
        <div class="flex-1 flex items-center justify-center px-4 py-6">
            <h1 class="text-2xl sm:text-4xl md:text-5xl text-center wrap-break-word">
                {{ answer }}
            </h1>
        </div>

        <!-- Action buttons: stacked on small, side-by-side on md+ -->
        <div class="flex flex-row items-center gap-4 px-4 pb-6 ml-auto mr-auto mb-5">
            <Button
                class="w-full md:w-40 h-12 md:h-40 flex items-center justify-center md:scale-200"
                :class="{ 'bg-green-600': answer !== 'Waiting the client to respond...', 'bg-gray-500': answer === 'Waiting the client to respond...' }"
                @click="correct"
                :disabled="answer === 'Waiting the client to respond...'"
            >
                <p class="text-lg md:text-3xl mt-auto mb-auto text-white">Correct</p>
            </Button>

            <Button
                class="w-full md:w-40 h-12 md:h-40 flex items-center justify-center md:scale-200"
                :class="{ 'bg-red-700': answer !== 'Waiting the client to respond...', 'bg-gray-500': answer === 'Waiting the client to respond...' }"
                @click="incorrect"
                :disabled="answer === 'Waiting the client to respond...'"
            >
                <p class="text-lg md:text-3xl mt-auto mb-auto text-white">Incorrect</p>
            </Button>
        </div>
    </div>
</template>