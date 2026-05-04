import { Component } from "solid-js"
import Select from "../components/Select"
import { hiveInitialColonizeValues } from "../util"

const HiveInitialColonizeSelector: Component<{
    class?: string
    value: float
    onChange: (value: float) => void
    disabled?: boolean
}> = (props) => {
    return (
        <Select
            class={props.class}
            value={props.value}
            onChange={(v) => props.onChange(v)}
            options={hiveInitialColonizeValues}
            getLabel={(x) => x * 100 + "%"}
            disabled={props.disabled}
        />
    )
}

export default HiveInitialColonizeSelector
