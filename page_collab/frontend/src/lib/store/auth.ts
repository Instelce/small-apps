import { get, writable } from "svelte/store";

export const authStore = writable({ token: "", user: "" })

export const isAuthentificated = (): boolean => {
    let token = get(authStore).token;

    if (token === "") {
        return false
    }

    return true;
}