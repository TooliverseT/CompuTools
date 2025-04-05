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
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum InputMode {
    Ascii,
    Hex,
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
    selected_algorithm: CrcAlgorithm,
    bytes: Vec<u8>,
    bytes_string: String,
    crc_result: u64,
    error_message: String,
    width: u8,
}

pub enum Msg {
    InputChanged(String),
    ModeChanged(InputMode),
    SelectAlgorithm(String),
    CopyToClipboard(String),
    Calculate,
}

impl Component for ToolCrc {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Ascii,
            selected_algorithm: CrcAlgorithm::Crc32IsoHdlc,
            bytes: Vec::new(),
            bytes_string: String::new(),
            crc_result: 0,
            error_message: String::new(),
            width: 32,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(value) => {
                self.input = value;
                self.error_message = String::new();
                self.calculate_crc();
                self.bytes_string = self
                    .bytes
                    .iter()
                    .map(|byte| format!("0x{:02X}", byte))
                    .collect::<Vec<String>>()
                    .join(" ");
                true
            }
            Msg::ModeChanged(mode) => {
                self.input_mode = mode;
                self.error_message = String::new();
                let cb = _ctx.link().callback(|_: u8| Msg::Calculate);
                cb.emit(0);
                true
            }
            Msg::SelectAlgorithm(value) => {
                if let Some(algorithm) = CrcAlgorithm::from_name(&value) {
                    self.selected_algorithm = algorithm;
                }
                let cb = _ctx.link().callback(|_: u8| Msg::Calculate);
                cb.emit(0);
                true
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
                // input_ref에서 HtmlInputElement를 가져옴
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    // 클립보드 작업 수행
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
                false // 리렌더링 필요 없음
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();

        let onchange_mode = _ctx.link().callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            Msg::SelectAlgorithm(select.value()) // 이 부분을 적절히 처리해야 합니다
        });

        let digits = ((self.width as f32 / 4.0).ceil()) as usize; // 4비트 = 1자리
        let formatted_crc = format!("0x{:0width$X}", self.crc_result, width = digits);

        html! {
            <>
                <div class="tool-wrapper">
                    <div>
                        <h1 class="tool-title">
                            { "CRC Converter" }
                        </h1>
                        <div class="tool-intro">
                            <p>
                                {"This page provides a simple tool for calculating CRC (Cyclic Redundancy Check) values to verify data integrity and detect errors. CRC is widely used in networking, storage, and embedded systems."}
                            </p>
                            <p> {"With this tool, you can:"} </p>
                            <ul>
                                <li>{"Compute CRC values for input data using customizable settings."}</li>
                                <li>{"Verify data integrity by comparing recalculated CRC values."}</li>
                            </ul>
                            <p>
                                {"The tool supports adjustable parameters like polynomial, initial value, XOR out, and reflection options, ensuring compatibility with various CRC standards."}
                            </p>
                            <p>
                                {"Hexadecimal input formats supported include:"}
                            </p>
                            <ul>
                                <li>{"0x01 \\x02 x03 04 05"}</li>
                                <li>{"0x01\\x02x030405"}</li>
                            </ul>
                            <p>
                                {"Note:"}
                            </p>
                            <ul>
                                <li>{"Provide input data in text or hexadecimal format."}</li>
                                <li>{"Ensure parameter settings match the CRC algorithm used in your application."}</li>
                            </ul>

                        </div>
                    </div>
                    <div class="tool-container">
                        <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 90%;">
                                {"Input Method: "}
                            </div>
                            <select
                                id="input-mode-select"
                                onchange={_ctx.link().callback(|e: Event| {
                                    let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                    match value.as_str() {
                                        "ascii" => Msg::ModeChanged(InputMode::Ascii),
                                        "hex" => Msg::ModeChanged(InputMode::Hex),
                                        _ => unreachable!(),
                                    }
                                })}>
                                <option value="ascii" selected={self.input_mode == InputMode::Ascii}>{ "ASCII" }</option>
                                <option value="hex" selected={self.input_mode == InputMode::Hex}>{ "HEX" }</option>
                            </select>
                        </div>
                        <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 90%;">
                                {"CRC algorithm: "}
                            </div>
                            <select
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
                        <div class="tool-inner">
                            <div>
                                <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Input" }</div>
                                <textarea
                                    type="text"
                                    value={self.input.clone()}
                                    oninput={link.callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::InputChanged(input.value())
                                    })}
                                    placeholder={
                                        if self.input_mode == InputMode::Ascii {
                                            "Enter ASCII text..."
                                        } else {
                                            "Enter HEX values (e.g., 0x01 \\x02 x03 04 05 or 0x01\\x02x030405)..."
                                        }
                                    }
                                />
                            </div>
                            <div>
                                <div class="tool-subtitle" style="margin-top: 15px;">{ "Processed data" }</div>
                                <textarea
                                    type="text"
                                    readonly=true
                                    style="cursor: pointer;"
                                    value={self.bytes_string.clone()}
                                    onclick={_ctx.link().callback(|e: MouseEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::CopyToClipboard(input.value())
                                    })} />
                            </div>
                        </div>
                        <div class="tool-inner" style="margin-top: 10px;">
                            <div>
                                <div class="tool-subtitle">{ format!("{} Result", self.selected_algorithm.name()) }</div>
                                <input
                                    type="text"
                                    name="crc"
                                    readonly=true
                                    style="cursor: pointer;"
                                    value={formatted_crc}
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
                    doc.set_title("CRC Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This page provides a simple and efficient tool for calculating CRC (Cyclic Redundancy Check) values for data integrity verification and error detection. CRC is widely used in networking, storage, and embedded systems. crc3, crc4, crc5, crc6, crc7, crc8, crc10, crc11, crc12, crc13, crc14, crc15, crc16, crc17, crc21, crc24, crc30, crc31,crc32, crc40, crc64. CTC Tool").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolCrc {
    fn parse_hex_input(&self, input: &str) -> Result<Vec<u8>, String> {
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
                            return Err("Invalid hex format: expected 'x' after '\\'".to_string());
                        }
                    } else {
                        return Err("Unexpected end of input after '\\'".to_string());
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
                        chars.next(); // 무시할 문자 건너뛰기
                        continue;
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
            return Err("No valid hex values found".to_string());
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
            return Err("Expected hex digits".to_string());
        }

        // 한 자리 숫자인 경우 앞에 0을 붙임
        if hex_str.len() == 1 {
            hex_str.insert(0, '0');
        }

        Ok(hex_str)
    }

    fn parse_hex_string(&self, hex_str: &str) -> Result<u8, String> {
        u8::from_str_radix(hex_str, 16).map_err(|_| format!("Invalid hex value: {}", hex_str))
    }

    fn calculate_crc(&mut self) {
        self.crc_result = 0;

        let input_bytes = match self.input_mode {
            InputMode::Ascii => Ok(self.input.as_bytes().to_vec()),
            InputMode::Hex => self.parse_hex_input(&self.input),
        };
        match input_bytes {
            Ok(bytes) => {
                self.bytes = bytes;
                (self.crc_result, self.width) = self.selected_algorithm.calculate(&self.bytes);
            }
            Err(e) => {
                self.error_message = format!("Error: {}", e);
            }
        }
    }
}
