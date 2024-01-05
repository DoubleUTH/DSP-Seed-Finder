import { Component, Index, Match, Show, Switch } from "solid-js"
import {
    ConditionType,
    GasType,
    OceanType,
    RuleType,
    SpectrType,
    StarType,
    VeinType,
} from "../enums"
import styles from "./RuleEditor.module.css"
import Select from "../components/Select"
import { IoTrash } from "solid-icons/io"
import Button from "../components/Button"
import NumberInput from "../components/NumberInput"
import { conditionTypeNames, planetTypes, veinNames } from "../util"
import clsx from "clsx"

const SelectSimpleRule: Component<{
    value?: SimpleRule
    onChange: (rule: SimpleRule) => void
    disabled?: boolean
}> = (props) => {
    return (
        <Select
            class={styles.selectRule}
            value={
                props.value?.type === RuleType.None ? undefined : props.value
            }
            onChange={props.onChange}
            isSelected={(rule) => rule.type === props.value?.type}
            options={rules}
            placeholder="Select..."
            getLabel={(rule) => ruleNames[rule.type]}
            error={!props.value || props.value?.type === RuleType.None}
            disabled={props.disabled}
        />
    )
}

const ConditionTypeSelector: Component<{
    value: Condition
    onChange: (value: Condition) => void
    disabled?: boolean
}> = (props) => (
    <Select
        class={styles.selectConditionType}
        value={props.value.type}
        onChange={(type) => props.onChange({ ...props.value, type })}
        options={[ConditionType.Gte, ConditionType.Lte, ConditionType.Eq]}
        getLabel={(type) => conditionTypeNames[type]}
        disabled={props.disabled}
    />
)

const ConditionValueInput: Component<{
    value: Condition
    onChange: (value: Condition) => void
    disabled?: boolean
    class?: string
    error?: boolean
    emptyValue: number
    maxLength?: number
}> = (props) => (
    <NumberInput
        class={props.class}
        value={props.value.value}
        onChange={(value) => props.onChange({ ...props.value, value })}
        emptyValue={props.emptyValue}
        disabled={props.disabled}
        maxLength={props.maxLength}
        error={props.error}
    />
)

const EditLuminosity: Component<{
    value: Rule.Luminosity
    onChange: (value: Rule.Luminosity) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Is{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputLuminosity}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0 || condition().value >= 3}
                disabled={props.disabled}
            />
            L
        </>
    )
}

const EditDysonRadius: Component<{
    value: Rule.DysonRadius
    onChange: (value: Rule.DysonRadius) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Is{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputDyson}
                maxLength={6}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />
            m
        </>
    )
}

const EditAverageVeinAmount: Component<{
    value: Rule.AverageVeinAmount
    onChange: (value: Rule.AverageVeinAmount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <Select
                class={styles.selectVein}
                value={props.value.vein}
                onChange={(vein) => props.onChange({ ...props.value, vein })}
                options={veins}
                getLabel={(vein) => veinNames[vein]}
                disabled={props.disabled}
            />{" "}
            and the estimated amount is{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputVein}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />
            {props.value.vein === VeinType.Oil ? " /s" : " "}
        </>
    )
}

const EditSpectr: Component<{
    value: Rule.Spectr
    onChange: (value: Rule.Spectr) => void
    disabled?: boolean
}> = (props) => {
    return (
        <>
            Is a{" "}
            <Select
                class={styles.selectSpectr}
                value={props.value.spectr[0]}
                onChange={(spectr) =>
                    props.onChange({ ...props.value, spectr: [spectr] })
                }
                options={spectrs}
                getLabel={(spectr) => spectr}
                disabled={props.disabled}
            />{" "}
            type star
        </>
    )
}

const EditTidalLockCount: Component<{
    value: Rule.TidalLockCount
    onChange: (value: Rule.TidalLockCount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />{" "}
            tidally locked planets
        </>
    )
}

const EditOceanType: Component<{
    value: Rule.OceanType
    onChange: (value: Rule.OceanType) => void
    disabled?: boolean
}> = (props) => {
    return (
        <>
            Has planets with{" "}
            <Select
                class={styles.selectOcean}
                value={props.value.oceanType}
                onChange={(oceanType) =>
                    props.onChange({ ...props.value, oceanType })
                }
                options={oceans}
                getLabel={(oceanType) =>
                    oceanType === OceanType.Water ? "Water" : "Sulfuric Acid"
                }
                disabled={props.disabled}
            />{" "}
            Ocean
        </>
    )
}

