import { Component } from "solid-js"
import NumberInput from "../components/NumberInput"

const StarCountSelector: Component<{
    class?: string
    value: integer
    onChange: (value: integer) => void
}> = (props) => {
    return (
        <NumberInput
            class={props.class}
            value={props.value}
            onChange={(value) => props.onChange(value)}
            min={32}
            max={64}
            step={1}
            emptyValue={0}
        />
    )
}

export default StarCountSelector
