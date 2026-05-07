import { Component, createMemo, createSignal, For, JSX } from "solid-js"
import styles from "./Starmap.module.css"
import { A, useNavigate } from "@solidjs/router"
import { StarType } from "../enums"
import { Portal } from "solid-js/web"
import { computePosition, flip } from "@floating-ui/dom"

type Color = [number, number, number]

function toRGB(color: string): Color {
    return [
        parseInt(color.slice(1, 3), 16),
        parseInt(color.slice(3, 5), 16),
        parseInt(color.slice(5), 16),
    ]
}

const colors: [number, Color][] = [
    [0, toRGB("#fe243b")],
    [0.05400363728404045, toRGB("#fe902f")],
    [0.08210773766040802, toRGB("#feb524")],
    [0.09922304004430771, toRGB("#fec721")],
    [0.15128976106643677, toRGB("#fef71e")],
    [0.23878754675388336, toRGB("#fefa00")],
    [0.27973926067352295, toRGB("#fefe00")],
    [0.3280837833881378, toRGB("#fefe07")],
    [0.42586174607276917, toRGB("#fefe98")],
    [0.5108104348182678, toRGB("#fefef3")],
    [0.54461669921875, toRGB("#fefefe")],
    [0.7830636501312256, toRGB("#fefefe")],
    [0.8255955576896667, toRGB("#cafefe")],
    [0.8672537803649902, toRGB("#43fefe")],
    [0.883392870426178, toRGB("#00fefe")],
    [0.9545682668685913, toRGB("#01d3fe")],
    [1, toRGB("#0072fe")],
]

const neutronStarColor = "#b685fe"
const blackHoleColor = "#6d40b1" // not black because it is not very visible

function getStarColor(color: float) {
    if (color >= 1) return colors[colors.length - 1]![1]
    if (color <= 0) return colors[0]![1]
    const index = colors.findLastIndex(([v]) => v <= color)
    const color1 = colors[index]!
    if (color1[0] === color) return color1[1]
    const color2 = colors[index + 1]!
    const t = (color - color1[0]) / color2[0] - color1[0]
    return color1[1].map((c, i) => c + t * (color2[1][i]! - c)) as Color
}

function sqrDistance([x1, y1, z1]: Position, [x2, y2, z2]: Position) {
    const x = x2 - x1
    const y = y2 - y1
    const z = z2 - z1
    return x * x + y * y + z * z
}

function getConnectors(stars: Star[]) {
    const connectors = new Map<Star, Star[]>()
    const lines = new Map<Star, Set<Star>>()

    function addLine(star1: Star, star2: Star) {
        if (lines.has(star1)) {
            lines.get(star1)!.add(star2)
        } else {
            lines.set(star1, new Set([star2]))
        }
        if (lines.has(star2)) {
            lines.get(star2)!.add(star1)
        } else {
            lines.set(star2, new Set([star1]))
        }
    }

    function removeLine(star1: Star, star2: Star) {
        lines.get(star1)?.delete(star2)
        lines.get(star2)?.delete(star1)
    }

    for (const star of stars) {
        const conns: Star[] = []
        connectors.set(star, conns)
        for (let i = 0; i < star.index; ++i) {
            const otherStar = stars[i]!
            const dist = sqrDistance(star.position, otherStar.position)
            if (dist < 64) {
                conns.push(otherStar)
                const otherConns = connectors.get(otherStar)!
                otherConns.push(star)
                otherConns.sort((a, b) => a.index - b.index)
            }
        }
        const tmpState: Record<number, number> = {}
        conns.forEach((otherStar, index1) => {
            const otherConns = connectors.get(otherStar)!
            for (let index2 = index1 + 1; index2 < conns.length; ++index2) {
                const thirdStar = conns[index2]!
                const hasTrangle = otherConns.find((s) => s === thirdStar)
                if (hasTrangle) {
                    const dist12 = sqrDistance(
                        star.position,
                        otherStar.position,
                    )
                    const dist13 = sqrDistance(
                        star.position,
                        thirdStar.position,
                    )
                    const dist23 = sqrDistance(
                        otherStar.position,
                        thirdStar.position,
                    )
                    const maxDist = Math.max(dist12, dist13, dist23)
                    if (maxDist === dist12) {
                        tmpState[index1] = -1
                        removeLine(star, otherStar)
                    } else if (tmpState[index1] === undefined) {
                        tmpState[index1] = 1
                        addLine(star, otherStar)
                    }
                    if (maxDist === dist13) {
                        tmpState[index2] = -1
                        removeLine(star, thirdStar)
                    } else if (tmpState[index2] === undefined) {
                        tmpState[index2] = 1
                        addLine(star, thirdStar)
                    }
                    if (maxDist === dist23) {
                        removeLine(otherStar, thirdStar)
                    }
                }
            }
            if (tmpState[index1] === undefined) {
                addLine(star, otherStar)
                tmpState[index1] = 1
            }
        })
    }

    const output: [Position, Position][] = Array.from(lines.entries()).flatMap(
        ([s1, conns]) =>
            [...conns]
                .filter((s2) => s1.index < s2.index)
                .map((s2) => [s1.position, s2.position]),
    )
    return output
}

