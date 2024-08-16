import { PUBLIC_API_HOST } from "$env/static/public";
import type { Infer, SuperForm, SuperValidated } from "sveltekit-superforms";
import { z } from "zod";
import { api } from ".";

export const registerSchema = z.object({
  username: z.string().min(2).max(50),
  email: z.string().email(),
  password: z.string().min(8),
});

export type RegisterSchema = typeof registerSchema;

export const loginSchema = z.object({
  email: z.string().email(),
  password: z.string(),
});

export type LoginSchema = typeof loginSchema;

export const login = async (form: SuperForm<Infer<LoginSchema>>) => {
  let data;
  const unsubscribe = form.form.subscribe((d) => (data = d));

  let response = await api("/auth/login", {
    body: JSON.stringify(data),
    method: "POST",
  });

  console.log(response);

  unsubscribe();
};
