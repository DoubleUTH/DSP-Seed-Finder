import { Component, Index, Show, createSignal } from "solid-js"
import Modal from "../components/Modal"
import RuleEditor from "./RuleEditor"
import Button from "../components/Button"
import Input from "../components/Input"
import Select from "../components/Select"
import { ConditionType } from "../enums"
import { validateRules } from "../util"
import NumberInput from "../components/NumberInput"
import { IoTrash } from "solid-icons/io"
import styles from "./MultiRuleEditor.module.css"
import { Trans, useLingui } from "#lingui"
import { useConditionTypeNames } from "../names"

const DeleteButton: Component<{ onDelete: () => void }> = (props) => {
    return (
        <div class={styles.delete} onClick={() => props.onDelete()}>
            <IoTrash />
        </div>
    )
}

const defaultMultiRule: MultiRule = {
    name: "",
    rules: [],
    condition: { type: ConditionType.Gte, value: 1 },
}

const RulesetButton: Component<{
    rules: SimpleRule[][]
    onEdit: () => void
}> = (props) => {
    const { t } = useLingui()
    return (
        <Button
            kind="outline"
            class={styles.buttonRuleset}
            onClick={() => props.onEdit()}
            theme={validateRules(props.rules) ? "default" : "error"}
        >
            {t`this ruleset`}
        </Button>
    )
}

const RuleBlockContent: Component<{
    value: MultiRule[]
    onChange: (value: MultiRule[]) => void
    onEdit: (index: integer) => void
    disabled?: boolean
}> = (props) => {
    function onDelete(index: number) {
        props.onChange(props.value.filter((_, i) => i !== index))
    }
    function onAdd() {
        props.onChange([...props.value, defaultMultiRule])
    }

    function editItem(index: number, fn: (v: MultiRule) => MultiRule) {
        props.onChange(props.value.map((x, i) => (index === i ? fn(x) : x)))
    }

    const { t } = useLingui()
    const conditionTypeNames = useConditionTypeNames()

    return (
        <>
            <Index each={props.value}>
                {(item, index) => (
                    <>
                        <Show when={index > 0}>
                            <div class={styles.or}>{t`OR`}</div>
                        </Show>
                        <div class={styles.row}>
                            <div class={styles.editRow}>
                                <Trans>
                                    Has{" "}
                                    <Select
                                        class={styles.selectConditionType}
                                        value={item().condition.type}
                                        onChange={(type) =>
                                            editItem(index, (r) => ({
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
                                        ]}
                                        getLabel={(type) =>
                                            conditionTypeNames[type]()
                                        }
                                        disabled={props.disabled}
                                    />{" "}
                                    <NumberInput
                                        class={styles.inputCount}
                                        value={item().condition.value}
                                        onChange={(value) =>
                                            editItem(index, (r) => ({
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
                                        error={item().condition.value <= 0}
                                    />{" "}
                                    star(s) that satisfy{" "}
                                    <RulesetButton
                                        rules={item().rules}
                                        onEdit={() => props.onEdit(index)}
                                    />
                                    . Description:{" "}
                                </Trans>
                            </div>
                            <Input
                                class={styles.description}
                                value={item().name}
                                onChange={(name) =>
                                    editItem(index, (r) => ({
                                        ...r,
                                        name,
                                    }))
                                }
                            />
                            <Show when={!props.disabled}>
                                <DeleteButton
                                    onDelete={() => onDelete(index)}
                                />
                            </Show>
                        </div>
                    </>
                )}
            </Index>

            <Show when={!props.disabled}>
                <Button class={styles.addOr} kind="outline" onClick={onAdd}>
                    {t`Add OR rule`}
                </Button>
            </Show>
        </>
    )
}

const MultiRuleEditor: Component<{
    value: MultiRule[][]
    onChange: (value: MultiRule[][]) => void
    disabled?: boolean
}> = (props) => {
    const [editing, setEditing] = createSignal<[number, number] | null>(null)

    function onRulesChange(rules: SimpleRule[][]) {
        const [ei, ej] = editing()!
        props.onChange(
            props.value.map((x, i) =>
                i === ei
                    ? x.map((y, j) => (j === ej ? { ...y, rules } : y))
                    : x,
            ),
        )
    }

    function onBlockChange(group: MultiRule[], index: number) {
        if (group.length > 0) {
            props.onChange(props.value.map((v, i) => (i === index ? group : v)))
        } else {
            const result = props.value.filter((_, i) => i !== index)
            if (result.length === 0) {
                props.onChange([[defaultMultiRule]])
            } else {
                props.onChange(result)
            }
        }
    }

    function onAdd() {
        props.onChange([...props.value, [defaultMultiRule]])
    }

    const { t } = useLingui()

    return (
        <div class={styles.multiRuleEditor}>
            <Index each={props.value}>
                {(group, index) => (
                    <>
                        <Show when={index > 0}>
                            <div class={styles.and}>{t`AND`}</div>
                        </Show>
                        <div class={styles.block}>
                            <RuleBlockContent
                                value={group()}
                                onChange={(group) =>
                                    onBlockChange(group, index)
                                }
                                onEdit={(i) => setEditing([index, i])}
                                disabled={props.disabled}
                            />
                        </div>
                    </>
                )}
            </Index>
            <Show when={!props.disabled}>
                <Button class={styles.addAnd} kind="outline" onClick={onAdd}>
                    {t`Add AND rule`}
                </Button>
            </Show>
            <Show when={editing()}>
                {(editing) => (
                    <Modal
                        visible
                        onClose={() => setEditing(null)}
                        backdropDismiss
                    >
                        <div class={styles.ruleBuilderTitle}>{t`Ruleset`}</div>
                        <RuleEditor
                            class={styles.ruleEditor}
                            value={
                                props.value[editing()[0]]![editing()[1]]!.rules
                            }
                            onChange={onRulesChange}
                            disabled={props.disabled}
                        />
                    </Modal>
                )}
            </Show>
        </div>
    )
}

export default MultiRuleEditor
