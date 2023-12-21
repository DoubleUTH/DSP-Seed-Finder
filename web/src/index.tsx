/* @refresh reload */
import { render } from "solid-js/web"

import "./index.css"
import App from "./App"

const root = document.getElementById("root")

const dispose = render(() => <App />, root!)

if (import.meta.hot) {
    import.meta.hot.dispose(dispose)
}
