import { Component, Index, createSignal } from "solid-js"
import styles from "./MultiRuleEditor.module.css"
import Modal from "../components/Modal"
import RuleEditor from "./RuleEditor"
import Tooltip from "../components/Tooltip"
import Button from "../components/Button"
import Input from "../components/Input"
import Select from "../components/Select"
import { ConditionType } from "../enums"
import { conditionTypeNames } from "../util"
import NumberInput from "../components/NumberInput"

const MultiRuleEditor: Component<{
    value: MultiRules
    onChange: (value: MultiRules) => void
    disabled?: boolean
}> = (props) => {
    const [editing, setEditing] = createSignal<number | null>(null)

    function editRow(
        index: number,
        fn: (v: MultiRules[number]) => MultiRules[number],
    ) {
        props.onChange(props.value.map((x, i) => (index === i ? fn(x) : x)))
    }

    return (
        <>
            <table class={styles.table}>
                <thead>
                    <tr class={styles.titleRow}>
                        <td />
                        <td>Rules</td>
                        <td>
                            <Tooltip text="Number of stars that matches the rules">
                                Number of stars
                            </Tooltip>
                        </td>
                        <td>Name</td>
                    </tr>
                </thead>
                <tbody>
                    <Index each={props.value}>
                        {(row, index) => (
                            <tr>
                                <td />
                                <td>
                                    <Button
                                        kind="outline"
                                        onClick={() => setEditing(index)}
                                    />
                                </td>
                                <td>
                                    <Select
                                        value={row().condition.type}
                                        onChange={(type) =>
                                            editRow(index, (r) => ({
                                                ...r,
                                                condition: {
                                                    ...r.condition,
                                                    type,
                                                },
                                            }))
                                        }
                                        options={[
                                            ConditionType.Gte,
                                            ConditionType.Lte,
                                            ConditionType.Eq,
                                            ConditionType.Neq,
                                        ]}
                                        getLabel={(type) =>
                                            conditionTypeNames[type]
                                        }
                                        disabled={props.disabled}
                                    />
                                    <NumberInput
                                        value={row().condition.value}
                                        onChange={(value) =>
                                            editRow(index, (r) => ({
                                                ...r,
                                                condition: {
                                                    ...r.condition,
                                                    value,
                                                },
                                            }))
                                        }
                                        emptyValue={-1}
                                        disabled={props.disabled}
                                        maxLength={2}
                                        error={row().condition.value <= 0}
                                    />
                                </td>
                                <td>
                                    <Input
                                        value={row().name}
                                        onChange={(name) =>
                                            editRow(index, (r) => ({
                                                ...r,
                                                name,
                                            }))
                                        }
                                    />
                                </td>
                            </tr>
                        )}
                    </Index>
                </tbody>
            </table>
            <Button kind="outline" class={styles.addNewRow}>
                Add new row
            </Button>
            <Modal
                visible={editing() !== null}
                onClose={() => setEditing(null)}
            >
                <RuleEditor
                    value={props.value[editing()!]!.rules}
                    onChange={(rules) =>
                        props.onChange(
                            props.value.map((x, i) =>
                                i === editing() ? { ...x, rules } : x,
                            ),
                        )
                    }
                />
            </Modal>
        </>
    )
}

export default MultiRuleEditor
