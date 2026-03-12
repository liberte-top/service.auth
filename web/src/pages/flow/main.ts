import "../../app.css";
import FlowPage from "./FlowPage.svelte";

const app = new FlowPage({
  target: document.getElementById("app")!,
});

export default app;
