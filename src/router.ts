import { createMemoryHistory, createRouter, RouteRecordRaw } from 'vue-router';
import Entry from './pages/Entry.vue';
import Game from './pages/Game.vue';
import JudgeEntry from './pages/JudgeEntry.vue';
import Judge from './pages/Judge.vue';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: Entry
    },
    {
        path: '/group/:id',
        component: Game
    },
    {
        path: '/judge',
        component: JudgeEntry
    },
    {
        path: '/judge/:id',
        component: Judge
    }
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes: routes
})