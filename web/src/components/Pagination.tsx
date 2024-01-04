import { IoChevronBack, IoChevronForward } from "solid-icons/io"
import NumberInput from "./NumberInput"
import { Component, createSignal } from "solid-js"
import styles from "./Pagination.module.css"

const Pagination: Component<{
    current: integer
    total: integer
    onChange: (page: integer) => void
}> = (props) => {
    // eslint-disable-next-line solid/reactivity
    const [page, setPage] = createSignal(props.current)

    function onChange(value: number) {
        if (value >= 1 && value <= props.total) {
            props.onChange(value)
            setPage(value)
        }
    }

    function handleSubmit(ev: Event) {
        ev.preventDefault()
        onChange(page())
    }

    return (
        <form class={styles.pagination} onSubmit={handleSubmit}>
            <button
                type="button"
                class={styles.paginationButton}
                disabled={props.current <= 1}
                onClick={() => onChange(props.current - 1)}
            >
                <IoChevronBack />
            </button>
            Page{" "}
            <NumberInput
                class={styles.paginationInput}
                value={page()}
                onChange={setPage}
                onBlur={() => onChange(page())}
                emptyValue={-1}
                error={
                    !Number.isInteger(page()) ||
                    page() <= 0 ||
                    page() > props.total
                }
            />{" "}
            of {props.total}{" "}
            <button
                type="button"
                class={styles.paginationButton}
                disabled={props.current >= props.total}
                onClick={() => onChange(props.current + 1)}
            >
                <IoChevronForward />
            </button>
        </form>
    )
}

export default Pagination
