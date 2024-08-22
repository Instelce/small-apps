import { z } from "zod"
import { get, post, type ListResponse } from "."
import type { UserAvatar } from "./user"


// Schemas
export const newPageSchema = z.object({
    name: z.string(),
});

export type NewPageSchema = typeof newPageSchema;


// Params
export type NewPageParams = z.infer<typeof newPageSchema>;


// Responses
export type Page = {
    id: number,
    name: string,
    content?: string
}

export type DetailPage = {
    id: number,
    name: string,
    content?: string
    collaborators: UserAvatar[]
}


export const pages = {
    list: async () => {
        return await get<ListResponse<DetailPage>>("/pages", true);
    },
    new: async (params: NewPageParams) => {
        return await post<NewPageParams, Page>("/pages", params, true);
    },
    get: async (id: number) => {
        return await get<DetailPage>(`/pages/${id}`, true);
    }
}