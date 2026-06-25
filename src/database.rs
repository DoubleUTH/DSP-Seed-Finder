use rusqlite::{params, Connection, OptionalExtension, Result, Statement, Transaction};

use crate::data::{enums::VeinType, galaxy::Galaxy, game_desc::GameDesc};

const INFO_ID: i32 = 0;
const fn lookup_ocean_type(ocean_type: i32) -> u8 {
    match ocean_type {
        -2 => 0b0001,   // Ice
        -1 => 0b0010,   // Lava
        1000 => 0b0100, // Water
        1116 => 0b1000, // Sulfur
        _ => 0b0000,    // None
    }
}

pub fn create_database(name: String) -> Result<Connection> {
    let conn = Connection::open(name)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS info (
        id INTEGER PRIMARY KEY,
        start INTEGER NOT NULL,
        end INTEGER NOT NULL,
        star_count INTEGER NOT NULL,
        resource_multiplier REAL NOT NULL,
        hive_initial_colonize REAL NOT NULL,
        hive_max_density REAL NOT NULL,
        use_actual_veins INTEGER NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stars (
        seed INTEGER NOT NULL,
        star_id INTEGER NOT NULL,
        pos_x INTEGER NOT NULL,
        pos_y INTEGER NOT NULL,
        pos_z INTEGER NOT NULL,
        dyson_radis INTEGER NOT NULL,
        gas_giant_count INTEGER NOT NULL,
        ice_giant_count INTEGER NOT NULL,
        initial_hive_count INTEGER NOT NULL,
        max_hive_count INTEGER NOT NULL,
        luminosity REAL NOT NULL,
        ocean_type INTEGER NOT NULL,
        planet_count INTEGER NOT NULL,
        planet_in_dyson_count INTEGER NOT NULL,
        satellite_count INTEGER NOT NULL,
        spectral_class INTEGER NOT NULL,
        themes INTEGER NOT NULL,
        tidal_lock_count INTEGER NOT NULL,
        hydrogen_rate REAL NOT NULL,
        fire_ice_rate REAL NOT NULL,
        deuterium_rate REAL NOT NULL,
        iron_ore INTEGER NOT NULL,
        copper_ore INTEGER NOT NULL,
        silicon_ore INTEGER NOT NULL,
        titanium_ore INTEGER NOT NULL,
        stone INTEGER NOT NULL,
        coal INTEGER NOT NULL,
        crude_oil INTEGER NOT NULL,
        fire_ice INTEGER NOT NULL,
        kimberlite_ore INTEGER NOT NULL,
        fractal_silicon INTEGER NOT NULL,
        organic_crystal INTEGER NOT NULL,
        grating_crystal INTEGER NOT NULL,
        stalagmite_crystal INTEGER NOT NULL,
        unipolar_magnet INTEGER NOT NULL,
        PRIMARY KEY (seed, star_id)
        )",
        (),
    )?;
    Ok(conn)
}

pub fn set_info(conn: &mut Connection, range: &(i32, i32), game: &GameDesc) -> Result<bool> {
    let tx = conn.transaction()?;
    let existing = tx
        .query_row(
            "SELECT start, end, star_count, resource_multiplier,
                    hive_initial_colonize, hive_max_density, use_actual_veins
                FROM info WHERE id = ?1",
            [INFO_ID],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )
        .optional()?;
    if let Some(row) = existing {
        let (
            start,
            end,
            star_count,
            resource_multiplier,
            hive_initial_colonize,
            hive_max_density,
            use_actual_veins,
        ): (i32, i32, u32, f32, f64, f64, i8) = row;
        if start != range.0
            || end != range.1
            || game.star_count as u32 != star_count
            || game.resource_multiplier != resource_multiplier
            || game.hive_initial_colonize != hive_initial_colonize
            || game.hive_max_density != hive_max_density
            || game.use_actual_veins != (use_actual_veins == 1)
        {
            return Ok(false);
        }
    } else {
        tx.execute("INSERT INTO info (id, start, end, star_count, resource_multiplier,
            hive_initial_colonize, hive_max_density, use_actual_veins) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![
                INFO_ID, range.0, range.1, game.star_count as u32, game.resource_multiplier, game.hive_initial_colonize,
                game.hive_max_density, if game.use_actual_veins { 1 } else { 0 }
            ])?;
        tx.commit()?;
    }
    Ok(true)
}