const EditStarType: Component<{
    value: Rule.StarType
    onChange: (value: Rule.StarType) => void
    disabled?: boolean
}> = (props) => {
    return (
        <>
            Is a{" "}
            <Select
                class={styles.selectStarType}
                value={props.value.starType[0]}
                onChange={(starType) =>
                    props.onChange({ ...props.value, starType: [starType] })
                }
                options={starTypes}
                getLabel={(starType) => starTypeNames[starType]}
                disabled={props.disabled}
            />
        </>
    )
}

const EditGasCount: Component<{
    value: Rule.GasCount
    onChange: (value: Rule.GasCount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />{" "}
            <Select
                class={styles.selectGas}
                value={props.value.ice}
                onChange={(ice) => props.onChange({ ...props.value, ice })}
                options={[null, false, true]}
                getLabel={(ice) =>
                    ice === null ? "gas/ice" : ice ? "ice" : "gas"
                }
                disabled={props.disabled}
            />{" "}
            giant(s)
        </>
    )
}

const EditSatelliteCount: Component<{
    value: Rule.SatelliteCount
    onChange: (value: Rule.SatelliteCount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />{" "}
            satellite(s)
        </>
    )
}

const EditPlanetCount: Component<{
    value: Rule.PlanetCount
    onChange: (value: Rule.PlanetCount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 1}
                disabled={props.disabled}
            />{" "}
            planets,{" "}
            <Select
                class={styles.selectGasType}
                value={props.value.excludeGiant}
                onChange={(excludeGiant) =>
                    props.onChange({ ...props.value, excludeGiant })
                }
                options={[false, true]}
                getLabel={(excludeGiant) =>
                    excludeGiant ? "excluding" : "including"
                }
                disabled={props.disabled}
            />{" "}
            gas/ice giants.
        </>
    )
}

const EditBirthDistance: Component<{
    value: Rule.BirthDistance
    onChange: (value: Rule.BirthDistance) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Is{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputDistance}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />
            ly away from the starting system
        </>
    )
}

const EditXDistance: Component<{
    value: Rule.XDistance
    onChange: (value: Rule.XDistance) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Is{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputDistance}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />
            ly away from any black hole / neutron star.
        </>
    )
}

const EditSpectrDistance: Component<{
    value: Rule.SpectrDistance
    onChange: (value: Rule.SpectrDistance) => void
    disabled?: boolean
}> = (props) => {
    const countCondition = () => props.value.countCondition
    const setCountCondition = (countCondition: Condition) =>
        props.onChange({ ...props.value, countCondition })
    const distanceCondition = () => props.value.distanceCondition
    const setDistanceCondition = (distanceCondition: Condition) =>
        props.onChange({ ...props.value, distanceCondition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={countCondition()}
                onChange={setCountCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={countCondition()}
                onChange={setCountCondition}
                emptyValue={-1}
                error={countCondition().value <= 0}
                disabled={props.disabled}
            />{" "}
            <Select
                class={styles.selectSpectr}
                value={props.value.spectr}
                onChange={(spectr) =>
                    props.onChange({ ...props.value, spectr })
                }
                options={spectrs}
                getLabel={(spectr) => spectr}
                disabled={props.disabled}
            />{" "}
            type stars that are{" "}
            <ConditionTypeSelector
                value={distanceCondition()}
                onChange={setDistanceCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputDistance}
                value={distanceCondition()}
                onChange={setDistanceCondition}
                emptyValue={-1}
                error={distanceCondition().value <= 0}
                disabled={props.disabled}
            />{" "}
            ly away.
        </>
    )
}

const EditGasRate: Component<{
    value: Rule.GasRate
    onChange: (value: Rule.GasRate) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <Select
                class={styles.selectGasType}
                value={props.value.gasType}
                onChange={(gasType) =>
                    props.onChange({ ...props.value, gasType })
                }
                options={gasTypes}
                getLabel={(vein) => gasTypeNames[vein]}
                disabled={props.disabled}
            />{" "}
            and{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputGasRate}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />
            /s of it
        </>
    )
}

const EditPlanetInDysonCount: Component<{
    value: Rule.PlanetInDysonCount
    onChange: (value: Rule.PlanetInDysonCount) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition
    const setCondition = (condition: Condition) =>
        props.onChange({ ...props.value, condition })
    return (
        <>
            Has{" "}
            <ConditionTypeSelector
                value={condition()}
                onChange={setCondition}
                disabled={props.disabled}
            />{" "}
            <ConditionValueInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition().value <= 0}
                disabled={props.disabled}
            />{" "}
            planet(s) within max dyson sphere radius,{" "}
            <Select
                class={styles.selectGasType}
                value={props.value.includeGiant}
                onChange={(includeGiant) =>
                    props.onChange({ ...props.value, includeGiant })
                }
                options={[false, true]}
                getLabel={(includeGiant) =>
                    includeGiant ? "including" : "excluding"
                }
                disabled={props.disabled}
            />{" "}
            gas/ice giants.
        </>
    )
}

