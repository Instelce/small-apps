import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { authStore, isAuthentificated } from '$lib/store/auth';
import { alreadyLoggedIn } from '$lib/api/auth';
import { get } from 'svelte/store';

export const load = (async ({ url, cookies }) => {
    console.log("Auth ", get(authStore), isAuthentificated() ? "Authentificated" : "");

    if (!isAuthentificated()) {
        let token = cookies.get("token");

        console.log("Token ? ", token);

        if (token) {
            await alreadyLoggedIn(token);
        }
    }

    if (!isAuthentificated() && url.pathname !== "/") {
        redirect(307, "/");
    }

    if (isAuthentificated() && url.pathname === "/") {
        redirect(307, "/pages");
    }

    return {
        auth: get(authStore)
    };
}) satisfies LayoutServerLoad;