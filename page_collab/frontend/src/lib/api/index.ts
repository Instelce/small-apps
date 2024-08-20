import { PUBLIC_API_HOST } from "$env/static/public"
import { authStore } from "$lib/store/auth";
import { get as getStore } from "svelte/store";

export type ResponseError = {
    error: string,
    description: string,
}

const base = async (uri: string, options?: RequestInit) => {
    let _options: RequestInit = {
        headers: {
            "Content-Type": "application/json"
        },
        ...options
    };

    return await fetch(PUBLIC_API_HOST + uri, _options);
}


export const get = async (uri: string, auth?: boolean, options?: RequestInit) => {
    let _options: RequestInit = {
        ...options
    };

    if (auth) {
        let token = getStore(authStore).token;

        _options.headers = {
            "Authorization": "Bearer " + token,
            ..._options.headers
        };
    }

    return await base(uri, _options);
}


export const post = async<P, R>(uri: string, body: P, auth?: boolean, options?: RequestInit) => {
    let _options: RequestInit = {
        method: "POST",
        body: JSON.stringify(body),
        ...options
    };

    if (auth) {
        let token = getStore(authStore).token;

        _options.headers = {
            "Authorization": "Bearer " + token,
            ..._options.headers
        };
    }

    let response = await base(uri, _options);
    let data: R | null = null;
    let errors: ResponseError | null = null;
    if (response.status !== 200) {
        errors = await response.json();
    } else {
        data = await response.json();
    }
    return { response, data, errors };
}