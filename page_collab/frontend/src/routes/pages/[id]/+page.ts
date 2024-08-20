import type { PageLoad } from './$types';

export const load = (async ({params}: PageLoad) => {
    return { params };
}) satisfies PageLoad;