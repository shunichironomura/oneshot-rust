export type Route =
  | { page: "home" }
  | { page: "problems" }
  | { page: "play"; problemId: string }
  | { page: "history"; problemId: string }
  /** `start`: begin a new challenge right away unless one is in progress. */
  | { page: "challenge"; start?: boolean };

export function parseRoute(hash: string): Route {
  const parts = hash
    .replace(/^#\/?/, "")
    .split("/")
    .filter((p) => p !== "")
    .map(decodeURIComponent);
  switch (parts[0]) {
    case "p":
      if (parts[1] === undefined) return { page: "problems" };
      return parts[2] === "history"
        ? { page: "history", problemId: parts[1] }
        : { page: "play", problemId: parts[1] };
    case "problems":
      return { page: "problems" };
    case "challenge":
      return parts[1] === "new" ? { page: "challenge", start: true } : { page: "challenge" };
    default:
      return { page: "home" };
  }
}

export function routeHref(route: Route): string {
  switch (route.page) {
    case "home":
      return "#/";
    case "problems":
      return "#/problems";
    case "play":
      return `#/p/${encodeURIComponent(route.problemId)}`;
    case "history":
      return `#/p/${encodeURIComponent(route.problemId)}/history`;
    case "challenge":
      return route.start === true ? "#/challenge/new" : "#/challenge";
  }
}
