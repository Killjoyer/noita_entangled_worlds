use bitcode::{Decode, Encode};

pub mod message_socket;

pub mod basic_types;
pub mod des;

pub use basic_types::*;

#[derive(Encode, Decode)]
pub struct ProxyKV {
    pub key: String,
    pub value: String,
}

#[derive(Encode, Decode)]
pub struct ProxyKVBin {
    pub key: u8,
    pub value: Vec<u8>,
}

#[derive(Encode, Decode)]
pub struct ModMessage {
    pub peer: PeerId,
    pub value: Vec<u8>,
}

#[derive(Encode, Decode, Clone)]
pub enum RemoteMessage {
    RemoteDes(des::RemoteDes),
}

#[derive(Encode, Decode)]
pub enum NoitaInbound {
    RawMessage(Vec<u8>),
    Ready {
        my_peer_id: PeerId,
    },
    ProxyToDes(des::ProxyToDes),
    RemoteMessage {
        source: PeerId,
        message: RemoteMessage,
    },
}

#[derive(Encode, Decode)]
pub enum NoitaOutbound {
    Raw(Vec<u8>),
    DesToProxy(des::DesToProxy),
    RemoteMessage {
        reliable: bool,
        destination: Destination<PeerId>,
        message: RemoteMessage,
    },
}
use strum::{EnumString, IntoStaticStr};

#[derive(EnumString, IntoStaticStr, Clone, Copy, Encode, Decode, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GameEffectEnum {
    None,
    Electrocution,
    Frozen,
    OnFire,
    Poison,
    Berserk,
    Charm,
    Polymorph,
    PolymorphRandom,
    Blindness,
    Telepathy,
    Teleportation,
    Regeneration,
    Levitation,
    MovementSlower,
    Farts,
    Drunk,
    BreathUnderwater,
    Radioactive,
    Wet,
    Oiled,
    Bloody,
    Slimy,
    CriticalHitBoost,
    Confusion,
    MeleeCounter,
    WormAttractor,
    WormDetractor,
    FoodPoisoning,
    FriendThundermage,
    FriendFiremage,
    InternalFire,
    InternalIce,
    Jarate,
    Knockback,
    KnockbackImmunity,
    MovementSlower2X,
    MovementFaster,
    StainsDropFaster,
    SavingGrace,
    DamageMultiplier,
    HealingBlood,
    Respawn,
    ProtectionFire,
    ProtectionRadioactivity,
    ProtectionExplosion,
    ProtectionMelee,
    ProtectionElectricity,
    Teleportitis,
    StainlessArmour,
    GlobalGore,
    EditWandsEverywhere,
    ExplodingCorpseShots,
    ExplodingCorpse,
    ExtraMoney,
    ExtraMoneyTrickKill,
    HoverBoost,
    ProjectileHoming,
    AbilityActionsMaterialized,
    NoDamageFlash,
    NoSlimeSlowdown,
    MovementFaster2X,
    NoWandEditing,
    LowHpDamageBoost,
    FasterLevitation,
    StunProtectionElectricity,
    StunProtectionFreeze,
    IronStomach,
    ProtectionAll,
    Invisibility,
    RemoveFogOfWar,
    ManaRegeneration,
    ProtectionDuringTeleport,
    ProtectionPolymorph,
    ProtectionFreeze,
    FrozenSpeedUp,
    UnstableTeleportation,
    PolymorphUnstable,
    Custom,
    AllergyRadioactive,
    RainbowFarts,
    Weakness,
    ProtectionFoodPoisoning,
    NoHeal,
    ProtectionEdges,
    ProtectionProjectile,
    PolymorphCessation,
    _Last,
}

#[derive(Encode, Decode, Clone)]
pub enum GameEffectData {
    Normal(GameEffectEnum),
    Custom(String),
    Projectile(Vec<u8>),
}
