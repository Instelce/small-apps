import { authStore, isAuthentificated } from '$lib/store/auth';
import { get } from 'svelte/store';
import type { PageServerLoad } from './$types';
import { pages } from '$lib/api/pages';
import { user } from '$lib/api/user';

export const load = (async ({ }) => {
    let { data, errors } = await user.pages();
    return {
        pages: data,
        users: data ? data.map((page) => page.collaborators).flat() : [],
    };
}) satisfies PageServerLoad;

