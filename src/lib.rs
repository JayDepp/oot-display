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
    pub name: [u8; 10], // TODO: ?
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
    DekuStick = 0x00,
    DekuNut = 0x01,
    Bomb = 0x02,
    // ..
    KokiriSword = 0x3B,
    // TODO: Fill rest
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
