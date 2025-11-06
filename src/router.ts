import { createMemoryHistory, createRouter } from 'vue-router';
import Problem from './pages/Problem.vue';
import Entry from './pages/Entry.vue';

const routes = [
    {
        path: '/problem',
        component: Problem
    },
    {
        path: '/',
        component: Entry
    }
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes: routes
})