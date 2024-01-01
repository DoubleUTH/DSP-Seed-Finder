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
import { veinNames } from "../util"

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

const EditLuminosity: Component<{
    value: Rule.Luminosity
    onChange: (value: Rule.Luminosity) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Is at least{" "}
            <NumberInput
                class={styles.inputLuminosity}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0 || condition() >= 3}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Is at least{" "}
            <NumberInput
                class={styles.inputDyson}
                maxLength={6}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
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
            and the estimated amount is at least{" "}
            <NumberInput
                class={styles.inputVein}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Has at least{" "}
            <NumberInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Has at least{" "}
            <NumberInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Has at least{" "}
            <NumberInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Has at least{" "}
            <NumberInput
                class={styles.inputCount}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 1}
                disabled={props.disabled}
            />{" "}
            planets
        </>
    )
}

const EditBirthDistance: Component<{
    value: Rule.BirthDistance
    onChange: (value: Rule.BirthDistance) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Is at most{" "}
            <NumberInput
                class={styles.inputDistance}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
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
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
    return (
        <>
            Is at most{" "}
            <NumberInput
                class={styles.inputDistance}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
                disabled={props.disabled}
            />
            ly away from a black hole / neutron star.
        </>
    )
}

const EditGasRate: Component<{
    value: Rule.GasRate
    onChange: (value: Rule.GasRate) => void
    disabled?: boolean
}> = (props) => {
    const condition = () => props.value.condition.value
    const setCondition = (value: number) => {
        props.onChange({
            ...props.value,
            condition: { ...props.value.condition, value },
        })
    }
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
            and at least{" "}
            <NumberInput
                class={styles.inputGasRate}
                value={condition()}
                onChange={setCondition}
                emptyValue={-1}
                error={condition() <= 0}
                disabled={props.disabled}
            />
            /s of it
        </>
    )
}

const EditSimpleRule: Component<{
    value: SimpleRule
    onChange: (value: SimpleRule) => void
    disabled?: boolean
}> = (props) => (
    <div class={styles.editRow}>
        {props.value.type === RuleType.Luminosity ? (
            EditLuminosity({ ...props, value: props.value })
        ) : props.value.type === RuleType.DysonRadius ? (
            EditDysonRadius({ ...props, value: props.value })
        ) : props.value.type === RuleType.AverageVeinAmount ? (
            EditAverageVeinAmount({ ...props, value: props.value })
        ) : props.value.type === RuleType.Spectr ? (
            EditSpectr({ ...props, value: props.value })
        ) : props.value.type === RuleType.TidalLockCount ? (
            EditTidalLockCount({ ...props, value: props.value })
        ) : props.value.type === RuleType.OceanType ? (
            EditOceanType({ ...props, value: props.value })
        ) : props.value.type === RuleType.StarType ? (
            EditStarType({ ...props, value: props.value })
        ) : props.value.type === RuleType.GasCount ? (
            EditGasCount({ ...props, value: props.value })
        ) : props.value.type === RuleType.SatelliteCount ? (
            EditSatelliteCount({ ...props, value: props.value })
        ) : props.value.type === RuleType.PlanetCount ? (
            EditPlanetCount({ ...props, value: props.value })
        ) : props.value.type === RuleType.BirthDistance ? (
            EditBirthDistance({ ...props, value: props.value })
        ) : props.value.type === RuleType.XDistance ? (
            EditXDistance({ ...props, value: props.value })
        ) : props.value.type === RuleType.GasRate ? (
            EditGasRate({ ...props, value: props.value })
        ) : props.value.type === RuleType.Birth ? (
            <div class={styles.birth}>Is the starting system</div>
        ) : (
            <></>
        )}
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
                <Button class={styles.orRule} kind="outline" onClick={onAdd}>
                    Add OR rule
                </Button>
            </Show>
        </Show>
    )
}

const RuleEditor: Component<{
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
        <div class={styles.ruleBuilder}>
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
    [RuleType.Luminosity]: "Luminosity",
    [RuleType.Spectr]: "Spectral Class",
    [RuleType.DysonRadius]: "Max Dyson Sphere Radius",
    [RuleType.PlanetCount]: "Planet Count",
    [RuleType.SatelliteCount]: "Satellite Count",
    [RuleType.TidalLockCount]: "Tidally Locked Planet Count",
    [RuleType.ThemeId]: "",
    [RuleType.GasCount]: "Gas/Ice Giant Count",
    [RuleType.OceanType]: "Ocean",
    [RuleType.GasRate]: "Gas Rate",
    [RuleType.AverageVeinAmount]: "Vein Amount",
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
    [StarType.GiantStar]: "Red/Blue Giant",
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
