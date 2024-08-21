import { authStore, isAuthentificated } from "$lib/store/auth";
import { type Handle } from "@sveltejs/kit";
import { get } from "svelte/store";


let authRequiredRoutes = [
    "pages"
]

const inAuthRequiredRoutes = (pathname: string) => {
    for (const route of authRequiredRoutes) {
        if (pathname.includes(route)) {
            return true;
        }
    }

    return false;
}

export const handle: Handle = async ({ event, resolve }) => {
    let response = await resolve(event);

    if (!isAuthentificated() && inAuthRequiredRoutes(event.url.pathname)) {
        // console.log("redirect to login");
    }

    return response;
}