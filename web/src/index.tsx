/* @refresh reload */
import { render } from "solid-js/web"
import { Route, Router } from "@solidjs/router"

import "./reset.css"
import "./index.css"
import App from "./App"
import Find from "./views/Find"
import Galaxy from "./views/Galaxy"

const root = document.getElementById("root")

const dispose = render(
    () => (
        <Router root={App}>
            <Route path="/" component={Find} />
            <Route path="/galaxy/:seed?/:index?" component={Galaxy} />
        </Router>
    ),
    root!,
)

if (import.meta.hot) {
    import.meta.hot.dispose(dispose)
}
