use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BLEData {
    pub device_id: String,
    pub time_stamp: i64,
    // pub adv_flags: Option<AdvFlag>,
    // pub payload: Vec<u8>,
    // pub service_uuids: Vec<BleUuid>,
    pub name: Option<String>,
    pub tx_power: Option<u8>,
    // pub service_uuid: Option<BleUuid>,
    // pub service_data: Option<Vec<u8>>,
    pub company_identifier: Option<u16>,
    pub manufacture_payload: Option<Vec<u8>>,
    // pub adv_type: AdvType,
    pub rssi: i8,
    // sid: u8
    // prim_phy: PrimPhy,
    // sec_phy: Option<SecPhy>,
    // periodic_itvl: u16
    pub addr: [u8; 6],
    // pub addr_type: BLEAddressType,
}

/// A Bluetooth UUID.
#[derive(Copy, Clone, Debug)]
pub enum BleUuid {
    /// A 16-bit UUID.
    Uuid16(u16),
    /// A 32-bit UUID.
    Uuid32(u32),
    /// A 128-bit UUID.
    Uuid128([u8; 16]),
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct AdvFlag: u8 {
    /// LE Limited Discoverable Mode
    const DiscLimited = 1 as _;
    /// LE General Discoverable Mode
    const DiscGeneral = 2 as _;
    /// BR/EDR Not Supported
    const BrEdrUnsupported = 4 as _;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Controller)
    const SimultaneousController = 0b01000;
    /// Simultaneous LE and BR/EDR to Same Device Capable (Host)
    const SimultaneousHost       = 0b10000;
  }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum PrimPhy {
    /// 1Mbps phy
    Phy1M = 1 as _,
    /// Coded phy
    Coded = 3 as _,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AdvType {
    /// indirect advertising
    Ind,
    /// direct advertising
    DirectInd,
    /// indirect scan response
    ScanInd,
    /// indirect advertising - not connectable
    NonconnInd,
    ScanResponse,
    // #[cfg(esp_idf_bt_nimble_ext_adv)]
    // Extended(u8),
}

/// Bluetooth Device address type
#[derive(PartialEq, Eq, TryFromPrimitive, Debug, Clone)]
#[repr(u8)]
pub enum BLEAddressType {
    Public = 0 as _,
    Random = 1 as _,
    PublicID = 2 as _,
    RandomID = 3 as _,
}