const themeIds = [
    16, 14, 19, 11, 7, 10, 12, 17, 24, 9, 1, 20, 23, 25, 15, 18, 22, 6, 13, 8,
]

const EditThemeId: Component<{
    value: Rule.ThemeId
    onChange: (value: Rule.ThemeId) => void
    disabled?: boolean
}> = (props) => {
    return (
        <>
            Has a{" "}
            <Select
                class={styles.selectPlanetType}
                value={props.value.themeIds[0]!}
                onChange={(themeId) =>
                    props.onChange({ ...props.value, themeIds: [themeId] })
                }
                options={themeIds}
                getLabel={(themeId) => planetTypes[themeId]!}
                disabled={props.disabled}
            />{" "}
            planet.
        </>
    )
}

function isType<T extends SimpleRule, K extends RuleType>(
    rule: T,
    type: K,
): T extends { type: K } ? T | false : never {
    return rule.type === type ? (rule as any) : false
}

const EditSimpleRule: Component<{
    value: SimpleRule
    onChange: (value: SimpleRule) => void
    disabled?: boolean
}> = (props) => (
    <div class={styles.editRow}>
        <Switch>
            <Match when={isType(props.value, RuleType.Luminosity)}>
                {(value) => <EditLuminosity {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.DysonRadius)}>
                {(value) => <EditDysonRadius {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.AverageVeinAmount)}>
                {(value) => (
                    <EditAverageVeinAmount {...props} value={value()} />
                )}
            </Match>
            <Match when={isType(props.value, RuleType.Spectr)}>
                {(value) => <EditSpectr {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.TidalLockCount)}>
                {(value) => <EditTidalLockCount {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.OceanType)}>
                {(value) => <EditOceanType {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.StarType)}>
                {(value) => <EditStarType {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.GasCount)}>
                {(value) => <EditGasCount {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.SatelliteCount)}>
                {(value) => <EditSatelliteCount {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.DysonRadius)}>
                {(value) => <EditDysonRadius {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.PlanetCount)}>
                {(value) => <EditPlanetCount {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.BirthDistance)}>
                {(value) => <EditBirthDistance {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.XDistance)}>
                {(value) => <EditXDistance {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.SpectrDistance)}>
                {(value) => <EditSpectrDistance {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.GasRate)}>
                {(value) => <EditGasRate {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.PlanetInDysonCount)}>
                {(value) => (
                    <EditPlanetInDysonCount {...props} value={value()} />
                )}
            </Match>
            <Match when={isType(props.value, RuleType.ThemeId)}>
                {(value) => <EditThemeId {...props} value={value()} />}
            </Match>
            <Match when={isType(props.value, RuleType.Birth)}>
                <div class={styles.birth}>Is the starting system</div>
            </Match>
        </Switch>
    </div>
)

const DeleteButton: Component<{ onDelete: () => void }> = (props) => {
    return (
        <div class={styles.delete} onClick={() => props.onDelete()}>
            <IoTrash />
        </div>
    )
}

