import type { DetailPage } from "$lib/api/pages";
import { get, writable } from "svelte/store";


const _pagesStore = writable<DetailPage[]>([]);

export const pagesStore = {
    all: () => {
        return get(_pagesStore);
    },
    get: (id: number) => {
        return get(_pagesStore).find(page => page.id === id);
    },
    set: (value: DetailPage[]) => {
        _pagesStore.set(value)
    },
    store: _pagesStore
}
