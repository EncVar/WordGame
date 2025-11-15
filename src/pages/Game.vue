<script setup lang="ts">
import { router } from '../router';
import { app } from '../main';
import { ProblemList, ProblemStatus } from '../problem';
import { getProblemList } from '../core';

import { ref } from 'vue';

import Button from '../components/Button.vue';
import ProblemView from '../components/ProblemView.vue';
import ProblemOverview from '../components/ProblemOverview.vue';

let problemStatusList = ref(new Array<Array<ProblemStatus>>(8).fill(new Array(5).fill('unrevealed' as ProblemStatus)));
let problemList = ref([] as ProblemList);

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
    try {
        for (let item of problemList.value)
            problemStatusList.value[item.column][item.score / 100] = item.status;
    } catch(e) {
        console.log(e);
        throw e;
    }
}, 1000);
</script>
<template>
    <div class="h-full select-none">
        <div class="flex flex-row gap-4 bg-amber-100 h-15">
            <Button @click="back()" class="ml-2.5 mt-auto mb-auto"> 
                <p class="mt-auto mb-auto mr-auto ml-auto relative">< Back</p> 
            </Button>
            <h1 class="mt-auto mb-auto ml-5">Playing as Group {{ $route.params.id }}</h1>
        </div>
        <div class="flex flex-row h-[90%] mt-2 mb-2">
            <div class="transition duration-300 grid grid-cols-8 grid-rows-5 h-[90%] ml-auto mr-5 mt-auto mb-auto border rounded-4xl border-gray-300 gap-2 p-2" :class="{ 'w-[80%]': progress >= 4, 'w-[50%]': progress >= 0 && progress < 4 }">
                <div class="row-start-1 row-end-2 col-start-1 col-end-2 bg-gray-200 rounded-3xl"></div>
                <div class="row-start-2 row-end-6 col-start-1 col-end-2 bg-gray-100 rounded-3xl flex flex-col pl-4 pr-4">
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">100</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">200</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center border-b border-b-gray-300">
                        <h1 class="text-center text-4xl h-fit">300</h1>
                    </div>
                    <div class="flex-auto flex justify-center items-center">
                        <h1 class="text-center text-4xl h-fit">400</h1>
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
                <ProblemOverview :enabled="enabled(1)" :status="problemStatusList[1][1]"/>
                <ProblemOverview :enabled="enabled(2)" :status="problemStatusList[2][1]"/>
                <ProblemOverview :enabled="enabled(3)" :status="problemStatusList[3][1]"/>
                <ProblemOverview :enabled="enabled(4)" :status="problemStatusList[4][1]"/>
                <ProblemOverview :enabled="enabled(5)" :status="problemStatusList[5][1]"/>
                <ProblemOverview :enabled="enabled(6)" :status="problemStatusList[6][1]"/>
                <ProblemOverview :enabled="enabled(7)" :status="problemStatusList[7][1]"/>
                <ProblemOverview :enabled="enabled(1)" :status="problemStatusList[1][2]"/>
                <ProblemOverview :enabled="enabled(2)" :status="problemStatusList[2][2]"/>
                <ProblemOverview :enabled="enabled(3)" :status="problemStatusList[3][2]"/>
                <ProblemOverview :enabled="enabled(4)" :status="problemStatusList[4][2]"/>
                <ProblemOverview :enabled="enabled(5)" :status="problemStatusList[5][2]"/>
                <ProblemOverview :enabled="enabled(6)" :status="problemStatusList[6][2]"/>
                <ProblemOverview :enabled="enabled(7)" :status="problemStatusList[7][2]"/>
                <ProblemOverview :enabled="enabled(1)" :status="problemStatusList[1][3]"/>
                <ProblemOverview :enabled="enabled(2)" :status="problemStatusList[2][3]"/>
                <ProblemOverview :enabled="enabled(3)" :status="problemStatusList[3][3]"/>
                <ProblemOverview :enabled="enabled(4)" :status="problemStatusList[4][3]"/>
                <ProblemOverview :enabled="enabled(5)" :status="problemStatusList[5][3]"/>
                <ProblemOverview :enabled="enabled(6)" :status="problemStatusList[6][3]"/>
                <ProblemOverview :enabled="enabled(7)" :status="problemStatusList[7][3]"/>
                <ProblemOverview :enabled="enabled(1)" :status="problemStatusList[1][4]"/>
                <ProblemOverview :enabled="enabled(2)" :status="problemStatusList[2][4]"/>
                <ProblemOverview :enabled="enabled(3)" :status="problemStatusList[3][4]"/>
                <ProblemOverview :enabled="enabled(4)" :status="problemStatusList[4][4]"/>
                <ProblemOverview :enabled="enabled(5)" :status="problemStatusList[5][4]"/>
                <ProblemOverview :enabled="enabled(6)" :status="problemStatusList[6][4]"/>
                <ProblemOverview :enabled="enabled(7)" :status="problemStatusList[7][4]"/>
            </div>
            <ProblemView class="transition duration-300 h-[90%] ml-0 mr-auto mt-auto mb-auto border rounded-4xl border-gray-300 w-[40%]" :problem-status-list="problemStatusList[Number($route.params.id)]" :problem-list="problemList" :group="Number($route.params.id)"/>
        </div>
    </div>
</template>