const StarNode: Component<{ star: Star; seed: number; search: string }> = (
    props,
) => {
    const navigate = useNavigate()
    const [hover, setHover] = createSignal(false)
    let node: SVGCircleElement
    let popup: HTMLAnchorElement

    const color = createMemo(() =>
        props.star.type === StarType.BlackHole
            ? blackHoleColor
            : props.star.type === StarType.NeutronStar
              ? neutronStarColor
              : `rgb(${getStarColor(props.star.color).join(", ")})`,
    )

    const url = createMemo(
        () => `/galaxy/${props.seed}/${props.star.index}${props.search}`,
    )

    function placePopup() {
        computePosition(node!, popup!, {
            strategy: "fixed",
            placement: "top",
            middleware: [flip({ fallbackPlacements: ["bottom"] })],
        }).then(({ x, y }) => {
            popup!.style.left = x + "px"
            popup!.style.top = y + "px"
        })
    }

    function getStarStyle(): JSX.CircleSVGAttributes<SVGCircleElement> {
        const { type, position } = props.star
        let size = 0.4
        if (type === StarType.GiantStar) {
            size *= 2
        } else if (type === StarType.WhiteDwarf) {
            size /= 2
        }
        return {
            r: size,
            cx: position[0],
            cy: -position[2],
            fill: color(),
            "stroke-width": 1,
            stroke: "transparent",
            onClick: () => navigate(url()),
            onMouseEnter: () => {
                placePopup()
                setHover(true)
            },
            onMouseLeave: () => {
                setHover(false)
            },
        }
    }

    return (
        <>
            <circle ref={node!} class={styles.star} {...getStarStyle()} />
            <Portal mount={document.getElementById("portal")!}>
                <A
                    href={url()}
                    ref={popup!}
                    style={{
                        color: color(),
                        display: hover() ? "block" : "none",
                    }}
                    class={styles.popup}
                >
                    {props.star.name}
                </A>
            </Portal>
        </>
    )
}

const Starmap: Component<{ galaxy: Galaxy; search: string }> = (props) => {
    function getViewBox() {
        let top = -Infinity
        let bottom = Infinity
        let left = Infinity
        let right = -Infinity

        for (const star of props.galaxy.stars) {
            const [x, , y] = star.position
            top = Math.max(top, y)
            bottom = Math.min(bottom, y)
            left = Math.min(left, x)
            right = Math.max(right, x)
        }
        top += 2
        bottom -= 2
        left -= 2
        right += 2
        return `${left} ${-top} ${right - left} ${top - bottom}`
    }

    return (
        <svg
            viewBox={getViewBox()}
            preserveAspectRatio="xMidYMid slice"
            class={styles.starmap}
        >
            <For each={getConnectors(props.galaxy.stars)}>
                {([[x1, , y1], [x2, , y2]]) => (
                    <line
                        x1={x1}
                        y1={-y1}
                        x2={x2}
                        y2={-y2}
                        stroke-width={0.07}
                        stroke="#666"
                    />
                )}
            </For>
            <For each={props.galaxy.stars}>
                {(star) => (
                    <StarNode
                        star={star}
                        seed={props.galaxy.seed}
                        search={props.search}
                    />
                )}
            </For>
        </svg>
    )
}

export default Starmap
