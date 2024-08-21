import { get, type ListResponse } from "."
import type { DetailPage } from "./pages";

export type UserAvatar = {
    name: string,
    avatar?: string,
}

export const user = {
    pages: async () => {
        return await get<ListResponse<DetailPage>>("/user/current/pages", true);
    }
}