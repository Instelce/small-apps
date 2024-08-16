import type { PageServerLoad } from "./$types";
import { fail, superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { login, loginSchema, registerSchema } from "$lib/api/auth";

export const prerender = true;

export const load: PageServerLoad = async () => {
  return {
    registerForm: await superValidate(zod(registerSchema)),
    loginForm: await superValidate(zod(loginSchema)),
  };
};

// export const actions: Actions = {
//   login: async (event) => {
//     const form = await superValidate(event, zod(loginSchema));

//     console.log(form.data);
//     if (!form.valid) {
//       return fail(400, {
//         form,
//       });
//     }

//     await login(form);
//   },
// };