pub fn create_insert_seed_stmt<'a, 'b: 'a>(tx: &'b mut Transaction) -> Result<Statement<'a>> {
    tx.prepare("INSERT OR REPLACE INTO stars (
        seed, star_id, pos_x, pos_y, pos_z,
        dyson_radis, gas_giant_count, ice_giant_count,
        initial_hive_count, max_hive_count, luminosity, ocean_type,
        planet_count, planet_in_dyson_count, satellite_count, spectral_class,
        themes, tidal_lock_count, hydrogen_rate, fire_ice_rate, deuterium_rate,
        iron_ore, copper_ore, silicon_ore, titanium_ore, stone, coal, crude_oil,
        fire_ice, kimberlite_ore, fractal_silicon, organic_crystal, grating_crystal, stalagmite_crystal, unipolar_magnet)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
        ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35)
    ")
}

type VeinAmount = i64;

pub type SeedParams = (
    i32,
    u32,
    f64,
    f64,
    f64,
    i32,
    u8,
    u8,
    i32,
    i32,
    f32,
    u8,
    u32,
    u8,
    u8,
    i32,
    u32,
    u8,
    f32,
    f32,
    f32,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
    VeinAmount,
);

pub fn get_seed_params(galaxy: &Galaxy, use_actual_veins: bool) -> Vec<SeedParams> {
    galaxy
        .stars
        .iter()
        .map(|sp| {
            let star = &sp.star;
            let planets = sp.get_planets();
            let mut gas_giant_count: u8 = 0;
            let mut ice_giant_count: u8 = 0;
            let mut ocean_type: u8 = 0;
            let mut planet_in_dyson_count: u8 = 0;
            let mut satellite_count: u8 = 0;
            let mut themes: u32 = 0;
            let mut tidal_lock_count: u8 = 0;
            let mut hydrogen_rate: f32 = 0.0;
            let mut fire_ice_rate: f32 = 0.0;
            let mut deuterium_rate: f32 = 0.0;
            let mut iron_ore: VeinAmount = 0;
            let mut copper_ore: VeinAmount = 0;
            let mut silicon_ore: VeinAmount = 0;
            let mut titanium_ore: VeinAmount = 0;
            let mut stone: VeinAmount = 0;
            let mut coal: VeinAmount = 0;
            let mut crude_oil: VeinAmount = 0;
            let mut fire_ice: VeinAmount = 0;
            let mut kimberlite_ore: VeinAmount = 0;
            let mut fractal_silicon: VeinAmount = 0;
            let mut organic_crystal: VeinAmount = 0;
            let mut grating_crystal: VeinAmount = 0;
            let mut stalagmite_crystal: VeinAmount = 0;
            let mut unipolar_magnet: VeinAmount = 0;
            let dyson_radius = star.get_dyson_radius() as f32 / 40000.0;
            for planet in planets {
                let theme = planet.get_theme();
                themes |= 1 << (theme.id - 1);
                if planet.is_gas_giant() {
                    if theme.temperature < 0.0 {
                        ice_giant_count += 1;
                    } else {
                        gas_giant_count += 1;
                    }
                    for (gas_type, rate) in planet.get_gases() {
                        match *gas_type {
                            1120 => hydrogen_rate += rate,
                            1011 => fire_ice_rate += rate,
                            1121 => deuterium_rate += rate,
                            _ => {}
                        };
                    }
                } else if use_actual_veins {
                    for vein in planet.get_actual_veins() {
                        let amount = vein.amount as VeinAmount;
                        match vein.vein_type {
                            VeinType::Iron => iron_ore += amount,
                            VeinType::Copper => copper_ore += amount,
                            VeinType::Silicium => silicon_ore += amount,
                            VeinType::Titanium => titanium_ore += amount,
                            VeinType::Stone => stone += amount,
                            VeinType::Coal => coal += amount,
                            VeinType::Oil => crude_oil += amount,
                            VeinType::Fireice => fire_ice += amount,
                            VeinType::Diamond => kimberlite_ore += amount,
                            VeinType::Fractal => fractal_silicon += amount,
                            VeinType::Crysrub => organic_crystal += amount,
                            VeinType::Grat => grating_crystal += amount,
                            VeinType::Bamboo => stalagmite_crystal += amount,
                            VeinType::Mag => unipolar_magnet += amount,
                            _ => {}
                        }
                    }
                } else {
                    for vein in planet.get_estimated_veins() {
                        let amount = ((vein.min_patch + vein.max_patch) as i64
                            * (vein.min_group + vein.max_group) as i64
                            * (vein.min_amount + vein.max_amount) as i64
                            / 8) as VeinAmount;
                        match vein.vein_type {
                            VeinType::Iron => iron_ore += amount,
                            VeinType::Copper => copper_ore += amount,
                            VeinType::Silicium => silicon_ore += amount,
                            VeinType::Titanium => titanium_ore += amount,
                            VeinType::Stone => stone += amount,
                            VeinType::Coal => coal += amount,
                            VeinType::Oil => crude_oil += amount,
                            VeinType::Fireice => fire_ice += amount,
                            VeinType::Diamond => kimberlite_ore += amount,
                            VeinType::Fractal => fractal_silicon += amount,
                            VeinType::Crysrub => organic_crystal += amount,
                            VeinType::Grat => grating_crystal += amount,
                            VeinType::Bamboo => stalagmite_crystal += amount,
                            VeinType::Mag => unipolar_magnet += amount,
                            _ => {}
                        }
                    }
                }
                ocean_type |= lookup_ocean_type(theme.water_item_id);
                if planet.get_sun_distance() < dyson_radius {
                    planet_in_dyson_count += 1;
                }
                if planet.has_orbit_around() {
                    satellite_count += 1;
                }
                if planet.is_tidal_locked() {
                    tidal_lock_count += 1;
                }
            }
            (
                galaxy.seed,
                star.index as u32,
                star.position.0,
                star.position.1,
                star.position.2,
                star.get_dyson_radius(),
                gas_giant_count,
                ice_giant_count,
                star.get_initial_hive_count(),
                star.get_max_hive_count(),
                star.get_luminosity(),
                ocean_type,
                planets.len() as u32,
                planet_in_dyson_count,
                satellite_count,
                (star.get_spectr() as i32) + 4,
                themes,
                tidal_lock_count,
                hydrogen_rate,
                fire_ice_rate,
                deuterium_rate,
                iron_ore,
                copper_ore,
                silicon_ore,
                titanium_ore,
                stone,
                coal,
                crude_oil,
                fire_ice,
                kimberlite_ore,
                fractal_silicon,
                organic_crystal,
                grating_crystal,
                stalagmite_crystal,
                unipolar_magnet,
            )
        })
        .collect()
}

pub fn insert_seed(stmt: &mut Statement<'_>, p: &Vec<SeedParams>) {
    for (
        p1,
        p2,
        p3,
        p4,
        p5,
        p6,
        p7,
        p8,
        p9,
        p10,
        p11,
        p12,
        p13,
        p14,
        p15,
        p16,
        p17,
        p18,
        p19,
        p20,
        p21,
        p22,
        p23,
        p24,
        p25,
        p26,
        p27,
        p28,
        p29,
        p30,
        p31,
        p32,
        p33,
        p34,
        p35,
    ) in p
    {
        let _ = stmt
            .execute(params![
                p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18,
                p19, p20, p21, p22, p23, p24, p25, p26, p27, p28, p29, p30, p31, p32, p33, p34,
                p35,
            ])
            .unwrap();
    }
}
