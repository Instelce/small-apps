import { z } from "zod";
import { get, post } from ".";
import { authStore } from "$lib/store/auth";


// Validation Schemas
export const loginSchema = z.object({
    email: z.string().email(),
    password: z.string(),
});

export type LoginSchema = typeof loginSchema;

export const joinSchema = z.object({
    name: z.string().min(2).max(50),
    email: z.string().email(),
    password: z.string().min(8),
})

export type JoinSchema = typeof joinSchema;


// Params
export type LoginParams = z.infer<typeof loginSchema>;
export type JoinParams = z.infer<typeof joinSchema>;


// Responses
export type LoginResponse = {
    token: string;
    pid: string;
    email: string;
    isVerified: boolean;
};

export type JoinResponse = {}

export const login = async (params: LoginParams) => {
    let { response, data, errors } = await post<LoginParams, LoginResponse>("/auth/login", params);

    // store token
    if (data) {
        authStore.update(store => {
            store.token = data.token;
            return store;
        });
    }

    return { data, errors }
}

export const join = async (params: JoinParams) => {
    let { data, errors } = await post<JoinParams, JoinResponse>("/auth/register", params);
    return { data, errors }
}