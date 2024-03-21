import type { PageServerLoad } from './$types';
import {data} from "../data"


export const load = (async () => {
    return {classes: data};
}) satisfies PageServerLoad;
