declare module "virtual:oneshot-content" {
  import type { Content } from "./core/problem.ts";

  const content: Content;
  export default content;
}