const EmptyRow: Component<{
    onChange: (rule: SimpleRule) => void
    onDelete?: () => void
}> = (props) => {
    return (
        <div class={styles.row}>
            <SelectSimpleRule onChange={props.onChange} />
            <Show when={!!props.onDelete}>
                <DeleteButton onDelete={() => props.onDelete?.()} />
            </Show>
        </div>
    )
}

const RuleBlockContent: Component<{
    value: SimpleRule[]
    onChange: (value: SimpleRule[]) => void
    disabled?: boolean
    onDelete?: () => void
}> = (props) => {
    function onChange(rule: SimpleRule, index: number) {
        props.onChange(props.value.map((v, i) => (i === index ? rule : v)))
    }
    function onDelete(index: number) {
        props.onChange(props.value.filter((_, i) => i !== index))
    }
    function onAdd() {
        props.onChange([...props.value, { type: RuleType.None }])
    }
    return (
        <Show
            when={props.value.length > 0}
            fallback={
                <EmptyRow
                    onChange={(rule) => props.onChange([rule])}
                    onDelete={props.onDelete}
                />
            }
        >
            <Index each={props.value}>
                {(item, index) => (
                    <>
                        <Show when={index > 0}>
                            <div class={styles.or}>OR</div>
                        </Show>
                        <div class={styles.row}>
                            <SelectSimpleRule
                                value={item()}
                                onChange={(rule) => onChange(rule, index)}
                                disabled={props.disabled}
                            />
                            <EditSimpleRule
                                value={item()}
                                onChange={(rule) => onChange(rule, index)}
                                disabled={props.disabled}
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
                    Add OR rule
                </Button>
            </Show>
        </Show>
    )
}

const RuleEditor: Component<{
    class?: string
    value: SimpleRule[][]
    onChange: (value: SimpleRule[][]) => void
    disabled?: boolean
}> = (props) => {
    function onDelete(index: number) {
        props.onChange(props.value.filter((_, i) => i !== index))
    }

    function onBlockChange(group: SimpleRule[], index: number) {
        if (group.length > 0) {
            props.onChange(props.value.map((v, i) => (i === index ? group : v)))
        } else {
            onDelete(index)
        }
    }

    function onAdd() {
        props.onChange([...props.value, []])
    }

    return (
        <div class={clsx(styles.ruleBuilder, props.class)}>
            <Show
                when={props.value.length > 0}
                fallback={
                    <div class={styles.block}>
                        <EmptyRow
                            onChange={(rule) => props.onChange([[rule]])}
                        />
                    </div>
                }
            >
                <Index each={props.value}>
                    {(group, index) => (
                        <>
                            <Show when={index > 0}>
                                <div class={styles.and}>AND</div>
                            </Show>
                            <div class={styles.block}>
                                <RuleBlockContent
                                    value={group()}
                                    onChange={(group) =>
                                        onBlockChange(group, index)
                                    }
                                    disabled={props.disabled}
                                    onDelete={
                                        props.value.length > 1
                                            ? () => onDelete(index)
                                            : undefined
                                    }
                                />
                            </div>
                        </>
                    )}
                </Index>

                <Show when={!props.disabled}>
                    <Button
                        class={styles.addAnd}
                        kind="outline"
                        onClick={onAdd}
                    >
                        Add AND rule
                    </Button>
                </Show>
            </Show>
        </div>
    )
}

export default RuleEditor

const ruleNames: Record<RuleType, string> = {
    [RuleType.None]: "Select...",
    [RuleType.And]: "",
    [RuleType.Or]: "",
    [RuleType.Birth]: "Starting System",
    [RuleType.StarType]: "Type of star",
    [RuleType.BirthDistance]: "Distance from Start",
    [RuleType.XDistance]: "Distance from X Star",
    [RuleType.SpectrDistance]: "Distance from Other Stars",
    [RuleType.Luminosity]: "Luminosity",
    [RuleType.Spectr]: "Spectral Class",
    [RuleType.DysonRadius]: "Max Dyson Sphere Radius",
    [RuleType.PlanetCount]: "Planet Count",
    [RuleType.SatelliteCount]: "Satellite Count",
    [RuleType.TidalLockCount]: "Tidally Locked Planet Count",
    [RuleType.ThemeId]: "Planet Themes",
    [RuleType.GasCount]: "Gas/Ice Giant Count",
    [RuleType.OceanType]: "Ocean",
    [RuleType.GasRate]: "Gas Rate",
    [RuleType.AverageVeinAmount]: "Vein Amount",
    [RuleType.PlanetInDysonCount]: "Planets in Dyson Sphere",
}

const rules: SimpleRule[] = [
    {
        type: RuleType.BirthDistance,
        condition: {
            type: ConditionType.Lte,
            value: 0,
        },
    },
    {
        type: RuleType.SpectrDistance,
        spectr: SpectrType.O,
        countCondition: {
            type: ConditionType.Gte,
            value: 1,
        },
        distanceCondition: {
            type: ConditionType.Lte,
            value: 0,
        },
    },
    {
        type: RuleType.XDistance,
        condition: {
            type: ConditionType.Lte,
            value: 0,
        },
    },
    {
        type: RuleType.GasCount,
        ice: null,
        condition: {
            type: ConditionType.Gte,
            value: 1,
        },
    },
    {
        type: RuleType.GasRate,
        gasType: GasType.Hydrogen,
        condition: {
            type: ConditionType.Gte,
            value: 0,
        },
    },
    {
        type: RuleType.Luminosity,
        condition: {
            type: ConditionType.Gte,
            value: 2,
        },
    },
    {
        type: RuleType.DysonRadius,
        condition: {
            type: ConditionType.Gte,
            value: 0,
        },
    },
    {
        type: RuleType.OceanType,
        oceanType: OceanType.Water,
    },
    {
        type: RuleType.PlanetCount,
        condition: {
            type: ConditionType.Gte,
            value: 2,
        },
        excludeGiant: false,
    },
    {
        type: RuleType.ThemeId,
        themeIds: [1],
    },
    {
        type: RuleType.PlanetInDysonCount,
        includeGiant: false,
        condition: {
            type: ConditionType.Gte,
            value: 1,
        },
    },
    {
        type: RuleType.SatelliteCount,
        condition: {
            type: ConditionType.Gte,
            value: 1,
        },
    },
    {
        type: RuleType.Spectr,
        spectr: [SpectrType.O],
    },
    {
        type: RuleType.Birth,
    },
    {
        type: RuleType.TidalLockCount,
        condition: {
            type: ConditionType.Gte,
            value: 1,
        },
    },
    {
        type: RuleType.StarType,
        starType: [StarType.MainSeqStar],
    },
    {
        type: RuleType.AverageVeinAmount,
        vein: VeinType.Iron,
        condition: {
            type: ConditionType.Gte,
            value: 0,
        },
    },
]

const veins: VeinType[] = [
    VeinType.Iron,
    VeinType.Copper,
    VeinType.Silicium,
    VeinType.Titanium,
    VeinType.Stone,
    VeinType.Coal,
    VeinType.Oil,
    VeinType.Fireice,
    VeinType.Diamond,
    VeinType.Fractal,
    VeinType.Crysrub,
    VeinType.Grat,
    VeinType.Bamboo,
    VeinType.Mag,
]

const spectrs: SpectrType[] = [
    SpectrType.O,
    SpectrType.B,
    SpectrType.A,
    SpectrType.F,
    SpectrType.G,
    SpectrType.K,
    SpectrType.M,
    SpectrType.X,
]

const oceans: OceanType[] = [OceanType.Water, OceanType.Sulfur]

const starTypes: StarType[] = [
    StarType.MainSeqStar,
    StarType.GiantStar,
    StarType.WhiteDwarf,
    StarType.BlackHole,
    StarType.NeutronStar,
]

const starTypeNames: Record<StarType, string> = {
    [StarType.MainSeqStar]: "Normal Star",
    [StarType.GiantStar]: "Giant Star",
    [StarType.WhiteDwarf]: "White Dwarf",
    [StarType.BlackHole]: "Black Hole",
    [StarType.NeutronStar]: "Neutron Star",
}

const gasTypes: GasType[] = [
    GasType.Hydrogen,
    GasType.Deuterium,
    GasType.Fireice,
]

const gasTypeNames: Record<GasType, string> = {
    [GasType.None]: "",
    [GasType.Hydrogen]: "Hydrogen",
    [GasType.Deuterium]: "Deuterium",
    [GasType.Fireice]: "Fire Ice",
}
