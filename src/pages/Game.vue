<script setup lang="ts">
import { router } from '../router';
import { app } from '../main';
import { ProblemList, ProblemStatus } from '../problem';
import { getProblemList } from '../core';

import { ref } from 'vue';

import Button from '../components/Button.vue';
import ProblemView from '../components/ProblemView.vue';
import ProblemOverview from '../components/ProblemOverview.vue';

let problemStatusList = ref(new Array<Array<ProblemStatus>>(8).fill([]));
let problemList = ref([] as ProblemList);
let total = ref(new Array<number>(8).fill(0));

function back() {
    router.replace("/");
    app.$cookies.remove("group");
}

function enabled(group: number) {
    return app.$route.params.id === group.toString();
}

let progress = ref(0);

setInterval(async () => {
    problemList.value = await getProblemList();
    let totalValue = new Array<number>(8).fill(0);
    for (let item of problemList.value) {
        if (item.status === "solved")
            totalValue[item.group] += item.score;
    }
    total.value = totalValue;

    for (let item of problemList.value) 
        if (problemStatusList.value[item.group])
            problemStatusList.value[item.group].push(item.status);
}, 300);
</script>
<template>
    <div class="transition duration-300 h-full select-none">
        <div class="flex flex-row gap-4 bg-amber-100 h-15">
            <Button @click="back()" class="ml-2.5 mt-auto mb-auto"> 
                <p class="mt-auto mb-auto mr-auto ml-auto relative">< Back</p> 
            </Button>
            <h1 class="mt-auto mb-auto ml-5">Playing as Group {{ $route.params.id }}</h1>
        </div>
        <div class="transition duration-300 flex flex-row h-[90%] mt-2 mb-2">
            <div class="transition duration-300 grid grid-cols-8 grid-rows-6 h-[90%] ml-auto mr-auto mt-auto mb-auto border rounded-4xl border-gray-300 gap-2 p-2 w-[50%]" 
                :class="{ 'w-[80%]': progress >= 4, 'w-[50%]': progress >= 0 && progress < 4 }">
                <div class="row-start-1 row-end-2 col-start-1 col-end-2 bg-gray-200 rounded-3xl"></div>
                <div class="row-start-2 row-end-7 col-start-1 col-end-2 bg-gray-100 rounded-3xl flex flex-col pl-4 pr-4">
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">100</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">200</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">300</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">400</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center">
                        <h1 class="text-center text-3xl h-fit">Total</h1>
                    </div>
                </div>
                <div class="col-start-2 col-end-9 row-start-1 row-end-2 bg-gray-100 rounded-3xl grid grid-cols-7 grid-rows-1 pt-6 pb-6">
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(1) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G1</h1>
                    </div>
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(2) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G2</h1>
                    </div>
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(3) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G3</h1>
                    </div>
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(4) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G4</h1>
                    </div>
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(5) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G5</h1>
                    </div>
                    <div :class="'flex items-center border-r border-r-gray-300' + (enabled(6) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G6</h1>
                    </div>
                    <div :class="'flex items-center' + (enabled(7) ? ' bg-gray-300' : '')">
                        <h1 class="text-center text-4xl h-fit ml-auto mr-auto">G7</h1>
                    </div>
                </div>
                <ProblemOverview :group="1" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="2" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="3" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="4" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="5" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="6" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="7" :problem-list="problemList" :score="100"></ProblemOverview>
                <ProblemOverview :group="1" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="2" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="3" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="4" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="5" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="6" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="7" :problem-list="problemList" :score="200"></ProblemOverview>
                <ProblemOverview :group="1" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="2" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="3" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="4" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="5" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="6" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="7" :problem-list="problemList" :score="300"></ProblemOverview>
                <ProblemOverview :group="1" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="2" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="3" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="4" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="5" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="6" :problem-list="problemList" :score="400"></ProblemOverview>
                <ProblemOverview :group="7" :problem-list="problemList" :score="400"></ProblemOverview>
                <div class="row-start-6 row-end-7 col-start-2 col-end-3 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[1] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-3 col-end-4 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[2] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-4 col-end-5 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[3] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-5 col-end-6 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[4] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-6 col-end-7 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[5] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-7 col-end-8 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[6] }} </h1>
                </div>
                <div class="row-start-6 row-end-7 col-start-8 col-end-9 bg-gray-200 rounded-3xl flex items-center justify-center">
                    <h1 class="text-4xl"> {{ total[7] }} </h1>
                </div>
            </div>
            <ProblemView :problem-status-list="problemStatusList[Number($route.params.id)]" :problem-list="problemList" :group="Number($route.params.id)"/>
        </div>
    </div>
</template>