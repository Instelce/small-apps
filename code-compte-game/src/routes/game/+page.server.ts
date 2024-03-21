import { arrayChoice, arrayMultipleChoice, arrayShuffle, randomInt } from '$lib/utils';
import type { PageServerLoad } from './$types';
import {data} from "../data";


export const load = (async ({params, url}) => {
    return {classes: data, gameType: url.searchParams.get('type')}
}) satisfies PageServerLoad;