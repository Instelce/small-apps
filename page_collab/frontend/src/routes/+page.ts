import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { joinSchema, loginSchema } from '$lib/api/auth';

export const load = (async ({ }) => {

    return {
        loginForm: await superValidate(zod(loginSchema)),
        joinForm: await superValidate(zod(joinSchema))
    };
}) satisfies PageLoad;
