use crc::{
    Crc, CRC_10_ATM, CRC_10_CDMA2000, CRC_10_GSM, CRC_11_FLEXRAY, CRC_11_UMTS, CRC_12_CDMA2000,
    CRC_12_DECT, CRC_12_GSM, CRC_12_UMTS, CRC_13_BBC, CRC_14_DARC, CRC_14_GSM, CRC_15_CAN,
    CRC_15_MPT1327, CRC_16_ARC, CRC_16_CDMA2000, CRC_16_CMS, CRC_16_DDS_110, CRC_16_DECT_R,
    CRC_16_DECT_X, CRC_16_DNP, CRC_16_EN_13757, CRC_16_GENIBUS, CRC_16_GSM, CRC_16_IBM_3740,
    CRC_16_IBM_SDLC, CRC_16_ISO_IEC_14443_3_A, CRC_16_KERMIT, CRC_16_LJ1200, CRC_16_M17,
    CRC_16_MAXIM_DOW, CRC_16_MCRF4XX, CRC_16_MODBUS, CRC_16_NRSC_5, CRC_16_OPENSAFETY_A,
    CRC_16_OPENSAFETY_B, CRC_16_PROFIBUS, CRC_16_RIELLO, CRC_16_SPI_FUJITSU, CRC_16_T10_DIF,
    CRC_16_TELEDISK, CRC_16_TMS37157, CRC_16_UMTS, CRC_16_USB, CRC_16_XMODEM, CRC_17_CAN_FD,
    CRC_21_CAN_FD, CRC_24_BLE, CRC_24_FLEXRAY_A, CRC_24_FLEXRAY_B, CRC_24_INTERLAKEN, CRC_24_LTE_A,
    CRC_24_LTE_B, CRC_24_OPENPGP, CRC_24_OS_9, CRC_30_CDMA, CRC_31_PHILIPS, CRC_32_AIXM,
    CRC_32_AUTOSAR, CRC_32_BASE91_D, CRC_32_BZIP2, CRC_32_CD_ROM_EDC, CRC_32_CKSUM, CRC_32_ISCSI,
    CRC_32_ISO_HDLC, CRC_32_JAMCRC, CRC_32_MEF, CRC_32_MPEG_2, CRC_32_XFER, CRC_3_GSM, CRC_3_ROHC,
    CRC_40_GSM, CRC_4_G_704, CRC_4_INTERLAKEN, CRC_5_EPC_C1G2, CRC_5_G_704, CRC_5_USB,
    CRC_64_ECMA_182, CRC_64_GO_ISO, CRC_64_MS, CRC_64_REDIS, CRC_64_WE, CRC_64_XZ,
    CRC_6_CDMA2000_A, CRC_6_CDMA2000_B, CRC_6_DARC, CRC_6_GSM, CRC_6_G_704, CRC_7_MMC, CRC_7_ROHC,
    CRC_7_UMTS, CRC_8_AUTOSAR, CRC_8_BLUETOOTH, CRC_8_CDMA2000, CRC_8_DARC, CRC_8_DVB_S2,
    CRC_8_GSM_A, CRC_8_GSM_B, CRC_8_HITAG, CRC_8_I_432_1, CRC_8_I_CODE, CRC_8_LTE, CRC_8_MAXIM_DOW,
    CRC_8_MIFARE_MAD, CRC_8_NRSC_5, CRC_8_OPENSAFETY, CRC_8_ROHC, CRC_8_SAE_J1850, CRC_8_SMBUS,
    CRC_8_TECH_3250, CRC_8_WCDMA,
};

use log::info;
use regex::Regex;
use std::str::FromStr;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement, Storage};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum InputMode {
    Ascii,
    Hex,
    Binary,
    Decimal,
    Octal,
}

#[derive(Clone, PartialEq)]
pub enum OutputMode {
    Decimal,
    Hex,
    Binary,
    Octal,
}

#[derive(Clone, PartialEq)]
pub enum HexStyle {
    WithPrefix,     // 0x48
    ShortPrefix,    // x48
    NoPrefix,       // 48
    EscapeSequence, // \x48
}

#[derive(Clone, PartialEq)]
pub enum BinaryStyle {
    WithPrefix,    // 0b01001000
    ShortPrefix,   // b01001000
    NoPrefix,      // 01001000
}

#[derive(Clone, PartialEq)]
pub enum OctalStyle {
    WithPrefix,     // 0o110
    ShortPrefix,    // o110
    NoPrefix,       // 110
    EscapeSequence, // \110
}

#[derive(Clone, PartialEq)]
pub enum Endianness {
    BigEndian,    // 네트워크 바이트 순서 (MSB first)
    LittleEndian, // 인텔 x86 바이트 순서 (LSB first)
}

#[derive(Clone, PartialEq)]
pub enum ByteFormatting {
    Continuous,   // 0x12345678
    ByteSeparated, // 0x12 0x34 0x56 0x78
}

#[derive(PartialEq, Clone, Copy)]
enum CrcAlgorithm {
    Crc3Gsm,
    Crc3Rohc,
    Crc4G704,
    Crc4Interlaken,
    Crc5EpcC1g2,
    Crc5G704,
    Crc5Usb,
    Crc6Cdma2000A,
    Crc6Cdma2000B,
    Crc6Darc,
    Crc6Gsm,
    Crc6G704,
    Crc7Mmc,
    Crc7Rohc,
    Crc7Umts,
    Crc8Autosar,
    Crc8Bluetooth,
    Crc8Cdma2000,
    Crc8Darc,
    Crc8DvbS2,
    Crc8GsmA,
    Crc8GsmB,
    Crc8Hitag,
    Crc8I4321,
    Crc8ICode,
    Crc8Lte,
    Crc8MaximDow,
    Crc8MifareMad,
    Crc8Nrsc5,
    Crc8Opensafety,
    Crc8Rohc,
    Crc8SaeJ1850,
    Crc8Smbus,
    Crc8Tech3250,
    Crc8Wcdma,
    Crc10Atm,
    Crc10Cdma2000,
    Crc10Gsm,
    Crc11Flexray,
    Crc11Umts,
    Crc12Cdma2000,
    Crc12Dect,
    Crc12Gsm,
    Crc12Umts,
    Crc13Bbc,
    Crc14Darc,
    Crc14Gsm,
    Crc15Can,
    Crc15Mpt1327,
    Crc16Arc,
    Crc16Cdma2000,
    Crc16Cms,
    Crc16Dds110,
    Crc16DectR,
    Crc16DectX,
    Crc16Dnp,
    Crc16En13757,
    Crc16Genibus,
    Crc16Gsm,
    Crc16Ibm3740,
    Crc16IbmSdlc,
    Crc16IsoIec144433A,
    Crc16Kermit,
    Crc16Lj1200,
    Crc16M17,
    Crc16MaximDow,
    Crc16Mcrf4xx,
    Crc16Modbus,
    Crc16Nrsc5,
    Crc16OpensafetyA,
    Crc16OpensafetyB,
    Crc16Profibus,
    Crc16Riello,
    Crc16SpiFujitsu,
    Crc16T10Dif,
    Crc16Teledisk,
    Crc16Tms37157,
    Crc16Umts,
    Crc16Usb,
    Crc16Xmodem,
    Crc17CanFd,
    Crc21CanFd,
    Crc24Ble,
    Crc24FlexrayA,
    Crc24FlexrayB,
    Crc24Interlaken,
    Crc24LteA,
    Crc24LteB,
    Crc24Openpgp,
    Crc24Os9,
    Crc30Cdma,
    Crc31Philips,
    Crc32Aixm,
    Crc32Autosar,
    Crc32Base91D,
    Crc32Bzip2,
    Crc32CdRomEdc,
    Crc32Cksum,
    Crc32Iscsi,
    Crc32IsoHdlc,
    Crc32Jamcrc,
    Crc32Mef,
    Crc32Mpeg2,
    Crc32Xfer,
    Crc40Gsm,
    Crc64Ecma182,
    Crc64GoIso,
    Crc64Ms,
    Crc64Redis,
    Crc64We,
    Crc64Xz,
}

impl CrcAlgorithm {
    fn name(&self) -> &str {
        match self {
            CrcAlgorithm::Crc3Gsm => "CRC-3/GSM",
            CrcAlgorithm::Crc3Rohc => "CRC-3/ROHC",
            CrcAlgorithm::Crc4G704 => "CRC-4/G-704",
            CrcAlgorithm::Crc4Interlaken => "CRC-4/INTERLAKEN",
            CrcAlgorithm::Crc5EpcC1g2 => "CRC-5/EPC-C1G2",
            CrcAlgorithm::Crc5G704 => "CRC-5/G-704",
            CrcAlgorithm::Crc5Usb => "CRC-5/USB",
            CrcAlgorithm::Crc6Cdma2000A => "CRC-6/CDMA2000-A",
            CrcAlgorithm::Crc6Cdma2000B => "CRC-6/CDMA2000-B",
            CrcAlgorithm::Crc6Darc => "CRC-6/DARC",
            CrcAlgorithm::Crc6Gsm => "CRC-6/GSM",
            CrcAlgorithm::Crc6G704 => "CRC-6/G-704",
            CrcAlgorithm::Crc7Mmc => "CRC-7/MMC",
            CrcAlgorithm::Crc7Rohc => "CRC-7/ROHC",
            CrcAlgorithm::Crc7Umts => "CRC-7/UMTS",
            CrcAlgorithm::Crc8Autosar => "CRC-8/AUTOSAR",
            CrcAlgorithm::Crc8Bluetooth => "CRC-8/BLUETOOTH",
            CrcAlgorithm::Crc8Cdma2000 => "CRC-8/CDMA2000",
            CrcAlgorithm::Crc8Darc => "CRC-8/DARC",
            CrcAlgorithm::Crc8DvbS2 => "CRC-8/DVB-S2",
            CrcAlgorithm::Crc8GsmA => "CRC-8/GSM-A",
            CrcAlgorithm::Crc8GsmB => "CRC-8/GSM-B",
            CrcAlgorithm::Crc8Hitag => "CRC-8/HITAG",
            CrcAlgorithm::Crc8I4321 => "CRC-8/I-432-1",
            CrcAlgorithm::Crc8ICode => "CRC-8/I-CODE",
            CrcAlgorithm::Crc8Lte => "CRC-8/LTE",
            CrcAlgorithm::Crc8MaximDow => "CRC-8/MAXIM-DOW",
            CrcAlgorithm::Crc8MifareMad => "CRC-8/MIFARE-MAD",
            CrcAlgorithm::Crc8Nrsc5 => "CRC-8/NRSC-5",
            CrcAlgorithm::Crc8Opensafety => "CRC-8/OPENSAFETY",
            CrcAlgorithm::Crc8Rohc => "CRC-8/ROHC",
            CrcAlgorithm::Crc8SaeJ1850 => "CRC-8/SAE-J1850",
            CrcAlgorithm::Crc8Smbus => "CRC-8/SMBUS",
            CrcAlgorithm::Crc8Tech3250 => "CRC-8/TECH-3250",
            CrcAlgorithm::Crc8Wcdma => "CRC-8/WCDMA",
            CrcAlgorithm::Crc10Atm => "CRC-10/ATM",
            CrcAlgorithm::Crc10Cdma2000 => "CRC-10/CDMA2000",
            CrcAlgorithm::Crc10Gsm => "CRC-10/GSM",
            CrcAlgorithm::Crc11Flexray => "CRC-11/FLEXRAY",
            CrcAlgorithm::Crc11Umts => "CRC-11/UMTS",
            CrcAlgorithm::Crc12Cdma2000 => "CRC-12/CDMA2000",
            CrcAlgorithm::Crc12Dect => "CRC-12/DECT",
            CrcAlgorithm::Crc12Gsm => "CRC-12/GSM",
            CrcAlgorithm::Crc12Umts => "CRC-12/UMTS",
            CrcAlgorithm::Crc13Bbc => "CRC-13/BBC",
            CrcAlgorithm::Crc14Darc => "CRC-14/DARC",
            CrcAlgorithm::Crc14Gsm => "CRC-14/GSM",
            CrcAlgorithm::Crc15Can => "CRC-15/CAN",
            CrcAlgorithm::Crc15Mpt1327 => "CRC-15/MPT1327",
            CrcAlgorithm::Crc16Arc => "CRC-16/ARC",
            CrcAlgorithm::Crc16Cdma2000 => "CRC-16/CDMA2000",
            CrcAlgorithm::Crc16Cms => "CRC-16/CMS",
            CrcAlgorithm::Crc16Dds110 => "CRC-16/DDS-110",
            CrcAlgorithm::Crc16DectR => "CRC-16/DECT-R",
            CrcAlgorithm::Crc16DectX => "CRC-16/DECT-X",
            CrcAlgorithm::Crc16Dnp => "CRC-16/DNP",
            CrcAlgorithm::Crc16En13757 => "CRC-16/EN-13757",
            CrcAlgorithm::Crc16Genibus => "CRC-16/GENIBUS",
            CrcAlgorithm::Crc16Gsm => "CRC-16/GSM",
            CrcAlgorithm::Crc16Ibm3740 => "CRC-16/IBM-3740",
            CrcAlgorithm::Crc16IbmSdlc => "CRC-16/IBM-SDLC",
            CrcAlgorithm::Crc16IsoIec144433A => "CRC-16/ISO-IEC-14443-3-A",
            CrcAlgorithm::Crc16Kermit => "CRC-16/KERMIT",
            CrcAlgorithm::Crc16Lj1200 => "CRC-16/LJ1200",
            CrcAlgorithm::Crc16M17 => "CRC-16/M17",
            CrcAlgorithm::Crc16MaximDow => "CRC-16/MAXIM-DOW",
            CrcAlgorithm::Crc16Mcrf4xx => "CRC-16/MCRF4XX",
            CrcAlgorithm::Crc16Modbus => "CRC-16/MODBUS",
            CrcAlgorithm::Crc16Nrsc5 => "CRC-16/NRSC-5",
            CrcAlgorithm::Crc16OpensafetyA => "CRC-16/OPENSAFETY-A",
            CrcAlgorithm::Crc16OpensafetyB => "CRC-16/OPENSAFETY-B",
            CrcAlgorithm::Crc16Profibus => "CRC-16/PROFIBUS",
            CrcAlgorithm::Crc16Riello => "CRC-16/RIELLO",
            CrcAlgorithm::Crc16SpiFujitsu => "CRC-16/SPI-FUJITSU",
            CrcAlgorithm::Crc16T10Dif => "CRC-16/T10-DIF",
            CrcAlgorithm::Crc16Teledisk => "CRC-16/TELEDISK",
            CrcAlgorithm::Crc16Tms37157 => "CRC-16/TMS37157",
            CrcAlgorithm::Crc16Umts => "CRC-16/UMTS",
            CrcAlgorithm::Crc16Usb => "CRC-16/USB",
            CrcAlgorithm::Crc16Xmodem => "CRC-16/XMODEM",
            CrcAlgorithm::Crc17CanFd => "CRC-17/CAN-FD",
            CrcAlgorithm::Crc21CanFd => "CRC-21/CAN-FD",
            CrcAlgorithm::Crc24Ble => "CRC-24/BLE",
            CrcAlgorithm::Crc24FlexrayA => "CRC-24/FLEXRAY-A",
            CrcAlgorithm::Crc24FlexrayB => "CRC-24/FLEXRAY-B",
            CrcAlgorithm::Crc24Interlaken => "CRC-24/INTERLAKEN",
            CrcAlgorithm::Crc24LteA => "CRC-24/LTE-A",
            CrcAlgorithm::Crc24LteB => "CRC-24/LTE-B",
            CrcAlgorithm::Crc24Openpgp => "CRC-24/OPENPGP",
            CrcAlgorithm::Crc24Os9 => "CRC-24/OS-9",
            CrcAlgorithm::Crc30Cdma => "CRC-30/CDMA",
            CrcAlgorithm::Crc31Philips => "CRC-31/PHILIPS",
            CrcAlgorithm::Crc32Aixm => "CRC-32/AIXM",
            CrcAlgorithm::Crc32Autosar => "CRC-32/AUTOSAR",
            CrcAlgorithm::Crc32Base91D => "CRC-32/BASE91-D",
            CrcAlgorithm::Crc32Bzip2 => "CRC-32/BZIP2",
            CrcAlgorithm::Crc32CdRomEdc => "CRC-32/CD-ROM-EDC",
            CrcAlgorithm::Crc32Cksum => "CRC-32/CKSUM",
            CrcAlgorithm::Crc32Iscsi => "CRC-32/ISCSI",
            CrcAlgorithm::Crc32IsoHdlc => "CRC-32/ISO-HDLC",
            CrcAlgorithm::Crc32Jamcrc => "CRC-32/JAMCRC",
            CrcAlgorithm::Crc32Mef => "CRC-32/MEF",
            CrcAlgorithm::Crc32Mpeg2 => "CRC-32/MPEG-2",
            CrcAlgorithm::Crc32Xfer => "CRC-32/XFER",
            CrcAlgorithm::Crc40Gsm => "CRC-40/GSM",
            CrcAlgorithm::Crc64Ecma182 => "CRC-64/ECMA-182",
            CrcAlgorithm::Crc64GoIso => "CRC-64/GO-ISO",
            CrcAlgorithm::Crc64Ms => "CRC-64/MS",
            CrcAlgorithm::Crc64Redis => "CRC-64/REDIS",
            CrcAlgorithm::Crc64We => "CRC-64/WE",
            CrcAlgorithm::Crc64Xz => "CRC-64/XZ",
        }
    }

