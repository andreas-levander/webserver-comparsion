import { Elysia, error } from "elysia";

import data from "./data"

const app = new Elysia()
.get("/", () => "Hello Elysia")
.get("/user", ({query: { username }}) => {
  const user = data.find(u => u.Username == username)
  if (user) return user
  return error(404)
  })
.listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
