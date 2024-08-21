import { superValidate } from 'sveltekit-superforms';
import type { PageServerLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { newPageSchema, pages } from '$lib/api/pages';
import { redirect } from '@sveltejs/kit';

export const load = (async () => {
    return {
        form: await superValidate(zod(newPageSchema))
    };
}) satisfies PageServerLoad;

export const actions = {
    default: async (event) => {
        const form = await superValidate(event, zod(newPageSchema));

        if (form.valid) {
            let data = form.data;
            let response = await pages.new(data);

            console.log(response);

            if (response.data) {
                // redirect to the new page
                redirect(307, `/pages/${response.data.id}`);
            }

        }
    }
}