    fn from_name(name: &str) -> Option<CrcAlgorithm> {
        match name {
            "CRC-3/GSM" => Some(CrcAlgorithm::Crc3Gsm),
            "CRC-3/ROHC" => Some(CrcAlgorithm::Crc3Rohc),
            "CRC-4/G-704" => Some(CrcAlgorithm::Crc4G704),
            "CRC-4/INTERLAKEN" => Some(CrcAlgorithm::Crc4Interlaken),
            "CRC-5/EPC-C1G2" => Some(CrcAlgorithm::Crc5EpcC1g2),
            "CRC-5/G-704" => Some(CrcAlgorithm::Crc5G704),
            "CRC-5/USB" => Some(CrcAlgorithm::Crc5Usb),
            "CRC-6/CDMA2000-A" => Some(CrcAlgorithm::Crc6Cdma2000A),
            "CRC-6/CDMA2000-B" => Some(CrcAlgorithm::Crc6Cdma2000B),
            "CRC-6/DARC" => Some(CrcAlgorithm::Crc6Darc),
            "CRC-6/GSM" => Some(CrcAlgorithm::Crc6Gsm),
            "CRC-6/G-704" => Some(CrcAlgorithm::Crc6G704),
            "CRC-7/MMC" => Some(CrcAlgorithm::Crc7Mmc),
            "CRC-7/ROHC" => Some(CrcAlgorithm::Crc7Rohc),
            "CRC-7/UMTS" => Some(CrcAlgorithm::Crc7Umts),
            "CRC-8/AUTOSAR" => Some(CrcAlgorithm::Crc8Autosar),
            "CRC-8/BLUETOOTH" => Some(CrcAlgorithm::Crc8Bluetooth),
            "CRC-8/CDMA2000" => Some(CrcAlgorithm::Crc8Cdma2000),
            "CRC-8/DARC" => Some(CrcAlgorithm::Crc8Darc),
            "CRC-8/DVB-S2" => Some(CrcAlgorithm::Crc8DvbS2),
            "CRC-8/GSM-A" => Some(CrcAlgorithm::Crc8GsmA),
            "CRC-8/GSM-B" => Some(CrcAlgorithm::Crc8GsmB),
            "CRC-8/HITAG" => Some(CrcAlgorithm::Crc8Hitag),
            "CRC-8/I-432-1" => Some(CrcAlgorithm::Crc8I4321),
            "CRC-8/I-CODE" => Some(CrcAlgorithm::Crc8ICode),
            "CRC-8/LTE" => Some(CrcAlgorithm::Crc8Lte),
            "CRC-8/MAXIM-DOW" => Some(CrcAlgorithm::Crc8MaximDow),
            "CRC-8/MIFARE-MAD" => Some(CrcAlgorithm::Crc8MifareMad),
            "CRC-8/NRSC-5" => Some(CrcAlgorithm::Crc8Nrsc5),
            "CRC-8/OPENSAFETY" => Some(CrcAlgorithm::Crc8Opensafety),
            "CRC-8/ROHC" => Some(CrcAlgorithm::Crc8Rohc),
            "CRC-8/SAE-J1850" => Some(CrcAlgorithm::Crc8SaeJ1850),
            "CRC-8/SMBUS" => Some(CrcAlgorithm::Crc8Smbus),
            "CRC-8/TECH-3250" => Some(CrcAlgorithm::Crc8Tech3250),
            "CRC-8/WCDMA" => Some(CrcAlgorithm::Crc8Wcdma),
            "CRC-10/ATM" => Some(CrcAlgorithm::Crc10Atm),
            "CRC-10/CDMA2000" => Some(CrcAlgorithm::Crc10Cdma2000),
            "CRC-10/GSM" => Some(CrcAlgorithm::Crc10Gsm),
            "CRC-11/FLEXRAY" => Some(CrcAlgorithm::Crc11Flexray),
            "CRC-11/UMTS" => Some(CrcAlgorithm::Crc11Umts),
            "CRC-12/CDMA2000" => Some(CrcAlgorithm::Crc12Cdma2000),
            "CRC-12/DECT" => Some(CrcAlgorithm::Crc12Dect),
            "CRC-12/GSM" => Some(CrcAlgorithm::Crc12Gsm),
            "CRC-12/UMTS" => Some(CrcAlgorithm::Crc12Umts),
            "CRC-13/BBC" => Some(CrcAlgorithm::Crc13Bbc),
            "CRC-14/DARC" => Some(CrcAlgorithm::Crc14Darc),
            "CRC-14/GSM" => Some(CrcAlgorithm::Crc14Gsm),
            "CRC-15/CAN" => Some(CrcAlgorithm::Crc15Can),
            "CRC-15/MPT1327" => Some(CrcAlgorithm::Crc15Mpt1327),
            "CRC-16/ARC" => Some(CrcAlgorithm::Crc16Arc),
            "CRC-16/CDMA2000" => Some(CrcAlgorithm::Crc16Cdma2000),
            "CRC-16/CMS" => Some(CrcAlgorithm::Crc16Cms),
            "CRC-16/DDS-110" => Some(CrcAlgorithm::Crc16Dds110),
            "CRC-16/DECT-R" => Some(CrcAlgorithm::Crc16DectR),
            "CRC-16/DECT-X" => Some(CrcAlgorithm::Crc16DectX),
            "CRC-16/DNP" => Some(CrcAlgorithm::Crc16Dnp),
            "CRC-16/EN-13757" => Some(CrcAlgorithm::Crc16En13757),
            "CRC-16/GENIBUS" => Some(CrcAlgorithm::Crc16Genibus),
            "CRC-16/GSM" => Some(CrcAlgorithm::Crc16Gsm),
            "CRC-16/IBM-3740" => Some(CrcAlgorithm::Crc16Ibm3740),
            "CRC-16/IBM-SDLC" => Some(CrcAlgorithm::Crc16IbmSdlc),
            "CRC-16/ISO-IEC-14443-3-A" => Some(CrcAlgorithm::Crc16IsoIec144433A),
            "CRC-16/KERMIT" => Some(CrcAlgorithm::Crc16Kermit),
            "CRC-16/LJ1200" => Some(CrcAlgorithm::Crc16Lj1200),
            "CRC-16/M17" => Some(CrcAlgorithm::Crc16M17),
            "CRC-16/MAXIM-DOW" => Some(CrcAlgorithm::Crc16MaximDow),
            "CRC-16/MCRF4XX" => Some(CrcAlgorithm::Crc16Mcrf4xx),
            "CRC-16/MODBUS" => Some(CrcAlgorithm::Crc16Modbus),
            "CRC-16/NRSC-5" => Some(CrcAlgorithm::Crc16Nrsc5),
            "CRC-16/OPENSAFETY-A" => Some(CrcAlgorithm::Crc16OpensafetyA),
            "CRC-16/OPENSAFETY-B" => Some(CrcAlgorithm::Crc16OpensafetyB),
            "CRC-16/PROFIBUS" => Some(CrcAlgorithm::Crc16Profibus),
            "CRC-16/RIELLO" => Some(CrcAlgorithm::Crc16Riello),
            "CRC-16/SPI-FUJITSU" => Some(CrcAlgorithm::Crc16SpiFujitsu),
            "CRC-16/T10-DIF" => Some(CrcAlgorithm::Crc16T10Dif),
            "CRC-16/TELEDISK" => Some(CrcAlgorithm::Crc16Teledisk),
            "CRC-16/TMS37157" => Some(CrcAlgorithm::Crc16Tms37157),
            "CRC-16/UMTS" => Some(CrcAlgorithm::Crc16Umts),
            "CRC-16/USB" => Some(CrcAlgorithm::Crc16Usb),
            "CRC-16/XMODEM" => Some(CrcAlgorithm::Crc16Xmodem),
            "CRC-17/CAN-FD" => Some(CrcAlgorithm::Crc17CanFd),
            "CRC-21/CAN-FD" => Some(CrcAlgorithm::Crc21CanFd),
            "CRC-24/BLE" => Some(CrcAlgorithm::Crc24Ble),
            "CRC-24/FLEXRAY-A" => Some(CrcAlgorithm::Crc24FlexrayA),
            "CRC-24/FLEXRAY-B" => Some(CrcAlgorithm::Crc24FlexrayB),
            "CRC-24/INTERLAKEN" => Some(CrcAlgorithm::Crc24Interlaken),
            "CRC-24/LTE-A" => Some(CrcAlgorithm::Crc24LteA),
            "CRC-24/LTE-B" => Some(CrcAlgorithm::Crc24LteB),
            "CRC-24/OPENPGP" => Some(CrcAlgorithm::Crc24Openpgp),
            "CRC-24/OS-9" => Some(CrcAlgorithm::Crc24Os9),
            "CRC-30/CDMA" => Some(CrcAlgorithm::Crc30Cdma),
            "CRC-31/PHILIPS" => Some(CrcAlgorithm::Crc31Philips),
            "CRC-32/AIXM" => Some(CrcAlgorithm::Crc32Aixm),
            "CRC-32/AUTOSAR" => Some(CrcAlgorithm::Crc32Autosar),
            "CRC-32/BASE91-D" => Some(CrcAlgorithm::Crc32Base91D),
            "CRC-32/BZIP2" => Some(CrcAlgorithm::Crc32Bzip2),
            "CRC-32/CD-ROM-EDC" => Some(CrcAlgorithm::Crc32CdRomEdc),
            "CRC-32/CKSUM" => Some(CrcAlgorithm::Crc32Cksum),
            "CRC-32/ISCSI" => Some(CrcAlgorithm::Crc32Iscsi),
            "CRC-32/ISO-HDLC" => Some(CrcAlgorithm::Crc32IsoHdlc),
            "CRC-32/JAMCRC" => Some(CrcAlgorithm::Crc32Jamcrc),
            "CRC-32/MEF" => Some(CrcAlgorithm::Crc32Mef),
            "CRC-32/MPEG-2" => Some(CrcAlgorithm::Crc32Mpeg2),
            "CRC-32/XFER" => Some(CrcAlgorithm::Crc32Xfer),
            "CRC-40/GSM" => Some(CrcAlgorithm::Crc40Gsm),
            "CRC-64/ECMA-182" => Some(CrcAlgorithm::Crc64Ecma182),
            "CRC-64/GO-ISO" => Some(CrcAlgorithm::Crc64GoIso),
            "CRC-64/MS" => Some(CrcAlgorithm::Crc64Ms),
            "CRC-64/REDIS" => Some(CrcAlgorithm::Crc64Redis),
            "CRC-64/WE" => Some(CrcAlgorithm::Crc64We),
            "CRC-64/XZ" => Some(CrcAlgorithm::Crc64Xz),
            _ => None, // name이 매칭되지 않으면 None 반환
        }
    }

