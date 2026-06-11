function toDouble(num) {
    if (Number.isInteger(num)) return num + ".0"
    return num
}

const themeDistribute = ["Default", "Birth", "Interstellar", "Rare"]

const planetType = ["None", "Volcano", "Ocean", "Desert", "Ice", "Gas"]

const veinType = [
    "None",
    "Iron",
    "Copper",
    "Silicium",
    "Titanium",
    "Stone",
    "Coal",
    "Oil",
    "Fireice",
    "Diamond",
    "Fractal",
    "Crysrub",
    "Grat",
    "Bamboo",
    "Mag",
    "Max",
]

function run() {
    const array = themeProtoSet.m_Structure.dataArray.map(
        (x) => `        ThemeProto {
            id: ${x.ID},
            name: "${x.Name}",
            water_item_id: ${x.WaterItemId},
            wind: ${toDouble(x.Wind)},
            distribute: ThemeDistribute::${themeDistribute[x.Distribute]},
            temperature: ${toDouble(x.Temperature)},
            planet_type: PlanetType::${planetType[x.PlanetType]},
            vein_spot: vec![${x.VeinSpot.join(", ")}],
            vein_count: vec![${x.VeinCount.map(toDouble).join(", ")}],
            vein_opacity: vec![${x.VeinOpacity.map(toDouble).join(", ")}],
            rare_veins: vec![${x.RareVeins.map((y) => "VeinType::" + veinType[y])}],
            rare_settings: vec![${x.RareSettings.map(toDouble).join(", ")}],
            gas_items: vec![${x.GasItems.join(", ")}],
            gas_speeds: vec![${x.GasSpeeds.join(", ")}],
            algos: vec![${x.Algos.join(", ")}],
        },`,
    )
    // eslint-disable-next-line no-undef
    console.log(array.join("\n"))
}

/* eslint-disable */
const themeProtoSet = {/** Copy ThemeProtoSet.json here */}

run()