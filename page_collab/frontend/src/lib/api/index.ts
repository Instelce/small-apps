import { PUBLIC_API_HOST } from "$env/static/public";

export async function api(uri: string, options: RequestInit) {
  let response = await fetch(PUBLIC_API_HOST + uri, {
    headers: {
      "Content-Type": "application/json",
    },
    ...options,
  });
  let value = await response.json();

  if (value == undefined) {
    return {};
  } else {
    return value;
  }
}
