import { PUBLIC_API_HOST } from "$env/static/public"
import { authStore } from "$lib/store/auth";
import { get as getStore } from "svelte/store";

export type ResponseError = {
    error: string,
    description: string,
}

export type ListResponse<T> = T[];

const base = async (uri: string, auth?: boolean, options?: RequestInit) => {
    let _options: RequestInit = {
        credentials: "include",
        ...options
    };

    _options.headers = {
        "Content-Type": "application/json",
        ...options?.headers
    }

    if (auth) {
        let token = getStore(authStore).token;

        _options.headers = {
            "Authorization": "Bearer " + token,
            ..._options.headers
        };
    }

    return await fetch(PUBLIC_API_HOST + uri, _options);
}


export const get = async<R>(uri: string, auth?: boolean, options?: RequestInit) => {
    let _options: RequestInit = {
        ...options
    };

    let response = await base(uri, auth, _options);

    let data: R | null = null;
    let errors: ResponseError | null = null;
    if (response.status !== 200) {
        errors = await response.json();
    } else {
        data = await response.json();
    }

    return { response, data, errors };
}


export const post = async<P, R>(uri: string, body: P, auth?: boolean, options?: RequestInit) => {
    let _options: RequestInit = {
        method: "POST",
        body: JSON.stringify(body),
        ...options
    };

    let response = await base(uri, auth, _options);

    let data: R | null = null;
    let errors: ResponseError | null = null;
    if (response.status !== 200) {
        errors = await response.json();
    } else {
        data = await response.json();
    }

    return { response, data, errors };
}