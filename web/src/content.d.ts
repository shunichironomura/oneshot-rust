declare module "virtual:oneshot-content" {
  import type { Content } from "./core/problem.ts";

  const content: Content;
  export default content;
}

declare module "virtual:oneshot-solutions" {
  import type { ModelSolution } from "./core/problem.ts";

  /** Loads the model solution of a problem (one chunk per problem). */
  const solutions: Record<string, () => Promise<ModelSolution>>;
  export default solutions;
}