    fn calculate(&self, data: &[u8]) -> (u64, u8) {
        match self {
            CrcAlgorithm::Crc3Gsm => {
                let crc = Crc::<u8>::new(&CRC_3_GSM);
                (crc.checksum(data) as u64, 3)
            }
            CrcAlgorithm::Crc3Rohc => {
                let crc = Crc::<u8>::new(&CRC_3_ROHC);
                (crc.checksum(data) as u64, 3)
            }
            CrcAlgorithm::Crc4G704 => {
                let crc = Crc::<u8>::new(&CRC_4_G_704);
                (crc.checksum(data) as u64, 4)
            }
            CrcAlgorithm::Crc4Interlaken => {
                let crc = Crc::<u8>::new(&CRC_4_INTERLAKEN);
                (crc.checksum(data) as u64, 4)
            }
            CrcAlgorithm::Crc5EpcC1g2 => {
                let crc = Crc::<u8>::new(&CRC_5_EPC_C1G2);
                (crc.checksum(data) as u64, 5)
            }
            CrcAlgorithm::Crc5G704 => {
                let crc = Crc::<u8>::new(&CRC_5_G_704);
                (crc.checksum(data) as u64, 5)
            }
            CrcAlgorithm::Crc5Usb => {
                let crc = Crc::<u8>::new(&CRC_5_USB);
                (crc.checksum(data) as u64, 5)
            }
            CrcAlgorithm::Crc6Cdma2000A => {
                let crc = Crc::<u8>::new(&CRC_6_CDMA2000_A);
                (crc.checksum(data) as u64, 6)
            }
            CrcAlgorithm::Crc6Cdma2000B => {
                let crc = Crc::<u8>::new(&CRC_6_CDMA2000_B);
                (crc.checksum(data) as u64, 6)
            }
            CrcAlgorithm::Crc6Darc => {
                let crc = Crc::<u8>::new(&CRC_6_DARC);
                (crc.checksum(data) as u64, 6)
            }
            CrcAlgorithm::Crc6Gsm => {
                let crc = Crc::<u8>::new(&CRC_6_GSM);
                (crc.checksum(data) as u64, 6)
            }
            CrcAlgorithm::Crc6G704 => {
                let crc = Crc::<u8>::new(&CRC_6_G_704);
                (crc.checksum(data) as u64, 6)
            }
            CrcAlgorithm::Crc7Mmc => {
                let crc = Crc::<u8>::new(&CRC_7_MMC);
                (crc.checksum(data) as u64, 7)
            }
            CrcAlgorithm::Crc7Rohc => {
                let crc = Crc::<u8>::new(&CRC_7_ROHC);
                (crc.checksum(data) as u64, 7)
            }
            CrcAlgorithm::Crc7Umts => {
                let crc = Crc::<u8>::new(&CRC_7_UMTS);
                (crc.checksum(data) as u64, 7)
            }
            CrcAlgorithm::Crc8Autosar => {
                let crc = Crc::<u8>::new(&CRC_8_AUTOSAR);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Bluetooth => {
                let crc = Crc::<u8>::new(&CRC_8_BLUETOOTH);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Cdma2000 => {
                let crc = Crc::<u8>::new(&CRC_8_CDMA2000);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Darc => {
                let crc = Crc::<u8>::new(&CRC_8_DARC);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8DvbS2 => {
                let crc = Crc::<u8>::new(&CRC_8_DVB_S2);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8GsmA => {
                let crc = Crc::<u8>::new(&CRC_8_GSM_A);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8GsmB => {
                let crc = Crc::<u8>::new(&CRC_8_GSM_B);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Hitag => {
                let crc = Crc::<u8>::new(&CRC_8_HITAG);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8I4321 => {
                let crc = Crc::<u8>::new(&CRC_8_I_432_1);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8ICode => {
                let crc = Crc::<u8>::new(&CRC_8_I_CODE);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Lte => {
                let crc = Crc::<u8>::new(&CRC_8_LTE);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8MaximDow => {
                let crc = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8MifareMad => {
                let crc = Crc::<u8>::new(&CRC_8_MIFARE_MAD);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Nrsc5 => {
                let crc = Crc::<u8>::new(&CRC_8_NRSC_5);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Opensafety => {
                let crc = Crc::<u8>::new(&CRC_8_OPENSAFETY);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Rohc => {
                let crc = Crc::<u8>::new(&CRC_8_ROHC);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8SaeJ1850 => {
                let crc = Crc::<u8>::new(&CRC_8_SAE_J1850);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Smbus => {
                let crc = Crc::<u8>::new(&CRC_8_SMBUS);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Tech3250 => {
                let crc = Crc::<u8>::new(&CRC_8_TECH_3250);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc8Wcdma => {
                let crc = Crc::<u8>::new(&CRC_8_WCDMA);
                (crc.checksum(data) as u64, 8)
            }
            CrcAlgorithm::Crc10Atm => {
                let crc = Crc::<u16>::new(&CRC_10_ATM);
                (crc.checksum(data) as u64, 10)
            }
            CrcAlgorithm::Crc10Cdma2000 => {
                let crc = Crc::<u16>::new(&CRC_10_CDMA2000);
                (crc.checksum(data) as u64, 10)
            }
            CrcAlgorithm::Crc10Gsm => {
                let crc = Crc::<u16>::new(&CRC_10_GSM);
                (crc.checksum(data) as u64, 10)
            }
            CrcAlgorithm::Crc11Flexray => {
                let crc = Crc::<u16>::new(&CRC_11_FLEXRAY);
                (crc.checksum(data) as u64, 11)
            }
            CrcAlgorithm::Crc11Umts => {
                let crc = Crc::<u16>::new(&CRC_11_UMTS);
                (crc.checksum(data) as u64, 11)
            }
            CrcAlgorithm::Crc12Cdma2000 => {
                let crc = Crc::<u16>::new(&CRC_12_CDMA2000);
                (crc.checksum(data) as u64, 12)
            }
            CrcAlgorithm::Crc12Dect => {
                let crc = Crc::<u16>::new(&CRC_12_DECT);
                (crc.checksum(data) as u64, 12)
            }
            CrcAlgorithm::Crc12Gsm => {
                let crc = Crc::<u16>::new(&CRC_12_GSM);
                (crc.checksum(data) as u64, 12)
            }
            CrcAlgorithm::Crc12Umts => {
                let crc = Crc::<u16>::new(&CRC_12_UMTS);
                (crc.checksum(data) as u64, 12)
            }
            CrcAlgorithm::Crc13Bbc => {
                let crc = Crc::<u16>::new(&CRC_13_BBC);
                (crc.checksum(data) as u64, 13)
            }
            CrcAlgorithm::Crc14Darc => {
                let crc = Crc::<u16>::new(&CRC_14_DARC);
                (crc.checksum(data) as u64, 14)
            }
            CrcAlgorithm::Crc14Gsm => {
                let crc = Crc::<u16>::new(&CRC_14_GSM);
                (crc.checksum(data) as u64, 14)
            }
            CrcAlgorithm::Crc15Can => {
                let crc = Crc::<u16>::new(&CRC_15_CAN);
                (crc.checksum(data) as u64, 15)
            }
            CrcAlgorithm::Crc15Mpt1327 => {
                let crc = Crc::<u16>::new(&CRC_15_MPT1327);
                (crc.checksum(data) as u64, 15)
            }
            CrcAlgorithm::Crc16Arc => {
                let crc = Crc::<u16>::new(&CRC_16_ARC);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Cdma2000 => {
                let crc = Crc::<u16>::new(&CRC_16_CDMA2000);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Cms => {
                let crc = Crc::<u16>::new(&CRC_16_CMS);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Dds110 => {
                let crc = Crc::<u16>::new(&CRC_16_DDS_110);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16DectR => {
                let crc = Crc::<u16>::new(&CRC_16_DECT_R);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16DectX => {
                let crc = Crc::<u16>::new(&CRC_16_DECT_X);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Dnp => {
                let crc = Crc::<u16>::new(&CRC_16_DNP);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16En13757 => {
                let crc = Crc::<u16>::new(&CRC_16_EN_13757);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Genibus => {
                let crc = Crc::<u16>::new(&CRC_16_GENIBUS);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Gsm => {
                let crc = Crc::<u16>::new(&CRC_16_GSM);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Ibm3740 => {
                let crc = Crc::<u16>::new(&CRC_16_IBM_3740);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16IbmSdlc => {
                let crc = Crc::<u16>::new(&CRC_16_IBM_SDLC);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16IsoIec144433A => {
                let crc = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Kermit => {
                let crc = Crc::<u16>::new(&CRC_16_KERMIT);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Lj1200 => {
                let crc = Crc::<u16>::new(&CRC_16_LJ1200);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16M17 => {
                let crc = Crc::<u16>::new(&CRC_16_M17);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16MaximDow => {
                let crc = Crc::<u16>::new(&CRC_16_MAXIM_DOW);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Mcrf4xx => {
                let crc = Crc::<u16>::new(&CRC_16_MCRF4XX);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Modbus => {
                let crc = Crc::<u16>::new(&CRC_16_MODBUS);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Nrsc5 => {
                let crc = Crc::<u16>::new(&CRC_16_NRSC_5);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16OpensafetyA => {
                let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_A);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16OpensafetyB => {
                let crc = Crc::<u16>::new(&CRC_16_OPENSAFETY_B);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Profibus => {
                let crc = Crc::<u16>::new(&CRC_16_PROFIBUS);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Riello => {
                let crc = Crc::<u16>::new(&CRC_16_RIELLO);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16SpiFujitsu => {
                let crc = Crc::<u16>::new(&CRC_16_SPI_FUJITSU);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16T10Dif => {
                let crc = Crc::<u16>::new(&CRC_16_T10_DIF);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Teledisk => {
                let crc = Crc::<u16>::new(&CRC_16_TELEDISK);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Tms37157 => {
                let crc = Crc::<u16>::new(&CRC_16_TMS37157);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Umts => {
                let crc = Crc::<u16>::new(&CRC_16_UMTS);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Usb => {
                let crc = Crc::<u16>::new(&CRC_16_USB);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc16Xmodem => {
                let crc = Crc::<u16>::new(&CRC_16_XMODEM);
                (crc.checksum(data) as u64, 16)
            }
            CrcAlgorithm::Crc17CanFd => {
                let crc = Crc::<u32>::new(&CRC_17_CAN_FD);
                (crc.checksum(data) as u64, 17)
            }
            CrcAlgorithm::Crc21CanFd => {
                let crc = Crc::<u32>::new(&CRC_21_CAN_FD);
                (crc.checksum(data) as u64, 21)
            }
            CrcAlgorithm::Crc24Ble => {
                let crc = Crc::<u32>::new(&CRC_24_BLE);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24FlexrayA => {
                let crc = Crc::<u32>::new(&CRC_24_FLEXRAY_A);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24FlexrayB => {
                let crc = Crc::<u32>::new(&CRC_24_FLEXRAY_B);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24Interlaken => {
                let crc = Crc::<u32>::new(&CRC_24_INTERLAKEN);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24LteA => {
                let crc = Crc::<u32>::new(&CRC_24_LTE_A);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24LteB => {
                let crc = Crc::<u32>::new(&CRC_24_LTE_B);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24Openpgp => {
                let crc = Crc::<u32>::new(&CRC_24_OPENPGP);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc24Os9 => {
                let crc = Crc::<u32>::new(&CRC_24_OS_9);
                (crc.checksum(data) as u64, 24)
            }
            CrcAlgorithm::Crc30Cdma => {
                let crc = Crc::<u32>::new(&CRC_30_CDMA);
                (crc.checksum(data) as u64, 30)
            }
            CrcAlgorithm::Crc31Philips => {
                let crc = Crc::<u32>::new(&CRC_31_PHILIPS);
                (crc.checksum(data) as u64, 31)
            }
            CrcAlgorithm::Crc32Aixm => {
                let crc = Crc::<u32>::new(&CRC_32_AIXM);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Autosar => {
                let crc = Crc::<u32>::new(&CRC_32_AUTOSAR);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Base91D => {
                let crc = Crc::<u32>::new(&CRC_32_BASE91_D);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Bzip2 => {
                let crc = Crc::<u32>::new(&CRC_32_BZIP2);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32CdRomEdc => {
                let crc = Crc::<u32>::new(&CRC_32_CD_ROM_EDC);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Cksum => {
                let crc = Crc::<u32>::new(&CRC_32_CKSUM);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Iscsi => {
                let crc = Crc::<u32>::new(&CRC_32_ISCSI);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32IsoHdlc => {
                let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Jamcrc => {
                let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Mef => {
                let crc = Crc::<u32>::new(&CRC_32_MEF);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Mpeg2 => {
                let crc = Crc::<u32>::new(&CRC_32_MPEG_2);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc32Xfer => {
                let crc = Crc::<u32>::new(&CRC_32_XFER);
                (crc.checksum(data) as u64, 32)
            }
            CrcAlgorithm::Crc40Gsm => {
                let crc = Crc::<u64>::new(&CRC_40_GSM);
                (crc.checksum(data) as u64, 40)
            }
            CrcAlgorithm::Crc64Ecma182 => {
                let crc = Crc::<u64>::new(&CRC_64_ECMA_182);
                (crc.checksum(data) as u64, 64)
            }
            CrcAlgorithm::Crc64GoIso => {
                let crc = Crc::<u64>::new(&CRC_64_GO_ISO);
                (crc.checksum(data) as u64, 64)
            }
            CrcAlgorithm::Crc64Ms => {
                let crc = Crc::<u64>::new(&CRC_64_MS);
                (crc.checksum(data) as u64, 64)
            }
            CrcAlgorithm::Crc64Redis => {
                let crc = Crc::<u64>::new(&CRC_64_REDIS);
                (crc.checksum(data) as u64, 64)
            }
            CrcAlgorithm::Crc64We => {
                let crc = Crc::<u64>::new(&CRC_64_WE);
                (crc.checksum(data) as u64, 64)
            }
            CrcAlgorithm::Crc64Xz => {
                let crc = Crc::<u64>::new(&CRC_64_XZ);
                (crc.checksum(data) as u64, 64)
            }
        }
    }

    fn all() -> Vec<CrcAlgorithm> {
        vec![
            CrcAlgorithm::Crc3Gsm,
            CrcAlgorithm::Crc3Rohc,
            CrcAlgorithm::Crc4G704,
            CrcAlgorithm::Crc4Interlaken,
            CrcAlgorithm::Crc5EpcC1g2,
            CrcAlgorithm::Crc5G704,
            CrcAlgorithm::Crc5Usb,
            CrcAlgorithm::Crc6Cdma2000A,
            CrcAlgorithm::Crc6Cdma2000B,
            CrcAlgorithm::Crc6Darc,
            CrcAlgorithm::Crc6Gsm,
            CrcAlgorithm::Crc6G704,
            CrcAlgorithm::Crc7Mmc,
            CrcAlgorithm::Crc7Rohc,
            CrcAlgorithm::Crc7Umts,
            CrcAlgorithm::Crc8Autosar,
            CrcAlgorithm::Crc8Bluetooth,
            CrcAlgorithm::Crc8Cdma2000,
            CrcAlgorithm::Crc8Darc,
            CrcAlgorithm::Crc8DvbS2,
            CrcAlgorithm::Crc8GsmA,
            CrcAlgorithm::Crc8GsmB,
            CrcAlgorithm::Crc8Hitag,
            CrcAlgorithm::Crc8I4321,
            CrcAlgorithm::Crc8ICode,
            CrcAlgorithm::Crc8Lte,
            CrcAlgorithm::Crc8MaximDow,
            CrcAlgorithm::Crc8MifareMad,
            CrcAlgorithm::Crc8Nrsc5,
            CrcAlgorithm::Crc8Opensafety,
            CrcAlgorithm::Crc8Rohc,
            CrcAlgorithm::Crc8SaeJ1850,
            CrcAlgorithm::Crc8Smbus,
            CrcAlgorithm::Crc8Tech3250,
            CrcAlgorithm::Crc8Wcdma,
            CrcAlgorithm::Crc10Atm,
            CrcAlgorithm::Crc10Cdma2000,
            CrcAlgorithm::Crc10Gsm,
            CrcAlgorithm::Crc11Flexray,
            CrcAlgorithm::Crc11Umts,
            CrcAlgorithm::Crc12Cdma2000,
            CrcAlgorithm::Crc12Dect,
            CrcAlgorithm::Crc12Gsm,
            CrcAlgorithm::Crc12Umts,
            CrcAlgorithm::Crc13Bbc,
            CrcAlgorithm::Crc14Darc,
            CrcAlgorithm::Crc14Gsm,
            CrcAlgorithm::Crc15Can,
            CrcAlgorithm::Crc15Mpt1327,
            CrcAlgorithm::Crc16Arc,
            CrcAlgorithm::Crc16Cdma2000,
            CrcAlgorithm::Crc16Cms,
            CrcAlgorithm::Crc16Dds110,
            CrcAlgorithm::Crc16DectR,
            CrcAlgorithm::Crc16DectX,
            CrcAlgorithm::Crc16Dnp,
            CrcAlgorithm::Crc16En13757,
            CrcAlgorithm::Crc16Genibus,
            CrcAlgorithm::Crc16Gsm,
            CrcAlgorithm::Crc16Ibm3740,
            CrcAlgorithm::Crc16IbmSdlc,
            CrcAlgorithm::Crc16IsoIec144433A,
            CrcAlgorithm::Crc16Kermit,
            CrcAlgorithm::Crc16Lj1200,
            CrcAlgorithm::Crc16M17,
            CrcAlgorithm::Crc16MaximDow,
            CrcAlgorithm::Crc16Mcrf4xx,
            CrcAlgorithm::Crc16Modbus,
            CrcAlgorithm::Crc16Nrsc5,
            CrcAlgorithm::Crc16OpensafetyA,
            CrcAlgorithm::Crc16OpensafetyB,
            CrcAlgorithm::Crc16Profibus,
            CrcAlgorithm::Crc16Riello,
            CrcAlgorithm::Crc16SpiFujitsu,
            CrcAlgorithm::Crc16T10Dif,
            CrcAlgorithm::Crc16Teledisk,
            CrcAlgorithm::Crc16Tms37157,
            CrcAlgorithm::Crc16Umts,
            CrcAlgorithm::Crc16Usb,
            CrcAlgorithm::Crc16Xmodem,
            CrcAlgorithm::Crc17CanFd,
            CrcAlgorithm::Crc21CanFd,
            CrcAlgorithm::Crc24Ble,
            CrcAlgorithm::Crc24FlexrayA,
            CrcAlgorithm::Crc24FlexrayB,
            CrcAlgorithm::Crc24Interlaken,
            CrcAlgorithm::Crc24LteA,
            CrcAlgorithm::Crc24LteB,
            CrcAlgorithm::Crc24Openpgp,
            CrcAlgorithm::Crc24Os9,
            CrcAlgorithm::Crc30Cdma,
            CrcAlgorithm::Crc31Philips,
            CrcAlgorithm::Crc32Aixm,
            CrcAlgorithm::Crc32Autosar,
            CrcAlgorithm::Crc32Base91D,
            CrcAlgorithm::Crc32Bzip2,
            CrcAlgorithm::Crc32CdRomEdc,
            CrcAlgorithm::Crc32Cksum,
            CrcAlgorithm::Crc32Iscsi,
            CrcAlgorithm::Crc32IsoHdlc,
            CrcAlgorithm::Crc32Jamcrc,
            CrcAlgorithm::Crc32Mef,
            CrcAlgorithm::Crc32Mpeg2,
            CrcAlgorithm::Crc32Xfer,
            CrcAlgorithm::Crc40Gsm,
            CrcAlgorithm::Crc64Ecma182,
            CrcAlgorithm::Crc64GoIso,
            CrcAlgorithm::Crc64Ms,
            CrcAlgorithm::Crc64Redis,
            CrcAlgorithm::Crc64We,
            CrcAlgorithm::Crc64Xz,
        ]
    }
}

pub struct ToolCrc {
    input: String,
    input_mode: InputMode,
    output_mode: OutputMode,
    hex_style: HexStyle,
    binary_style: BinaryStyle,
    octal_style: OctalStyle,
    endianness: Endianness,
    byte_formatting: ByteFormatting,
    selected_algorithm: CrcAlgorithm,
    bytes: Vec<u8>,
    bytes_string: String,
    crc_result: u64,
    error_message: Option<String>,
    width: u8,
}

pub enum Msg {
    InputChanged(String),
    ModeChanged(InputMode),
    OutputModeChanged(OutputMode),
    HexStyleChanged(HexStyle),
    BinaryStyleChanged(BinaryStyle),
    OctalStyleChanged(OctalStyle),
    EndiannessChanged(Endianness),
    ByteFormattingChanged(ByteFormatting),
    SelectAlgorithm(String),
    CopyToClipboard(String),
    Calculate,
}

impl Component for ToolCrc {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::load_from_storage()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(value) => {
                self.input = value.clone();
                self.error_message = None;

                if value.is_empty() {
                    self.bytes.clear();
                    self.bytes_string.clear();
                    self.crc_result = 0;
                    return true;
                }

                let parsed_bytes = match self.input_mode {
                    InputMode::Ascii => Ok(self.input.as_bytes().to_vec()),
                    InputMode::Hex => self.parse_hex_input(&self.input),
                    InputMode::Binary => self.parse_binary_input(&self.input),
                    InputMode::Decimal => self.parse_decimal_input(&self.input),
                    InputMode::Octal => self.parse_octal_input(&self.input),
                };

                match parsed_bytes {
                    Ok(bytes) => {
                        self.bytes = bytes;
                        self.calculate_crc();
                        self.bytes_string = self
                            .bytes
                            .iter()
                            .map(|byte| format!("0x{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(" ");
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.bytes.clear();
                        self.bytes_string.clear();
                        self.crc_result = 0;
                    }
                }
                true
            }
            Msg::ModeChanged(mode) => {
                self.input_mode = mode;
                self.error_message = None;
                self.input = String::new();
                self.bytes.clear();
                self.bytes_string.clear();
                self.crc_result = 0;
                self.save_to_storage();
                true
            }
            Msg::OutputModeChanged(mode) => {
                self.output_mode = mode;
                self.save_to_storage();
                true
            }
            Msg::HexStyleChanged(style) => {
                self.hex_style = style;
                self.save_to_storage();
                true
            }
            Msg::BinaryStyleChanged(style) => {
                self.binary_style = style;
                self.save_to_storage();
                true
            }
            Msg::OctalStyleChanged(style) => {
                self.octal_style = style;
                self.save_to_storage();
                true
            }
            Msg::EndiannessChanged(endianness) => {
                self.endianness = endianness;
                self.save_to_storage();
                true
            }
            Msg::ByteFormattingChanged(formatting) => {
                self.byte_formatting = formatting;
                self.save_to_storage();
                true
            }
            Msg::SelectAlgorithm(value) => {
                if let Some(algorithm) = CrcAlgorithm::from_name(&value) {
                    self.selected_algorithm = algorithm;
                    self.calculate_crc();
                    self.save_to_storage();
                    true
                } else {
                    false
                }
            }
            Msg::Calculate => {
                self.calculate_crc();
                self.bytes_string = self
                    .bytes
                    .iter()
                    .map(|byte| format!("0x{:02X}", byte))
                    .collect::<Vec<String>>()
                    .join(" ");
                true
            }
            Msg::CopyToClipboard(value) => {
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    wasm_bindgen_futures::spawn_local(async move {
                        let promise = clipboard.write_text(&value);
                        let future = JsFuture::from(promise);

                        match future.await {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    });
                } else {
                    {};
                }
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();

        let onchange_mode = _ctx.link().callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            Msg::SelectAlgorithm(select.value())
        });

        let digits = ((self.width as f32 / 4.0).ceil()) as usize;
        let formatted_crc = format!("0x{:0width$X}", self.crc_result, width = digits);

        html! {
            <>
                        <h1 class="tool-title">
                            { "CRC Tool" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is CRC?"}</h2>
                            <p>{"CRC (Cyclic Redundancy Check) is a widely used error-detecting code designed to detect accidental changes to raw data. It is commonly used in digital networks, storage devices, and embedded systems to ensure data integrity."}</p>
                            <p>{"A CRC algorithm processes input data and produces a short, fixed-length checksum (the CRC value) that can be used to verify the integrity of the data during transmission or storage."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This CRC Tool Works"}</h2>
                            <p>{"This tool allows you to calculate CRC values for any input using a wide range of industry-standard CRC algorithms. You can choose the input format (ASCII or HEX), select the CRC algorithm, and instantly see the result."}</p>
                            <h3>{"🔥 Advanced Features:"}</h3>
                            <ul>
                                <li><strong>{"Comprehensive Algorithm Library:"}</strong> {"100+ CRC algorithms including CRC-3, CRC-4, CRC-5, CRC-6, CRC-7, CRC-8, CRC-10, CRC-11, CRC-12, CRC-13, CRC-14, CRC-15, CRC-16, CRC-17, CRC-21, CRC-24, CRC-30, CRC-31, CRC-32, CRC-40, CRC-64"}</li>
                                <li><strong>{"Smart Algorithm Selection:"}</strong> {"Categorized by bit-width with industry-specific recommendations and popularity rankings"}</li>
                                <li><strong>{"Multi-Format Input Support:"}</strong> {"ASCII text, HEX (multiple formats), Binary (0b/b prefix), Decimal (space-separated), and Octal (0o/o/\\ prefix)"}</li>
                                <li><strong>{"Flexible Output Customization:"}</strong> {"Choose format (HEX/DEC/BIN/OCT), style (prefix options), endianness (Big/Little), and byte formatting (continuous/separated)"}</li>
                                <li><strong>{"Real-time Calculation:"}</strong> {"Instant CRC computation as you type with comprehensive error validation"}</li>
                                <li><strong>{"Persistent Settings:"}</strong> {"Auto-save preferences to Local Storage for consistent user experience"}</li>
                                <li><strong>{"Professional-Grade Accuracy:"}</strong> {"Industry-standard implementations with verified test vectors"}</li>
                            </ul>

                            <h3>{"🎯 Algorithm Selection Intelligence"}</h3>
                            <div class="example-box">
                                <p><strong>{"Popular Algorithms by Category:"}</strong></p>
                                <ul>
                                    <li><strong>{"Network & Ethernet:"}</strong> {"CRC-32/ISO-HDLC (most common), CRC-16/MODBUS, CRC-8/AUTOSAR"}</li>
                                    <li><strong>{"Storage & Filesystems:"}</strong> {"CRC-32/MPEG-2, CRC-64/XZ, CRC-32/BZIP2"}</li>
                                    <li><strong>{"Embedded & IoT:"}</strong> {"CRC-16/CCITT, CRC-8/MAXIM-DOW, CRC-5/USB"}</li>
                                    <li><strong>{"Telecommunications:"}</strong> {"CRC-16/GSM, CRC-8/GSM-A, CRC-11/UMTS"}</li>
                                    <li><strong>{"Automotive:"}</strong> {"CRC-15/CAN, CRC-17/CAN-FD, CRC-8/SAE-J1850"}</li>
                                    <li><strong>{"Bluetooth & Wireless:"}</strong> {"CRC-24/BLE, CRC-8/BLUETOOTH, CRC-8/WCDMA"}</li>
                                </ul>
                            </div>

                            <h3>{"📊 Input Format Examples & Best Practices:"}</h3>
                            <div class="example-box">
                                <p><strong>{"ASCII input (recommended for text data):"}</strong></p>
                                <ul>
                                    <li>{"\"Hello World\" → Direct character encoding"}</li>
                                    <li>{"\"CompuTools CRC\" → UTF-8 byte sequence"}</li>
                                    <li>{"\"test\\ndata\" → Includes control characters"}</li>
                                </ul>
                                
                                <p><strong>{"HEX input (flexible formats accepted):"}</strong></p>
                                <ul>
                                    <li>{"Standard: \"0x01 0x02 0x03 0x04 0x05\""}</li>
                                    <li>{"Escape: \"\\x01\\x02\\x03\\x04\\x05\""}</li>
                                    <li>{"Short: \"x01x02x03x04x05\""}</li>
                                    <li>{"Raw: \"0102030405\""}</li>
                                    <li>{"Mixed: \"0x01 \\x02 x03 04 05\" (auto-detected)"}</li>
                                </ul>

                                <p><strong>{"Binary input (8-bit sequences):"}</strong></p>
                                <ul>
                                    <li>{"Prefixed: \"0b01001000 0b01100101\""}</li>
                                    <li>{"Short: \"b01001000 b01100101\""}</li>
                                    <li>{"Raw: \"01001000 01100101 01101100\""}</li>
                                    <li>{"Stream: \"0100100001100101011011000110110001101111\""}</li>
                                </ul>

                                <p><strong>{"Decimal input (byte values 0-255):"}</strong></p>
                                <ul>
                                    <li>{"Space-separated: \"72 101 108 108 111\""}</li>
                                    <li>{"Valid range: 0-255 per value"}</li>
                                    <li>{"Example: \"123 45 67 89 10\" → 5 bytes"}</li>
                                </ul>

                                <p><strong>{"Octal input (base-8 representation):"}</strong></p>
                                <ul>
                                    <li>{"Standard: \"0o110 0o145 0o154\""}</li>
                                    <li>{"Short: \"o110 o145 o154\""}</li>
                                    <li>{"Raw: \"110 145 154 154 157\""}</li>
                                    <li>{"Escape: \"\\110\\145\\154\\154\\157\""}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔍 CRC Algorithm Reference Guide"}</h2>
                            <p>{"Understanding which CRC algorithm to choose is crucial for compatibility and reliability. Each algorithm has specific characteristics that make it suitable for different applications."}</p>
                            
                            <h3>{"📋 Algorithm Categories & Characteristics"}</h3>
                            
                            <div style="margin: 20px 0;">
                                <h4>{"🔷 CRC-8 Family (Most Common for Small Data)"}</h4>
                                <div style="background-color: var(--color-third); padding: 10px; border-radius: 5px; margin: 10px 0;">
                                    <table style="width: 100%; border-collapse: collapse; font-size: 12px;">
                                        <thead style="background-color: var(--color-fourth); color: white;">
                                            <tr>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Algorithm"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Polynomial"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Primary Use Case"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: center;">{"Popularity"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-8/AUTOSAR"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x2F"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Automotive ECU communication"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-8/MAXIM-DOW"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x31"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"1-Wire devices, Dallas sensors"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-8/SMBUS"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x07"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"SMBus protocol, I2C communications"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-8/BLUETOOTH"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0xA7"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Bluetooth HEC calculation"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-8/DVB-S2"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0xD5"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Digital video broadcasting"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐"}</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <div style="margin: 20px 0;">
                                <h4>{"🔶 CRC-16 Family (Industry Standard)"}</h4>
                                <div style="background-color: var(--color-third); padding: 10px; border-radius: 5px; margin: 10px 0;">
                                    <table style="width: 100%; border-collapse: collapse; font-size: 12px;">
                                        <thead style="background-color: var(--color-fourth); color: white;">
                                            <tr>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Algorithm"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Polynomial"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Primary Use Case"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: center;">{"Popularity"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-16/MODBUS"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x8005"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Industrial automation, MODBUS protocol"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-16/USB"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x8005"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"USB token and data packets"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-16/XMODEM"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x1021"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"File transfer protocols (XMODEM, YMODEM)"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-16/DNP"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x3D65"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Distributed Network Protocol (power systems)"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-16/PROFIBUS"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x1DCF"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"PROFIBUS fieldbus protocol"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐"}</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <div style="margin: 20px 0;">
                                <h4>{"🔸 CRC-32 Family (Most Robust)"}</h4>
                                <div style="background-color: var(--color-third); padding: 10px; border-radius: 5px; margin: 10px 0;">
                                    <table style="width: 100%; border-collapse: collapse; font-size: 12px;">
                                        <thead style="background-color: var(--color-fourth); color: white;">
                                            <tr>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Algorithm"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Polynomial"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: left;">{"Primary Use Case"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd; text-align: center;">{"Popularity"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-32/ISO-HDLC"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x04C11DB7"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Ethernet, PNG, ZIP, most common CRC-32"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-32/MPEG-2"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x04C11DB7"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"MPEG-2 transport streams, AAC audio"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;"><strong>{"CRC-32/BZIP2"}</strong></td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x04C11DB7"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"BZIP2 compression algorithm"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-32/AUTOSAR"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0xF4ACFB13"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"Automotive software architecture"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐⭐"}</td>
                                            </tr>
                                            <tr>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"CRC-32/ISCSI"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; font-family: monospace;">{"0x1EDC6F41"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd;">{"iSCSI storage protocol"}</td>
                                                <td style="padding: 6px; border: 1px solid #ddd; text-align: center;">{"⭐⭐"}</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <h3>{"🎯 Algorithm Selection Guide"}</h3>
                            <div class="example-box">
                                <p><strong>{"Choose based on your application:"}</strong></p>
                                <ul>
                                    <li><strong>{"Web Development:"}</strong> {"CRC-32/ISO-HDLC (universal compatibility)"}</li>
                                    <li><strong>{"File Integrity:"}</strong> {"CRC-32/ISO-HDLC, CRC-64/XZ (for large files)"}</li>
                                    <li><strong>{"Network Protocols:"}</strong> {"CRC-16/MODBUS, CRC-32/ISO-HDLC"}</li>
                                    <li><strong>{"Embedded Systems:"}</strong> {"CRC-8/MAXIM-DOW, CRC-16/XMODEM"}</li>
                                    <li><strong>{"Industrial Automation:"}</strong> {"CRC-16/MODBUS, CRC-8/AUTOSAR"}</li>
                                    <li><strong>{"Legacy Compatibility:"}</strong> {"CRC-16/ARC, CRC-32/JAMCRC"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💼 Professional Use Cases & Real-World Applications"}</h2>
                            
                            <div class="use-case">
                                <h3>{"1. 🌐 Network Communications & Protocols"}</h3>
                                <h4>{"Ethernet Frame Check Sequence (FCS)"}</h4>
                                <p>{"Every Ethernet frame includes a 32-bit CRC-32/ISO-HDLC checksum to detect transmission errors. Network switches and routers automatically verify this checksum and discard corrupted frames."}</p>
                                <div class="example-box">
                                    <p><strong>{"Real Example:"}</strong></p>
                                    <ul>
                                        <li>{"Frame data: \"48656C6C6F\" (ASCII: Hello)"}</li>
                                        <li>{"CRC-32/ISO-HDLC result: 0x3610A686"}</li>
                                        <li>{"Application: Frame validation in network equipment"}</li>
                                    </ul>
                                </div>

                                <h4>{"MODBUS Industrial Communication"}</h4>
                                <p>{"MODBUS RTU uses CRC-16/MODBUS for error detection in industrial control systems. This ensures reliable communication between PLCs, sensors, and HMI systems."}</p>
                                <div class="example-box">
                                    <p><strong>{"Industrial Scenario:"}</strong></p>
                                    <ul>
                                        <li>{"Command: Read holding registers from device ID 1"}</li>
                                        <li>{"Data: \"01 03 00 00 00 0A\" (hex)"}</li>
                                        <li>{"CRC-16/MODBUS: Calculated and appended"}</li>
                                        <li>{"Result: Verified communication integrity"}</li>
                                    </ul>
                                </div>
                            </div>
                            
                            <div class="use-case">
                                <h3>{"2. 💾 Storage Systems & File Integrity"}</h3>
                                <h4>{"ZIP Archive Integrity"}</h4>
                                <p>{"ZIP files use CRC-32/ISO-HDLC to verify each compressed file's integrity. Archive managers check these values during extraction to detect corruption."}</p>
                                
                                <h4>{"Database Transaction Logs"}</h4>
                                <p>{"Modern databases use CRC checksums to ensure transaction log integrity. PostgreSQL, for example, uses CRC-32 for write-ahead log (WAL) files."}</p>
                                <div class="example-box">
                                    <p><strong>{"Database Protection:"}</strong></p>
                                    <ul>
                                        <li>{"Each log record includes CRC checksum"}</li>
                                        <li>{"Automatic corruption detection during recovery"}</li>
                                        <li>{"Prevents data loss from storage failures"}</li>
                                    </ul>
                                </div>
                            </div>
                            
                            <div class="use-case">
                                <h3>{"3. 🚗 Automotive & Embedded Systems"}</h3>
                                <h4>{"CAN Bus Communication"}</h4>
                                <p>{"Controller Area Network (CAN) uses CRC-15/CAN for frame validation in automotive systems. Critical for safety systems like ABS, airbags, and engine control."}</p>
                                
                                <h4>{"AUTOSAR Software Architecture"}</h4>
                                <p>{"Automotive software components use CRC-8/AUTOSAR and CRC-32/AUTOSAR for inter-module communication verification and memory protection."}</p>
                                <div class="example-box">
                                    <p><strong>{"Safety-Critical Application:"}</strong></p>
                                    <ul>
                                        <li>{"ECU firmware validation using CRC-32/AUTOSAR"}</li>
                                        <li>{"Real-time message integrity in brake systems"}</li>
                                        <li>{"Compliance with ISO 26262 functional safety"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="use-case">
                                <h3>{"4. 📡 Telecommunications & Wireless"}</h3>
                                <h4>{"Bluetooth Low Energy (BLE)"}</h4>
                                <p>{"BLE uses CRC-24/BLE for packet integrity in IoT devices, wearables, and smart home systems. Ensures reliable data transmission with minimal power consumption."}</p>
                                
                                <h4>{"GSM/UMTS Mobile Networks"}</h4>
                                <p>{"Mobile networks use various CRC algorithms (CRC-8/GSM-A, CRC-16/GSM, CRC-11/UMTS) for channel coding and error detection in voice and data transmission."}</p>
                            </div>

                            <div class="use-case">
                                <h3>{"5. 🔧 Development & Testing Scenarios"}</h3>
                                <h4>{"Protocol Development & Debugging"}</h4>
                                <ul>
                                    <li><strong>{"Frame Analysis:"}</strong> {"Verify protocol implementations by calculating expected CRC values"}</li>
                                    <li><strong>{"Test Vector Generation:"}</strong> {"Create test cases with known CRC results for unit testing"}</li>
                                    <li><strong>{"Interoperability Testing:"}</strong> {"Ensure different vendors' implementations produce identical CRC values"}</li>
                                    <li><strong>{"Legacy System Integration:"}</strong> {"Validate data exchange with older systems using specific CRC variants"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Comprehensive Step-by-Step Tutorials"}</h2>
                            
                            <div class="tutorial-step">
                                <h3>{"Tutorial 1: Network Protocol Implementation"}</h3>
                                <p><strong>{"Scenario:"}</strong> {"Implementing a custom protocol with CRC-16/MODBUS for industrial IoT sensors"}</p>
                                
                                <h4>{"Step 1: Define Your Protocol Frame"}</h4>
                                <ol>
                                    <li>{"Set Input Method to 'HEX'"}</li>
                                    <li>{"Enter frame data: \"01 03 00 00 00 0A\" (Device ID, Function, Address, Count)"}</li>
                                    <li>{"Select 'CRC-16/MODBUS' algorithm"}</li>
                                    <li>{"Choose 'Little Endian' for MODBUS compatibility"}</li>
                                    <li>{"Set Byte Format to 'Byte Separated' for clarity"}</li>
                                </ol>
                                
                                <h4>{"Step 2: Calculate and Verify"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Expected Results:"}</strong></p>
                                    <ul>
                                        <li>{"Input: \"01 03 00 00 00 0A\""}</li>
                                        <li>{"CRC-16/MODBUS: 0x2BA1 (Little Endian: 0xA1 0x2B)"}</li>
                                        <li>{"Complete Frame: \"01 03 00 00 00 0A A1 2B\""}</li>
                                    </ul>
                                </div>
                                
                                <h4>{"Step 3: Implementation in Code"}</h4>
                                <div class="example-box">
                                    <p><strong>{"C Implementation Pattern:"}</strong></p>
                                    <pre style="background-color: #f5f5f5; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 11px; overflow-x: auto;">
{r#"uint16_t calculate_modbus_crc(uint8_t *data, size_t length) {
    uint16_t crc = 0xFFFF;
    for (size_t i = 0; i < length; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            if (crc & 0x0001) {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc = crc >> 1;
            }
        }
    }
    return crc;
}"#}
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Tutorial 2: File Integrity Verification System"}</h3>
                                <p><strong>{"Scenario:"}</strong> {"Building a file checksum system using CRC-32/ISO-HDLC"}</p>
                                
                                <h4>{"Step 1: Test with Known Data"}</h4>
                                <ol>
                                    <li>{"Set Input Method to 'ASCII'"}</li>
                                    <li>{"Enter test string: \"The quick brown fox jumps over the lazy dog\""}</li>
                                    <li>{"Select 'CRC-32/ISO-HDLC' algorithm"}</li>
                                    <li>{"Set Output Format to 'HEX' with '0x' prefix"}</li>
                                    <li>{"Choose 'Big Endian' for standard file formats"}</li>
                                </ol>
                                
                                <h4>{"Step 2: Verify Standard Test Vector"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Standard Test Results:"}</strong></p>
                                    <ul>
                                        <li>{"Input: \"The quick brown fox jumps over the lazy dog\""}</li>
                                        <li>{"Expected CRC-32: 0x414FA339"}</li>
                                        <li>{"Use this as a reference for your implementation"}</li>
                                    </ul>
                                </div>
                                
                                <h4>{"Step 3: Implement File Processing"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Python Implementation Example:"}</strong></p>
                                    <pre style="background-color: #f5f5f5; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 11px; overflow-x: auto;">
{r#"import zlib

def calculate_file_crc32(filepath):
    """Calculate CRC-32/ISO-HDLC for file contents"""
    crc = 0
    with open(filepath, 'rb') as f:
        for chunk in iter(lambda: f.read(4096), b""):
            crc = zlib.crc32(chunk, crc)
    return crc & 0xffffffff  # Ensure positive 32-bit result

# Usage
file_crc = calculate_file_crc32('document.pdf')
print(f"File CRC-32: 0x{file_crc:08X}")
"#}
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Tutorial 3: Embedded System Sensor Validation"}</h3>
                                <p><strong>{"Scenario:"}</strong> {"Implementing CRC-8/MAXIM-DOW for Dallas 1-Wire temperature sensors"}</p>
                                
                                <h4>{"Step 1: Sensor Data Format"}</h4>
                                <ol>
                                    <li>{"Set Input Method to 'HEX'"}</li>
                                    <li>{"Enter sensor reading: \"50 05 4B 46 7F FF 0C 10\" (temperature data)"}</li>
                                    <li>{"Select 'CRC-8/MAXIM-DOW' algorithm"}</li>
                                    <li>{"Set Output Format to 'HEX' with '0x' prefix"}</li>
                                </ol>
                                
                                <h4>{"Step 2: Validate Sensor Communication"}</h4>
                                <div class="example-box">
                                    <p><strong>{"1-Wire Protocol Validation:"}</strong></p>
                                    <ul>
                                        <li>{"Sensor Data: \"50 05 4B 46 7F FF 0C 10\""}</li>
                                        <li>{"CRC-8/MAXIM-DOW: Calculate for first 8 bytes"}</li>
                                        <li>{"Expected: CRC should match the 9th byte"}</li>
                                        <li>{"Result: Validates sensor communication integrity"}</li>
                                    </ul>
                                </div>
                                
                                <h4>{"Step 3: Microcontroller Implementation"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Arduino/C++ Example:"}</strong></p>
                                    <pre style="background-color: #f5f5f5; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 11px; overflow-x: auto;">
{r#"uint8_t crc8_maxim_dow(uint8_t *data, size_t length) {
    uint8_t crc = 0;
    for (size_t i = 0; i < length; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            if (crc & 0x01) {
                crc = (crc >> 1) ^ 0x8C;
            } else {
                crc = crc >> 1;
            }
        }
    }
    return crc;
}

// Validate DS18B20 temperature sensor
bool validate_sensor_data(uint8_t *sensor_data) {
    uint8_t calculated_crc = crc8_maxim_dow(sensor_data, 8);
    return calculated_crc == sensor_data[8];
}"#}
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Tutorial 4: Custom Protocol Design"}</h3>
                                <p><strong>{"Scenario:"}</strong> {"Designing a robust communication protocol for drone telemetry"}</p>
                                
                                <h4>{"Step 1: Protocol Frame Design"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Frame Structure:"}</strong></p>
                                    <ul>
                                        <li>{"Header: 0xAA (sync byte)"}</li>
                                        <li>{"Length: Data payload length"}</li>
                                        <li>{"Type: Message type identifier"}</li>
                                        <li>{"Data: Variable length payload"}</li>
                                        <li>{"CRC: 16-bit checksum"}</li>
                                    </ul>
                                </div>
                                
                                <h4>{"Step 2: Algorithm Selection Process"}</h4>
                                <ol>
                                    <li>{"Consider requirements: Real-time, wireless, safety-critical"}</li>
                                    <li>{"Evaluate options: CRC-16/XMODEM vs CRC-16/MODBUS vs CRC-16/USB"}</li>
                                    <li>{"Test with our tool using sample telemetry data"}</li>
                                    <li>{"Input sample: \"AA 08 01 1A 2B 3C 4D 5E 6F\" (header + data)"}</li>
                                    <li>{"Compare CRC results for each algorithm"}</li>
                                    <li>{"Choose CRC-16/XMODEM for aviation compatibility"}</li>
                                </ol>
                                
                                <h4>{"Step 3: Performance Optimization"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Optimization Strategies:"}</strong></p>
                                    <ul>
                                        <li><strong>{"Table-based CRC:"}</strong> {"Pre-compute lookup tables for speed"}</li>
                                        <li><strong>{"Hardware CRC:"}</strong> {"Use built-in CRC units in modern MCUs"}</li>
                                        <li><strong>{"Streaming CRC:"}</strong> {"Calculate incrementally as data arrives"}</li>
                                        <li><strong>{"Error Recovery:"}</strong> {"Implement retransmission on CRC failure"}</li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Advanced Technical Deep Dive"}</h2>
                            
                            <h3>{"🧮 Mathematical Foundation"}</h3>
                            <p>{"CRC calculations are based on polynomial arithmetic in GF(2) - Galois Field with two elements. Understanding this foundation helps in debugging and optimizing implementations."}</p>
                            
                            <div class="example-box">
                                <p><strong>{"Key Mathematical Concepts:"}</strong></p>
                                <ul>
                                    <li><strong>{"Generator Polynomial:"}</strong> {"Defines the CRC algorithm characteristics (e.g., 0x1021 for CRC-16/XMODEM)"}</li>
                                    <li><strong>{"Polynomial Division:"}</strong> {"CRC is the remainder when data polynomial is divided by generator"}</li>
                                    <li><strong>{"Initial Value:"}</strong> {"Starting state of the CRC register (0x0000, 0xFFFF, etc.)"}</li>
                                    <li><strong>{"XOR Output:"}</strong> {"Final value XORed with result (often 0x0000 or 0xFFFF)"}</li>
                                    <li><strong>{"Bit Reflection:"}</strong> {"Input/output bit order (normal vs reflected)"}</li>
                                </ul>
                            </div>

                            <h3>{"⚡ Performance Considerations"}</h3>
                            <h4>{"Hardware vs Software Implementation"}</h4>
                            <div class="example-box">
                                <p><strong>{"Performance Comparison (1MB data):"}</strong></p>
                                <ul>
                                    <li><strong>{"Bit-by-bit Software:"}</strong> {"~50ms (basic implementation)"}</li>
                                    <li><strong>{"Table-based Software:"}</strong> {"~5ms (256-entry lookup table)"}</li>
                                    <li><strong>{"Hardware CRC Unit:"}</strong> {"~0.5ms (dedicated silicon)"}</li>
                                    <li><strong>{"SIMD Optimized:"}</strong> {"~1ms (vectorized operations)"}</li>
                                </ul>
                            </div>

                            <h3>{"🔍 Error Detection Capabilities"}</h3>
                            <p>{"Different CRC algorithms provide varying levels of error detection capability:"}</p>
                            
                            <div class="example-box">
                                <p><strong>{"Error Detection Guarantees:"}</strong></p>
                                <ul>
                                    <li><strong>{"Single-bit errors:"}</strong> {"100% detection for all CRC algorithms"}</li>
                                    <li><strong>{"Two-bit errors:"}</strong> {"100% detection if bits are within CRC width"}</li>
                                    <li><strong>{"Odd number of errors:"}</strong> {"100% detection if generator polynomial has (x+1) factor"}</li>
                                    <li><strong>{"Burst errors:"}</strong> {"100% detection for bursts ≤ CRC width"}</li>
                                    <li><strong>{"Random errors:"}</strong> {"(1 - 2^(-n)) probability for n-bit CRC"}</li>
                                </ul>
                            </div>

                            <h3>{"🛡️ Security Considerations"}</h3>
                            <p>{"While CRC is excellent for error detection, it's important to understand its limitations:"}</p>
                            
                            <div class="example-box">
                                <p><strong>{"Security Limitations:"}</strong></p>
                                <ul>
                                    <li><strong>{"Not Cryptographically Secure:"}</strong> {"Easily reversible with knowledge of algorithm"}</li>
                                    <li><strong>{"Intentional Modification:"}</strong> {"Attackers can modify data while preserving CRC"}</li>
                                    <li><strong>{"Predictable:"}</strong> {"Deterministic output for given input"}</li>
                                    <li><strong>{"Alternative for Security:"}</strong> {"Use cryptographic hashes (SHA-256) for tamper detection"}</li>
                                </ul>
                            </div>

                            <h3>{"🔄 Implementation Best Practices"}</h3>
                            <div class="example-box">
                                <p><strong>{"Professional Development Guidelines:"}</strong></p>
                                <ul>
                                    <li><strong>{"Test Vector Validation:"}</strong> {"Always validate implementation against known test vectors"}</li>
                                    <li><strong>{"Endianness Awareness:"}</strong> {"Document and test byte order for multi-byte CRCs"}</li>
                                    <li><strong>{"Algorithm Documentation:"}</strong> {"Clearly specify polynomial, initial value, and post-processing"}</li>
                                    <li><strong>{"Performance Profiling:"}</strong> {"Measure actual performance in target environment"}</li>
                                    <li><strong>{"Error Handling:"}</strong> {"Define behavior for CRC mismatches in your protocol"}</li>
                                    <li><strong>{"Version Control:"}</strong> {"Tag CRC parameters in protocol version management"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Comprehensive FAQ & Troubleshooting"}</h2>
                            
                            <div class="faq-item">
                                <h3>{"Q: Why do I get different CRC results from different tools?"}</h3>
                                <p>{"A: CRC algorithms with the same name can have different parameters (polynomial, initial value, XOR output, bit reflection). Our tool uses the standard Catalogue of CRC algorithms. Common variations include CRC-16 (multiple variants exist) and CRC-32 (IEEE 802.3 vs others)."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Which CRC algorithm should I choose for my application?"}</h3>
                                <p>{"A: Consider these factors: (1) Industry standards in your domain, (2) Required error detection capability, (3) Performance constraints, (4) Compatibility with existing systems. For general use: CRC-32/ISO-HDLC. For industrial: CRC-16/MODBUS. For embedded: CRC-8/MAXIM-DOW."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: How do I handle endianness in multi-byte CRCs?"}</h3>
                                <p>{"A: Endianness affects how multi-byte CRC values are transmitted/stored. Big Endian sends most significant byte first (network byte order), Little Endian sends least significant byte first (Intel x86). Our tool shows both formats - choose based on your protocol specification."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Can CRC detect all types of errors?"}</h3>
                                <p>{"A: CRC provides excellent error detection but not 100% coverage. It guarantees detection of: single-bit errors, double-bit errors (within CRC span), and burst errors up to CRC width. For random errors, detection probability is (1 - 2^(-n)) for n-bit CRC."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Why use CRC instead of simple checksums?"}</h3>
                                <p>{"A: CRC provides much better error detection than simple checksums. While checksums can miss many error patterns (like swapped bytes or systematic bit shifts), CRC uses polynomial mathematics to detect these patterns reliably."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: How do I optimize CRC calculation performance?"}</h3>
                                <p>{"A: Several optimization strategies: (1) Use lookup tables instead of bit-by-bit calculation, (2) Leverage hardware CRC units in modern processors, (3) Process data in larger chunks, (4) Use SIMD instructions for parallel processing, (5) Consider slice-by-8 or slice-by-16 algorithms for high throughput."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: What's the difference between CRC and cryptographic hashes?"}</h3>
                                <p>{"A: CRC is designed for error detection (accidental changes), while cryptographic hashes (SHA-256, etc.) are designed for security (intentional tampering). CRC is faster and simpler but not secure against malicious modification. Use CRC for integrity, cryptographic hashes for security."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: How do I validate my CRC implementation?"}</h3>
                                <p>{"A: Use standard test vectors: (1) Test with known inputs and verify expected outputs, (2) Use the string '123456789' as a common test case, (3) Verify against multiple reference implementations, (4) Test edge cases (empty data, single byte, maximum size), (5) Cross-validate with our tool's results."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Professional Tips & Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Algorithm Selection:"}</strong> {"Always check industry standards for your domain before choosing a CRC algorithm"}</li>
                                <li><strong>{"Test Vector Validation:"}</strong> {"Validate implementations using standard test vectors like 'Check: 0x2144DF1C' for CRC-32"}</li>
                                <li><strong>{"Performance Optimization:"}</strong> {"Use table-based lookup for production code - 256x faster than bit-by-bit calculation"}</li>
                                <li><strong>{"Documentation:"}</strong> {"Clearly specify polynomial, initial value, XOR output, and bit reflection in your specifications"}</li>
                                <li><strong>{"Cross-Platform Testing:"}</strong> {"Verify CRC calculations across different endianness systems"}</li>
                                <li><strong>{"Error Recovery:"}</strong> {"Implement appropriate error recovery strategies when CRC validation fails"}</li>
                                <li><strong>{"Version Management:"}</strong> {"Include CRC algorithm details in protocol version documentation"}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Remember that CRC is for error detection, not security - use cryptographic hashes for tamper detection"}</li>
                                <li><strong>{"Hardware Utilization:"}</strong> {"Leverage built-in CRC units in modern microcontrollers and processors"}</li>
                                <li><strong>{"Streaming Implementation:"}</strong> {"Design CRC calculation to work with streaming data for memory efficiency"}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools & Resources"}</h2>
                            <p>{"Enhance your data integrity workflow with these complementary tools:"}</p>
                            <ul>
                                <li><a href="/file-hash/">{"File Hash Generator"}</a> {" - Calculate MD5, SHA-1, SHA-256, SHA-512 for cryptographic integrity verification"}</li>
                                <li><a href="/base64/">{"Base64 Encoder/Decoder"}</a> {" - Encode binary CRC values for text-based protocols"}</li>
                                <li><a href="/ascii/">{"ASCII Converter"}</a> {" - Convert between text and numeric representations for CRC input preparation"}</li>
                                <li><a href="/base/">{"Number Base Converter"}</a> {" - Convert CRC values between decimal, hexadecimal, binary, and octal"}</li>
                            </ul>
                            
                            <h3>{"📚 External References"}</h3>
                            <ul>
                                <li><strong>{"Catalogue of CRC Algorithms:"}</strong> {" Comprehensive database of standardized CRC parameters"}</li>
                                <li><strong>{"RFC 3385:"}</strong> {" Internet Protocol CRC calculation considerations"}</li>
                                <li><strong>{"IEEE 802.3:"}</strong> {" Ethernet frame check sequence specification"}</li>
                                <li><strong>{"ISO 3309:"}</strong> {" Data communication CRC procedures"}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
                        // Input Settings Section
                        <div style="background-color: var(--color-third); padding: 10px; border-radius: 6px; margin-bottom: 10px;">
                            <h3 style="margin: 0 0 8px 0; color: var(--color-primary); font-size: 14px; border-bottom: 1px solid var(--color-primary); padding-bottom: 3px;">
                                <i class="fa-solid fa-arrow-right" style="margin-right: 6px; font-size: 12px;"></i>
                                {"Input Settings"}
                            </h3>
                            
                            <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Input Method: "}
                                </div>
                                <select
                                    id="input-mode-select"
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "ascii" => Msg::ModeChanged(InputMode::Ascii),
                                            "hex" => Msg::ModeChanged(InputMode::Hex),
                                            "binary" => Msg::ModeChanged(InputMode::Binary),
                                            "decimal" => Msg::ModeChanged(InputMode::Decimal),
                                            "octal" => Msg::ModeChanged(InputMode::Octal),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="ascii" selected={self.input_mode == InputMode::Ascii}>{ "ASCII" }</option>
                                    <option value="hex" selected={self.input_mode == InputMode::Hex}>{ "HEX" }</option>
                                    <option value="binary" selected={self.input_mode == InputMode::Binary}>{ "BINARY" }</option>
                                    <option value="decimal" selected={self.input_mode == InputMode::Decimal}>{ "DECIMAL" }</option>
                                    <option value="octal" selected={self.input_mode == InputMode::Octal}>{ "OCTAL" }</option>
                                </select>
                            </div>
                        </div>
                        
                        // Output Settings Section
                        <div style="background-color: var(--color-third); padding: 10px; border-radius: 6px; margin-bottom: 10px;">
                            <h3 style="margin: 0 0 8px 0; color: var(--color-secondary); font-size: 14px; border-bottom: 1px solid var(--color-secondary); padding-bottom: 3px;">
                                <i class="fa-solid fa-arrow-left" style="margin-right: 6px; font-size: 12px;"></i>
                                {"Output Settings"}
                            </h3>
                            
                            <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"CRC Algorithm: "}
                                </div>
                                <select
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={onchange_mode}>
                                        {CrcAlgorithm::all().into_iter().map(|algorithm| {
                                            let is_selected = algorithm == self.selected_algorithm;
                                            html! {
                                                <option
                                                    value={algorithm.name().to_string()}
                                                    selected={is_selected}
                                                >
                                                    { algorithm.name() }
                                                </option>
                                            }
                                        }).collect::<Html>()}
                                </select>
                            </div>
                            
                            <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Output Format: "}
                                </div>
                                <select
                                    id="output-mode-select"
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "decimal" => Msg::OutputModeChanged(OutputMode::Decimal),
                                            "hex" => Msg::OutputModeChanged(OutputMode::Hex),
                                            "binary" => Msg::OutputModeChanged(OutputMode::Binary),
                                            "octal" => Msg::OutputModeChanged(OutputMode::Octal),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="hex" selected={self.output_mode == OutputMode::Hex}>{ "HEX" }</option>
                                    <option value="decimal" selected={self.output_mode == OutputMode::Decimal}>{ "DECIMAL" }</option>
                                    <option value="binary" selected={self.output_mode == OutputMode::Binary}>{ "BINARY" }</option>
                                    <option value="octal" selected={self.output_mode == OutputMode::Octal}>{ "OCTAL" }</option>
                                </select>
                            </div>
                            
                            // 스타일 선택 드롭다운 (출력 모드별로 표시)
                            if self.output_mode == OutputMode::Hex {
                                <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                    <div style="width: 70%; font-size: 13px;">
                                        {"Hex Style: "}
                                    </div>
                                    <select
                                        style="width: 30%; padding: 2px; font-size: 12px;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::HexStyleChanged(HexStyle::WithPrefix),
                                                "short_prefix" => Msg::HexStyleChanged(HexStyle::ShortPrefix),
                                                "no_prefix" => Msg::HexStyleChanged(HexStyle::NoPrefix),
                                                "escape_sequence" => Msg::HexStyleChanged(HexStyle::EscapeSequence),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.hex_style == HexStyle::WithPrefix}>{ "0x48" }</option>
                                        <option value="short_prefix" selected={self.hex_style == HexStyle::ShortPrefix}>{ "x48" }</option>
                                        <option value="no_prefix" selected={self.hex_style == HexStyle::NoPrefix}>{ "48" }</option>
                                        <option value="escape_sequence" selected={self.hex_style == HexStyle::EscapeSequence}>{ "\\x48" }</option>
                                    </select>
                                </div>
                            }
                            
                            if self.output_mode == OutputMode::Binary {
                                <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                    <div style="width: 70%; font-size: 13px;">
                                        {"Binary Style: "}
                                    </div>
                                    <select
                                        style="width: 30%; padding: 2px; font-size: 12px;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::BinaryStyleChanged(BinaryStyle::WithPrefix),
                                                "short_prefix" => Msg::BinaryStyleChanged(BinaryStyle::ShortPrefix),
                                                "no_prefix" => Msg::BinaryStyleChanged(BinaryStyle::NoPrefix),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.binary_style == BinaryStyle::WithPrefix}>{ "0b01000001" }</option>
                                        <option value="short_prefix" selected={self.binary_style == BinaryStyle::ShortPrefix}>{ "b01000001" }</option>
                                        <option value="no_prefix" selected={self.binary_style == BinaryStyle::NoPrefix}>{ "01000001" }</option>
                                    </select>
                                </div>
                            }
                            
                            if self.output_mode == OutputMode::Octal {
                                <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                    <div style="width: 70%; font-size: 13px;">
                                        {"Octal Style: "}
                                    </div>
                                    <select
                                        style="width: 30%; padding: 2px; font-size: 12px;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::OctalStyleChanged(OctalStyle::WithPrefix),
                                                "short_prefix" => Msg::OctalStyleChanged(OctalStyle::ShortPrefix),
                                                "no_prefix" => Msg::OctalStyleChanged(OctalStyle::NoPrefix),
                                                "escape_sequence" => Msg::OctalStyleChanged(OctalStyle::EscapeSequence),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.octal_style == OctalStyle::WithPrefix}>{ "0o101" }</option>
                                        <option value="short_prefix" selected={self.octal_style == OctalStyle::ShortPrefix}>{ "o101" }</option>
                                        <option value="no_prefix" selected={self.octal_style == OctalStyle::NoPrefix}>{ "101" }</option>
                                        <option value="escape_sequence" selected={self.octal_style == OctalStyle::EscapeSequence}>{ "\\101" }</option>
                                    </select>
                                </div>
                            }

                            <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Byte Order: "}
                                </div>
                                <select
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "big_endian" => Msg::EndiannessChanged(Endianness::BigEndian),
                                            "little_endian" => Msg::EndiannessChanged(Endianness::LittleEndian),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="big_endian" selected={self.endianness == Endianness::BigEndian}>{ "Big Endian" }</option>
                                    <option value="little_endian" selected={self.endianness == Endianness::LittleEndian}>{ "Little Endian" }</option>
                                </select>
                            </div>
                            
                            <div style="display: flex; align-items: center;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Byte Format: "}
                                </div>
                                <select
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "continuous" => Msg::ByteFormattingChanged(ByteFormatting::Continuous),
                                            "byte_separated" => Msg::ByteFormattingChanged(ByteFormatting::ByteSeparated),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="continuous" selected={self.byte_formatting == ByteFormatting::Continuous}>{ "Continuous" }</option>
                                    <option value="byte_separated" selected={self.byte_formatting == ByteFormatting::ByteSeparated}>{ "Byte Separated" }</option>
                                </select>
                            </div>
                        </div>

                        <div class="tool-inner">
                            <div>
                                <div class="tool-subtitle" style="margin-bottom: 3px; font-size: 14px;">{ "Input" }</div>
                                <textarea
                                    type="text"
                                    style={format!("{}; height: 80px; resize: vertical;", if self.error_message.is_some() { 
                                        "overflow: auto; border: 2px solid var(--color-error)" 
                                    } else { 
                                        "overflow: auto" 
                                    })}
                                    value={self.input.clone()}
                                    oninput={link.callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::InputChanged(input.value())
                                    })}
                                    placeholder={
                                        match self.input_mode {
                                            InputMode::Ascii => "Enter ASCII text...",
                                            InputMode::Hex => "Enter HEX values (e.g., 0x01 \\x02 x03 04 05 or 0x01\\x02x030405)...",
                                            InputMode::Binary => "Enter BINARY values (e.g., 0b01001000 b01100101 01101100)...",
                                            InputMode::Decimal => "Enter DECIMAL values (e.g., 72 101 108 108 111)...",
                                            InputMode::Octal => "Enter OCTAL values (e.g., 0o110 o145 \\154)...",
                                        }
                                    }
                                />
                                if let Some(error_msg) = &self.error_message {
                                    <div style="color: var(--color-error); font-size: 11px; margin-top: 2px; line-height: 1.2;">
                                        { error_msg }
                                    </div>
                                }
                                <div style="color: var(--color-subfont); font-size: 10px; margin-top: 2px;">
                                    { match self.input_mode {
                                        InputMode::Ascii => "Any text characters are supported",
                                        InputMode::Hex => "Supports: 0x01, \\x02, x03, 04, 05 formats and combinations",
                                        InputMode::Binary => "Supports: 0b01000001, b01000001, 01000001 formats",
                                        InputMode::Decimal => "Valid range: 0-255 (space-separated numbers)",
                                        InputMode::Octal => "Supports: 0o101, o101, 101, \\101 formats",
                                    }}
                                </div>
                            </div>
                            <div>
                                <div class="tool-subtitle" style="margin-top: 10px; font-size: 14px;">{ "Processed data" }</div>
                                <textarea
                                    type="text"
                                    readonly=true
                                    style="cursor: pointer; height: 50px; resize: vertical;"
                                    value={self.bytes_string.clone()}
                                    onclick={_ctx.link().callback(|e: MouseEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::CopyToClipboard(input.value())
                                    })} />
                            </div>
                        </div>
                        <div class="tool-inner" style="margin-top: 8px;">
                            <div>
                                <div class="tool-subtitle" style="font-size: 14px;">{ format!("{} Result", self.selected_algorithm.name()) }</div>
                                <input
                                    type="text"
                                    name="crc"
                                    readonly=true
                                    style="cursor: pointer; height: 40px; font-weight: bold;"
                                    value={self.format_crc_output()}
                                    onclick={_ctx.link().callback(|e: MouseEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::CopyToClipboard(input.value())
                                    })} />
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("CRC Tool - Online CRC Calculator & Decoder | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "Free online CRC tool and calculator. Calculate CRC checksums with 100+ algorithms including CRC-32, CRC-16, CRC-8. CRC decoder and encoder for data integrity verification. Supports ASCII, HEX, Binary input formats. Professional CRC calculator for developers, engineers, and network protocols.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolCrc {
    fn parse_hex_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // "0x" 또는 "\x" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'x' || next == 'X' {
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'x' 건너뛰기
                            current_number = self.collect_hex_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            current_number.push('0');
                        }
                    } else {
                        current_number.push('0');
                    }
                }
                '\\' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'x' || next == 'X' {
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'x' 건너뛰기
                            current_number = self.collect_hex_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            return Err("Invalid escape sequence: expected 'x' after '\\'".to_string());
                        }
                    } else {
                        return Err("Incomplete escape sequence: unexpected end of input after '\\'".to_string());
                    }
                }
                'x' | 'X' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'x' 건너뛰기
                    current_number = self.collect_hex_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 16진수 숫자 수집
                _ => {
                    if c.is_ascii_hexdigit() {
                        current_number.push(chars.next().unwrap());
                    } else {
                        return Err(format!("Invalid character '{}' in hexadecimal input. Only 0-9, A-F, a-f are allowed.", c));
                    }

                    // 두 자리가 모이면 바이트로 변환
                    if current_number.len() == 2 {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            // 한 자리 숫자인 경우 앞에 0을 붙임
            if current_number.len() == 1 {
                current_number.insert(0, '0');
            }
            result.push(self.parse_hex_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid hexadecimal values found in input".to_string());
        }

        Ok(result)
    }

    fn collect_hex_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut hex_str = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_ascii_hexdigit() {
                hex_str.push(chars.next().unwrap());
                if hex_str.len() == 2 {
                    break;
                }
            } else {
                break;
            }
        }

        if hex_str.is_empty() {
            return Err("Expected hexadecimal digits after prefix".to_string());
        }

        // 한 자리 숫자인 경우 앞에 0을 붙임
        if hex_str.len() == 1 {
            hex_str.insert(0, '0');
        }

        Ok(hex_str)
    }

    fn parse_hex_string(&self, hex_str: &str) -> Result<u8, String> {
        u8::from_str_radix(hex_str, 16).map_err(|_| format!("Invalid hexadecimal value: '{}'", hex_str))
    }

    fn calculate_crc(&mut self) {
        if self.bytes.is_empty() {
            self.crc_result = 0;
            return;
        }

        (self.crc_result, self.width) = self.selected_algorithm.calculate(&self.bytes);
    }

    fn format_crc_output(&self) -> String {
        // 엔디안에 따라 바이트 순서 조정
        let crc_value = match self.endianness {
            Endianness::BigEndian => self.crc_result,
            Endianness::LittleEndian => {
                match self.width {
                    8 => self.crc_result, // 8비트는 엔디안 무관
                    16 => ((self.crc_result & 0xFF) << 8) | ((self.crc_result >> 8) & 0xFF),
                    24 => {
                        let b0 = (self.crc_result >> 16) & 0xFF;
                        let b1 = (self.crc_result >> 8) & 0xFF;
                        let b2 = self.crc_result & 0xFF;
                        (b2 << 16) | (b1 << 8) | b0
                    },
                    32 => {
                        let b0 = (self.crc_result >> 24) & 0xFF;
                        let b1 = (self.crc_result >> 16) & 0xFF;
                        let b2 = (self.crc_result >> 8) & 0xFF;
                        let b3 = self.crc_result & 0xFF;
                        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
                    },
                    64 => {
                        let mut result = 0u64;
                        for i in 0..8 {
                            let byte = (self.crc_result >> (i * 8)) & 0xFF;
                            result |= byte << ((7 - i) * 8);
                        }
                        result
                    },
                    _ => {
                        // 다른 비트 크기에 대한 일반적인 처리
                        let bytes = ((self.width + 7) / 8) as usize;
                        let mut result = 0u64;
                        for i in 0..bytes {
                            let byte = (self.crc_result >> (i * 8)) & 0xFF;
                            result |= byte << (((bytes - 1 - i) as u64) * 8);
                        }
                        result
                    }
                }
            }
        };

        match self.output_mode {
            OutputMode::Decimal => {
                format!("{}", crc_value)
            }
            OutputMode::Hex => {
                self.format_hex_output(crc_value)
            }
            OutputMode::Binary => {
                self.format_binary_output(crc_value)
            }
            OutputMode::Octal => {
                self.format_octal_output(crc_value)
            }
        }
    }

    fn format_hex_output(&self, value: u64) -> String {
        let bytes = ((self.width + 7) / 8) as usize;
        
        match self.byte_formatting {
            ByteFormatting::Continuous => {
                let digits = bytes * 2;
                match self.hex_style {
                    HexStyle::WithPrefix => format!("0x{:0width$X}", value, width = digits),
                    HexStyle::ShortPrefix => format!("x{:0width$X}", value, width = digits),
                    HexStyle::NoPrefix => format!("{:0width$X}", value, width = digits),
                    HexStyle::EscapeSequence => format!("\\x{:0width$X}", value, width = digits),
                }
            }
            ByteFormatting::ByteSeparated => {
                let mut byte_strings = Vec::new();
                for i in 0..bytes {
                    let byte = (value >> ((bytes - 1 - i) * 8)) & 0xFF;
                    let byte_str = match self.hex_style {
                        HexStyle::WithPrefix => format!("0x{:02X}", byte),
                        HexStyle::ShortPrefix => format!("x{:02X}", byte),
                        HexStyle::NoPrefix => format!("{:02X}", byte),
                        HexStyle::EscapeSequence => format!("\\x{:02X}", byte),
                    };
                    byte_strings.push(byte_str);
                }
                byte_strings.join(" ")
            }
        }
    }

    fn format_binary_output(&self, value: u64) -> String {
        match self.byte_formatting {
            ByteFormatting::Continuous => {
                let binary_str = format!("{:0width$b}", value, width = self.width as usize);
                match self.binary_style {
                    BinaryStyle::WithPrefix => format!("0b{}", binary_str),
                    BinaryStyle::ShortPrefix => format!("b{}", binary_str),
                    BinaryStyle::NoPrefix => binary_str,
                }
            }
            ByteFormatting::ByteSeparated => {
                let bytes = ((self.width + 7) / 8) as usize;
                let mut byte_strings = Vec::new();
                for i in 0..bytes {
                    let byte = (value >> ((bytes - 1 - i) * 8)) & 0xFF;
                    let byte_str = match self.binary_style {
                        BinaryStyle::WithPrefix => format!("0b{:08b}", byte),
                        BinaryStyle::ShortPrefix => format!("b{:08b}", byte),
                        BinaryStyle::NoPrefix => format!("{:08b}", byte),
                    };
                    byte_strings.push(byte_str);
                }
                byte_strings.join(" ")
            }
        }
    }

    fn format_octal_output(&self, value: u64) -> String {
        let bytes = ((self.width + 7) / 8) as usize;
        
        match self.byte_formatting {
            ByteFormatting::Continuous => {
                let digits = ((self.width as f32 / 3.0).ceil()) as usize;
                match self.octal_style {
                    OctalStyle::WithPrefix => format!("0o{:0width$o}", value, width = digits),
                    OctalStyle::ShortPrefix => format!("o{:0width$o}", value, width = digits),
                    OctalStyle::NoPrefix => format!("{:0width$o}", value, width = digits),
                    OctalStyle::EscapeSequence => format!("\\{:0width$o}", value, width = digits),
                }
            }
            ByteFormatting::ByteSeparated => {
                let mut byte_strings = Vec::new();
                for i in 0..bytes {
                    let byte = (value >> ((bytes - 1 - i) * 8)) & 0xFF;
                    let byte_str = match self.octal_style {
                        OctalStyle::WithPrefix => format!("0o{:03o}", byte),
                        OctalStyle::ShortPrefix => format!("o{:03o}", byte),
                        OctalStyle::NoPrefix => format!("{:03o}", byte),
                        OctalStyle::EscapeSequence => format!("\\{:03o}", byte),
                    };
                    byte_strings.push(byte_str);
                }
                byte_strings.join(" ")
            }
        }
    }

    fn parse_binary_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // "0b" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'b' || next == 'B' {
                            if !current_number.is_empty() {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'b' 건너뛰기
                            current_number = self.collect_binary_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            // 일반 '0' 이진수 숫자
                            current_number.push('0');
                            if current_number.len() == 8 {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                        }
                    } else {
                        // 마지막 문자가 '0'인 경우
                        current_number.push('0');
                        if current_number.len() == 8 {
                            result.push(self.parse_binary_string(&current_number)?);
                            current_number.clear();
                        }
                    }
                }
                'b' | 'B' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'b' 건너뛰기
                    current_number = self.collect_binary_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 이진수 숫자 수집
                '1' => {
                    current_number.push(chars.next().unwrap());
                    // 8비트가 모이면 바이트로 변환
                    if current_number.len() == 8 {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                }
                _ => {
                    return Err(format!("Invalid character '{}' in binary input. Only 0 and 1 are allowed.", c));
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            if current_number.len() > 8 {
                return Err(format!("Binary sequence '{}' is longer than 8 bits", current_number));
            }
            result.push(self.parse_binary_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid binary values found in input".to_string());
        }

        Ok(result)
    }

    fn collect_binary_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut binary_str = String::new();

        while let Some(&c) = chars.peek() {
            if c == '0' || c == '1' {
                binary_str.push(chars.next().unwrap());
                if binary_str.len() == 8 {
                    break;
                }
            } else {
                break;
            }
        }

        if binary_str.is_empty() {
            return Err("Expected binary digits after prefix".to_string());
        }

        Ok(binary_str)
    }

    fn parse_binary_string(&self, binary_str: &str) -> Result<u8, String> {
        if binary_str.len() > 8 {
            return Err(format!("Binary sequence '{}' exceeds 8 bits", binary_str));
        }
        u8::from_str_radix(binary_str, 2).map_err(|_| format!("Invalid binary value: '{}'", binary_str))
    }

    fn parse_decimal_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        input
            .split_whitespace()
            .map(|s| {
                s.parse::<u16>()
                    .map_err(|_| format!("Invalid decimal number: '{}'", s))
                    .and_then(|num| {
                        if num > 255 {
                            Err(format!("Decimal value {} exceeds maximum range (0-255)", num))
                        } else {
                            Ok(num as u8)
                        }
                    })
            })
            .collect()
    }

    fn parse_octal_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // "\NNN" escape sequence 처리
                '\\' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // '\' 건너뛰기
                    current_number = self.collect_octal_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // "0o" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'o' || next == 'O' {
                            if !current_number.is_empty() {
                                result.push(self.parse_octal_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'o' 건너뛰기
                            current_number = self.collect_octal_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_octal_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            current_number.push('0');
                        }
                    } else {
                        current_number.push('0');
                    }
                }
                'o' | 'O' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'o' 건너뛰기
                    current_number = self.collect_octal_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 8진수 숫자 수집
                '0'..='7' => {
                    current_number.push(chars.next().unwrap());
                    // 3자리가 모이면 바이트로 변환 (8진수 377 = 255)
                    if current_number.len() == 3 {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                _ => {
                    return Err(format!("Invalid character '{}' in octal input. Only 0-7 are allowed.", c));
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            if current_number.len() > 3 {
                return Err(format!("Octal sequence '{}' is longer than 3 digits", current_number));
            }
            result.push(self.parse_octal_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid octal values found in input".to_string());
        }

        Ok(result)
    }

    fn collect_octal_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut octal_str = String::new();

        while let Some(&c) = chars.peek() {
            if c >= '0' && c <= '7' {
                octal_str.push(chars.next().unwrap());
                if octal_str.len() == 3 {
                    break;
                }
            } else {
                break;
            }
        }

        if octal_str.is_empty() {
            return Err("Expected octal digits after prefix".to_string());
        }

        Ok(octal_str)
    }

    fn parse_octal_string(&self, octal_str: &str) -> Result<u8, String> {
        if octal_str.len() > 3 {
            return Err(format!("Octal sequence '{}' exceeds 3 digits", octal_str));
        }
        u8::from_str_radix(octal_str, 8).map_err(|_| format!("Invalid octal value: '{}'", octal_str))
    }

    // Local Storage 키 상수들
    const STORAGE_KEY_INPUT_MODE: &'static str = "crc_input_mode";
    const STORAGE_KEY_OUTPUT_MODE: &'static str = "crc_output_mode";
    const STORAGE_KEY_HEX_STYLE: &'static str = "crc_hex_style";
    const STORAGE_KEY_BINARY_STYLE: &'static str = "crc_binary_style";
    const STORAGE_KEY_OCTAL_STYLE: &'static str = "crc_octal_style";
    const STORAGE_KEY_ENDIANNESS: &'static str = "crc_endianness";
    const STORAGE_KEY_BYTE_FORMATTING: &'static str = "crc_byte_formatting";
    const STORAGE_KEY_CRC_ALGORITHM: &'static str = "crc_algorithm";

    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    fn load_from_storage() -> Self {
        let storage = Self::get_local_storage();
        
        let input_mode = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_INPUT_MODE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "ascii" => Some(InputMode::Ascii),
                "hex" => Some(InputMode::Hex),
                "binary" => Some(InputMode::Binary),
                "decimal" => Some(InputMode::Decimal),
                "octal" => Some(InputMode::Octal),
                _ => None,
            })
            .unwrap_or(InputMode::Ascii);

        let output_mode = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_OUTPUT_MODE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "decimal" => Some(OutputMode::Decimal),
                "hex" => Some(OutputMode::Hex),
                "binary" => Some(OutputMode::Binary),
                "octal" => Some(OutputMode::Octal),
                _ => None,
            })
            .unwrap_or(OutputMode::Hex);

        let hex_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_HEX_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(HexStyle::WithPrefix),
                "short_prefix" => Some(HexStyle::ShortPrefix),
                "no_prefix" => Some(HexStyle::NoPrefix),
                "escape_sequence" => Some(HexStyle::EscapeSequence),
                _ => None,
            })
            .unwrap_or(HexStyle::WithPrefix);

        let binary_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_BINARY_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(BinaryStyle::WithPrefix),
                "short_prefix" => Some(BinaryStyle::ShortPrefix),
                "no_prefix" => Some(BinaryStyle::NoPrefix),
                _ => None,
            })
            .unwrap_or(BinaryStyle::WithPrefix);

        let octal_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_OCTAL_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(OctalStyle::WithPrefix),
                "short_prefix" => Some(OctalStyle::ShortPrefix),
                "no_prefix" => Some(OctalStyle::NoPrefix),
                "escape_sequence" => Some(OctalStyle::EscapeSequence),
                _ => None,
            })
            .unwrap_or(OctalStyle::WithPrefix);

        let endianness = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_ENDIANNESS).ok().flatten())
            .and_then(|s| match s.as_str() {
                "big_endian" => Some(Endianness::BigEndian),
                "little_endian" => Some(Endianness::LittleEndian),
                _ => None,
            })
            .unwrap_or(Endianness::BigEndian);

        let byte_formatting = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_BYTE_FORMATTING).ok().flatten())
            .and_then(|s| match s.as_str() {
                "continuous" => Some(ByteFormatting::Continuous),
                "byte_separated" => Some(ByteFormatting::ByteSeparated),
                _ => None,
            })
            .unwrap_or(ByteFormatting::Continuous);

        let selected_algorithm = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_CRC_ALGORITHM).ok().flatten())
            .and_then(|s| CrcAlgorithm::from_name(&s))
            .unwrap_or(CrcAlgorithm::Crc32IsoHdlc);

        Self {
            input: String::new(),
            input_mode,
            output_mode,
            hex_style,
            binary_style,
            octal_style,
            endianness,
            byte_formatting,
            selected_algorithm,
            bytes: Vec::new(),
            bytes_string: String::new(),
            crc_result: 0,
            error_message: None,
            width: 32,
        }
    }

    fn save_to_storage(&self) {
        if let Some(storage) = Self::get_local_storage() {
            let input_mode_str = match self.input_mode {
                InputMode::Ascii => "ascii",
                InputMode::Hex => "hex",
                InputMode::Binary => "binary",
                InputMode::Decimal => "decimal",
                InputMode::Octal => "octal",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_INPUT_MODE, input_mode_str);

            let output_mode_str = match self.output_mode {
                OutputMode::Decimal => "decimal",
                OutputMode::Hex => "hex",
                OutputMode::Binary => "binary",
                OutputMode::Octal => "octal",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_OUTPUT_MODE, output_mode_str);

            let hex_style_str = match self.hex_style {
                HexStyle::WithPrefix => "with_prefix",
                HexStyle::ShortPrefix => "short_prefix",
                HexStyle::NoPrefix => "no_prefix",
                HexStyle::EscapeSequence => "escape_sequence",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_HEX_STYLE, hex_style_str);

            let binary_style_str = match self.binary_style {
                BinaryStyle::WithPrefix => "with_prefix",
                BinaryStyle::ShortPrefix => "short_prefix",
                BinaryStyle::NoPrefix => "no_prefix",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_BINARY_STYLE, binary_style_str);

            let octal_style_str = match self.octal_style {
                OctalStyle::WithPrefix => "with_prefix",
                OctalStyle::ShortPrefix => "short_prefix",
                OctalStyle::NoPrefix => "no_prefix",
                OctalStyle::EscapeSequence => "escape_sequence",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_OCTAL_STYLE, octal_style_str);

            let endianness_str = match self.endianness {
                Endianness::BigEndian => "big_endian",
                Endianness::LittleEndian => "little_endian",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_ENDIANNESS, endianness_str);

            let byte_formatting_str = match self.byte_formatting {
                ByteFormatting::Continuous => "continuous",
                ByteFormatting::ByteSeparated => "byte_separated",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_BYTE_FORMATTING, byte_formatting_str);

            let _ = storage.set_item(Self::STORAGE_KEY_CRC_ALGORITHM, self.selected_algorithm.name());
        }
    }
}
