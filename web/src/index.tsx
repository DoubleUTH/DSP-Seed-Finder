/* @refresh reload */
import { render } from "solid-js/web"
import { Navigate, Route, Router } from "@solidjs/router"

import "./reset.css"
import "./index.css"
import App from "./App"
import Find from "./views/Find"
import Galaxy from "./views/Galaxy"

const root = document.getElementById("root")

const dispose = render(
    () => (
        <Router base="/DSP-Seed-Finder" root={App}>
            <Route path="/find-star/:profileId?" component={Find} />
            <Route path="/galaxy/:seed?/:index?" component={Galaxy} />
            <Route path="" component={() => <Navigate href="/find-star" />} />
        </Router>
    ),
    root!,
)

if (import.meta.hot) {
    import.meta.hot.dispose(dispose)
}
