import "../../app.css";
import ProfilePage from "./ProfilePage.svelte";

const app = new ProfilePage({
  target: document.getElementById("app")!,
});

export default app;
