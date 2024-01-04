/* @refresh reload */
import { render } from "solid-js/web"
import { Navigate, Route, Router } from "@solidjs/router"

import "./reset.css"
import "./index.css"
import App from "./App"
import FindStar from "./views/FindStar"
import Galaxy from "./views/Galaxy"
import FindGalaxy from "./views/FindGalaxy"

const root = document.getElementById("root")

const dispose = render(
    () => (
        <Router base="/DSP-Seed-Finder" root={App}>
            <Route path="/find-star/:profileId?" component={FindStar} />
            <Route path="/find-galaxy/:profileId?" component={FindGalaxy} />
            <Route path="/galaxy/:seed?/:index?" component={Galaxy} />
            <Route path="" component={() => <Navigate href="/find-star" />} />
        </Router>
    ),
    root!,
)

if (import.meta.hot) {
    import.meta.hot.dispose(dispose)
}
