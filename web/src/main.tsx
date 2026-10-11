import { render } from "preact";
import { App } from "./app.tsx";
// The scorecard's typefaces: the printed part, and the pencil.
import "@fontsource/barlow-condensed/600.css";
import "@fontsource/barlow-condensed/700.css";
import "@fontsource/caveat/600.css";
import "@fontsource/caveat/700.css";
import "./styles.css";

const root = document.getElementById("app");
if (root === null) throw new Error("#app not found");
render(<App />, root);
