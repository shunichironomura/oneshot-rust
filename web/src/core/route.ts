export type Route =
  | { page: "list" }
  | { page: "play"; problemId: string }
  | { page: "history"; problemId: string }
  | { page: "challenge" }
  | { page: "settings" };

export function parseRoute(hash: string): Route {
  const parts = hash
    .replace(/^#\/?/, "")
    .split("/")
    .filter((p) => p !== "")
    .map(decodeURIComponent);
  switch (parts[0]) {
    case "p":
      if (parts[1] === undefined) return { page: "list" };
      return parts[2] === "history"
        ? { page: "history", problemId: parts[1] }
        : { page: "play", problemId: parts[1] };
    case "challenge":
      return { page: "challenge" };
    case "settings":
      return { page: "settings" };
    default:
      return { page: "list" };
  }
}

export function routeHref(route: Route): string {
  switch (route.page) {
    case "list":
      return "#/";
    case "play":
      return `#/p/${encodeURIComponent(route.problemId)}`;
    case "history":
      return `#/p/${encodeURIComponent(route.problemId)}/history`;
    case "challenge":
      return "#/challenge";
    case "settings":
      return "#/settings";
  }
}
