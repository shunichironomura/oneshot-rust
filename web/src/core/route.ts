export type Route =
  | { page: "home" }
  /** Records: every problem with its best, and past rounds. */
  | { page: "problems" }
  /** The round in progress, or the screen to start one. */
  | { page: "round" }
  | { page: "scorecard"; roundId: string }
  /** One problem played on its own, outside a round. */
  | { page: "practice"; problemId: string }
  | { page: "history"; problemId: string };

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
        : { page: "practice", problemId: parts[1] };
    case "problems":
      return { page: "problems" };
    case "round":
      return parts[1] === undefined ? { page: "round" } : { page: "scorecard", roundId: parts[1] };
    // Links from before rounds replaced the 連続チャレンジ.
    case "challenge":
      return { page: "round" };
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
    case "round":
      return "#/round";
    case "scorecard":
      return `#/round/${encodeURIComponent(route.roundId)}`;
    case "practice":
      return `#/p/${encodeURIComponent(route.problemId)}`;
    case "history":
      return `#/p/${encodeURIComponent(route.problemId)}/history`;
  }
}
