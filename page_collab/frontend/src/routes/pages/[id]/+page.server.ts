import { pages } from '$lib/api/pages';
import { pagesStore } from '$lib/store/pages';
import type { PageServerLoad } from './$types';

export const load = (async ({ params }) => {
    let id = parseInt(params.id);
    let page = pagesStore.get(id);

    if (!page) {
        let response = await pages.get(id);
        if (response.data) {
            page = response.data;
        }
    }

    return {
        page
    };
}) satisfies PageServerLoad;