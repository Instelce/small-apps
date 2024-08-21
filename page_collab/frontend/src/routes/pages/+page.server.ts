import { authStore, isAuthentificated } from '$lib/store/auth';
import { get } from 'svelte/store';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {

    return {};
}) satisfies PageServerLoad;

