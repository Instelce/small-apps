import { authStore, isAuthentificated } from '$lib/store/auth';
import { get } from 'svelte/store';
import type { PageServerLoad } from './$types';
import { pages } from '$lib/api/pages';
import { user } from '$lib/api/user';
import { pagesStore } from '$lib/store/pages';

export const load = (async ({ }) => {
    let { data, errors } = await user.pages();

    let users = data ? data.map((page) => page.collaborators).flat() : [];
    let colors: { [key: string]: string } = {};
    let availableColors = ['bg-orange-400', 'bg-purple-400', 'bg-indigo-400', 'bg-yellow-400'];
    users.forEach((user) => {
        colors[user.name] = availableColors[Math.floor(Math.random() * availableColors.length)];
    });

    pagesStore.set(data ? data : []);

    return {
        colors,
        pages: data,
        users,
    };
}) satisfies PageServerLoad;

