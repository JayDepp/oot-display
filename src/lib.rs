use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct SaveData {
    pub entrance_index: u32,
    pub age_modifier: AgeModifier,
    _00: [u8; 4],
    pub time_of_day: u16,
    _01: [u8; 2],
    pub day_night_modifier: DayNightModifier,
    _02: [u8; 14],
    pub death_counter: u16,
    pub name: [u8; 10], // TODO: What is the format?
    pub heart_containers: Hearts,
    pub health: Hearts,
    _03: [u8; 1],
    pub magic: u8,
    pub rupees: u16,
    _04: Ignore50,
    pub equipped_items_id: EquippedItemsID,
    pub equipped_items_slot: EquippedItemsSlot,
    _06: [u8; 2],
    pub equipped_tunic_and_boots: EquippedTunicAndBoots,
    pub equipped_sword_and_sheild: EquippedSwordAndShield,
    _05: [u8; 2],
    pub item_slot_item_ids: ItemSlotItemIds,
    pub item_slot_amount: ItemSlotAmount,
    pub magic_beans_purchased: u8,
    pub sword_and_shield: SwordAndShield,
    pub tunic_and_boots: TunicAndBoots,
    // TODO: ...
}

// TODO: Should these be swapped?
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum AgeModifier {
    Adult = 0x0,
    Child = 0x1,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum DayNightModifier {
    Day = 0x0,
    Night = 0x1,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Hearts(u16);

// TODO: Check if backwards
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct EquippedItemsID {
    pub c_right: ItemID,
    pub c_down: ItemID,
    pub c_left: ItemID,
    pub b: ItemID,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ItemID {
    // Inventory Items
    DekuStick = 0x00,
    DekuNut = 0x01,
    Bomb = 0x02,
    FairyBow = 0x03,
    FireArrow = 0x04,
    DinsFire = 0x05,
    FairySlingshot = 0x06,
    FairyOcarina = 0x07,
    OcarinaOfTime = 0x08,
    Bombchu10 = 0x09,
    Hookshot = 0x0A,
    Longshot = 0x0B,
    IceArrow = 0x0C,
    FaroresWind = 0x0D,
    Boomerang = 0x0E,
    LensOfTruth = 0x0F,
    MagicBean = 0x10,
    MegatonHammer = 0x11,
    LightArrow = 0x12,
    NayrusLove = 0x13,
    EmptyBottle = 0x14,
    RedPotion = 0x15,
    GreenPotion = 0x16,
    BluePotion = 0x17,
    BottledFairy = 0x18,
    Fish = 0x19,
    LonLonMilkAndBottle = 0x1A,
    RutosLetter = 0x1B,
    BlueFire = 0x1C,
    Bug = 0x1D,
    BigPoe = 0x1E,
    LonLonMilkHalf = 0x1F,
    Poe = 0x20,
    WeirdEgg = 0x21,
    Chicken = 0x22,
    ZeldasLetter = 0x23,
    KeatonMask = 0x24,
    SkullMask = 0x25,
    SpookyMask = 0x26,
    BunnyHood = 0x27,
    GoronMask = 0x28,
    ZoraMask = 0x29,
    GerudoMask = 0x2A,
    MaskOfTruth = 0x2B,
    SOLDOUT = 0x2C,
    PocketEgg = 0x2D,
    PocketCucco = 0x2E,
    Cojiro = 0x2F,
    OddMushroom = 0x30,
    OddPotion = 0x31,
    PoachersSaw = 0x32,
    GoronsSwordBroken = 0x33,
    Prescription = 0x34,
    EyeballFrog = 0x35,
    EyeDrops = 0x36,
    ClaimCheck = 0x37,
    FairyBowWithFireArrow = 0x38,
    FairyBowWithIceArrow = 0x39,
    FairyBowWithLightArrow = 0x3A,
    // Equipment
    KokiriSword = 0x3B,
    MasterSword = 0x3C,
    GiantsKnifeOrBiggoronsSword = 0x3D,
    DekuShield = 0x3E,
    HylianShield = 0x3F,
    MirrorShield = 0x40,
    KokiriTunic = 0x41,
    GoronTunic = 0x42,
    ZoraTunic = 0x43,
    KokiriBoots = 0x44,
    IronBoots = 0x45,
    HoverBoots = 0x46,
    BulletBag30 = 0x47,
    BulletBag40 = 0x48,
    BulletBag50 = 0x49,
    Quiver30 = 0x4A,
    BigQuiver40 = 0x4B,
    BiggestQuiver50 = 0x4C,
    BombBag20 = 0x4D,
    BigBombBag30 = 0x4E,
    BiggestBombBag40 = 0x4F,
    GoronsBracelet = 0x50,
    SilverGauntlets = 0x51,
    GoldenGauntlets = 0x52,
    SilverScale = 0x53,
    GoldenScale = 0x54,
    GiantsKnifeBroken = 0x55,
    AdultsWallet = 0x56,
    GiantsWallet = 0x57,
    DekuSeeds5 = 0x58,
    FishingPole = 0x59,
    // QuestItems
    MinuetOfForest = 0x5A,
    BoleroOfFire = 0x5B,
    SerenadeOfWater = 0x5C,
    RequiemOfSpirit = 0x5D,
    NocturneOfShadow = 0x5E,
    PreludeOfLight = 0x5F,
    ZeldasLullaby = 0x60,
    EponasSong = 0x61,
    SariasSong = 0x62,
    SunsSong = 0x63,
    SongOfTime = 0x64,
    SongOfStorms = 0x65,
    ForestMedallion = 0x66,
    FireMedallion = 0x67,
    WaterMedallion = 0x68,
    SpiritMedallion = 0x69,
    ShadowMedallion = 0x6A,
    LightMedallion = 0x6B,
    KokirisEmerald = 0x6C,
    GoronsRuby = 0x6D,
    ZorasSapphire = 0x6E,
    StoneOfAgony = 0x6F,
    GerudosCard = 0x70,
    GoldSkulltulaToken = 0x71,
    HeartContainer = 0x72,
    PieceOfHeart = 0x73,
    // Dungeon Items
    BigKey = 0x74,
    Compass = 0x75,
    DungeonMap = 0x76,
    // Collectibles
    SmallKey = 0x77,
    SmallMagicJar = 0x78,
    LargeMagicJar = 0x79,
    PieceOFHeart = 0x7A,
    // Removed Items
    Removed0 = 0x7B,
    Removed1 = 0x7C,
    Removed2 = 0x7D,
    Removed3 = 0x7E,
    Removed4 = 0x7F,
    Removed5 = 0x80,
    Removed6 = 0x81,
    // Other Items
    LonLonMilk = 0x82,
    RecoveryHeart = 0x83,
    GreenRupee = 0x84,
    BlueRupee = 0x85,
    RedRupee = 0x86,
    PurpleRupee = 0x87,
    HugeRupee = 0x88,
    Removed7 = 0x89,
    DekuSticks5 = 0x8A,
    DekuSticks10 = 0x8B,
    DekuNuts5 = 0x8C,
    DekuNuts10 = 0x8D,
    Bombs5 = 0x8E,
    Bombs10 = 0x8F,
    Bombs20 = 0x90,
    Bombs30 = 0x91,
    Arrows5Or10 = 0x92,
    Arrows10Or30 = 0x93,
    Arrows30Or50 = 0x94,
    DekuSeeds30 = 0x95,
    Bombchu5 = 0x96,
    Bombchu20 = 0x97,
    DekuStickUpgrade20 = 0x98,
    DekuStickUpgrade30 = 0x99,
    DekuNutUpgrade30 = 0x9A,
    DekuNutUpgrade40 = 0x9B,
    NoItem = 0xFF,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Ignore50([u8; 48]);

use std::fmt;
impl fmt::Debug for Ignore50 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0.iter() {
            write!(f, "{} ", x)?;
        }
        Ok(())
    }
}

// TODO: Check if backwards
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct EquippedItemsSlot {
    _unused: u8,
    pub c_right: ItemID,
    pub c_down: ItemID,
    pub c_left: ItemID,
}

bitflags! {
    pub struct EquippedTunicAndBoots: u8 {
        const KokiriTunic = 0x01;
        const GoronTunic = 0x02;
        const ZoraTunic = 0x03;
        const KokiriBoots = 0x10;
        const IronBoots = 0x20;
        const HoverBoots = 0x30;
    }
}

bitflags! {
    pub struct EquippedSwordAndShield: u8 {
        const KokiriSword = 0x01;
        const MasterSword = 0x02;
        const BiggoronsSword = 0x03;
        const KokiriShield = 0x10;
        const HylianShield = 0x20;
        const MirrorShield = 0x30;
    }
}

// TODO: This ordering seems wrong.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ItemSlotItemIds {
    pub deku_stick: ItemID,
    pub deku_nut: ItemID,
    pub bomb: ItemID,
    pub fairy_bow: ItemID,
    pub fire_arrow: ItemID,
    pub dins_fire: ItemID,
    pub slingshot: ItemID,
    pub ocarina: ItemID,
    pub bombchu: ItemID,
    pub hookshot_longshot: ItemID,
    pub ice_arrow: ItemID,
    pub farores_wind: ItemID,
    pub boomerang: ItemID,
    pub lens_of_truth: ItemID,
    pub magical_beans: ItemID,
    pub megaton_hammer: ItemID,
    pub light_arrow: ItemID,
    pub nayrus_love: ItemID,
    pub bottle_1: ItemID,
    pub bottle_2: ItemID,
    pub bottle_3: ItemID,
    pub bottle_4: ItemID,
    pub adult_trade_item: ItemID,
    pub child_trade_item: ItemID,
}

// TODO: This ordering seems wrong.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ItemSlotAmount {
    pub deku_stick: u8,
    pub deku_nut: u8,
    pub bomb: u8,
    pub fairy_bow: u8,
    pub fire_arrow: u8,
    pub dins_fire: u8,
    pub slingshot: u8,
    pub ocarina: u8,
    pub bombchu: u8,
    pub hookshot_longshot: u8,
    pub ice_arrow: u8,
    pub farores_wind: u8,
    pub boomerang: u8,
    pub lens_of_truth: u8,
    pub magical_beans: u8,
}

bitflags! {
    pub struct TunicAndBoots: u8 {
        const KokiriTunic = 0x01;
        const GoronTunic = 0x02;
        const ZoraTunic = 0x04;
        const KokiriBoots = 0x10;
        const IronBoots = 0x20;
        const HoverBoots = 0x40;
    }
}

bitflags! {
    pub struct SwordAndShield: u8 {
        const KokiriSword = 0x01;
        const MasterSword = 0x02;
        const BiggoronsSword = 0x04;
        const BrokenGiantsKnife = 0x08;
        const KokiriShield = 0x10;
        const HylianShield = 0x20;
        const MirrorShield = 0x40;
    }
}
