import { Component } from "solid-js"
import NumberInput from "../components/NumberInput"
import { maxStarCount, minStarCount } from "../util"

const StarCountSelector: Component<{
    class?: string
    value: integer
    onChange: (value: integer) => void
    disabled?: boolean
}> = (props) => {
    return (
        <NumberInput
            class={props.class}
            value={props.value}
            onChange={(value) => props.onChange(value)}
            error={props.value < minStarCount || props.value > maxStarCount}
            emptyValue={0}
            disabled={props.disabled}
        />
    )
}

export default StarCountSelector
