//! ******************************************************************************
//! @file    : version.rs
//! @brief   : Firmware version (Semantic Versioning 2.0 + suffix)
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *********************   VERSION CONTROL INFORMATION   ************************
//!
//!  major  : 0 pre-production, 1 released, 2+ incompatible future versions
//!  minor  : compatible additions
//!  patch  : bug fixes
//!  suffix : 0 production, 1 alpha, 2 beta, 3 release candidate
//!
//!  Change with every push and add a CHANGELOG.md entry.
//!
//! ******************************************************************************

/// @brief  Version layout. `repr(C)` keeps the 4 bytes in this order so the same
///         struct can later be sent to a host as raw bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Version
{
    pub major  : u8,
    pub minor  : u8,
    pub patch  : u8,
    pub suffix : u8,
}



/// @brief  Version Object
pub const VERSION: Version = Version
{
    major  : 0,
    minor  : 1,
    patch  : 0,
    suffix : 1,     /* alpha */
};

/********************************* END OF FILE *********************************/
