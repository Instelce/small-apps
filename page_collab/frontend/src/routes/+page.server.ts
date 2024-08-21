import type { Actions, PageServerLoad } from './$types';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { join, joinSchema, login, loginSchema, type LoginParams } from '$lib/api/auth';
import { redirect } from '@sveltejs/kit';

export const load = (async () => {
    return {
        loginForm: await superValidate(zod(loginSchema)),
        joinForm: await superValidate(zod(joinSchema))
    };
}) satisfies PageServerLoad;

export const actions: Actions = {
    login: async (event) => {
        const form = await superValidate(event, zod(loginSchema));

        if (form.valid) {
            let data = form.data;
            let response = await login(data);
            console.log(response.data);
            if (response.data) {
                let token = response.data.token;
                event.cookies.set("token", token, { path: '/' });
            }
        }
    },
    join: async (event) => {
        const form = await superValidate(event, zod(joinSchema));

        if (form.valid) {
            let data = form.data;
            let response = await join(data);
        }
    }
}