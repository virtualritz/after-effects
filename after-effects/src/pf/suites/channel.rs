
use crate::*;
use ae_sys::*;

define_suite!(
    /// Some file types contain more than just pixel data; use [`ChannelSuite`] to determine whether such information is present,
    ChannelSuite,
    PF_ChannelSuite1,
    kPFChannelSuite1,
    kPFChannelSuiteVersion1
);

impl ChannelSuite {
    /// Acquire this suite from the host. Returns error if the suite is not available.
    /// Suite is released on drop.
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }

    /// Retrieves the number of auxiliary channels associated with the indexed layer.
    /// - `param_index` is the parameter index of the layer whose source you wish to interrogate
    pub fn layer_channel_count(&self, effect_ref: impl AsPtr<PF_ProgPtr>, param_index: i32) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, PF_GetLayerChannelCount -> ae_sys::A_long, effect_ref.as_ptr(), param_index)? as _)
    }

    /// Retrieves (by index) a reference to, and description of, the specified channel.
    /// Given a channel index return the opaque channelRef and a channel description.
    /// Channel index must lie between 0 and num_channels-1.
    /// You will use the channelRef in all subsequent calls
    pub fn layer_channel_indexed_ref_and_desc(&self, effect_ref: impl AsPtr<PF_ProgPtr>, param_index: i32, channel_index: i32) -> Result<Option<(PF_ChannelRef, PF_ChannelDesc)>, Error> {
        let mut found: PF_Boolean  = 0;
        // SAFETY: Zero-initializing PF_ChannelRef for FFI call.
        // Detailed explanation: (1) PF_ChannelRef is an opaque FFI type from the Adobe SDK that is valid when zero-initialized,
        // (2) the value will be fully initialized by the subsequent FFI call to PF_GetLayerChannelIndexedRefAndDesc before use,
        // (3) this is a standard pattern for out-parameters in the Adobe After Effects SDK.
        // Would be UB if: PF_ChannelRef required non-zero bit patterns for validity, but as an opaque handle type it does not.
        let mut channel_ref = unsafe { std::mem::zeroed() };
        // SAFETY: Zero-initializing PF_ChannelDesc for FFI call.
        // Detailed explanation: (1) PF_ChannelDesc is a C-compatible struct from the Adobe SDK,
        // (2) the struct contains only primitive types and pointers that are valid when zero-initialized,
        // (3) the value will be fully initialized by the subsequent FFI call before being returned to the caller.
        // Would be UB if: PF_ChannelDesc contained types with validity invariants that exclude all-zeros (e.g., NonNull, bool with invalid bit pattern), but it contains only C-compatible primitives.
        let mut channel_desc = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, PF_GetLayerChannelIndexedRefAndDesc, effect_ref.as_ptr(), param_index, channel_index, &mut found, &mut channel_ref, &mut channel_desc)?;
        if found == 1 {
            Ok(Some((channel_ref, channel_desc)))
        } else {
            Ok(None)
        }
    }

    /// Retrieves an auxiliary channel by type.
    pub fn layer_channel_typed_ref_and_desc(&self, effect_ref: impl AsPtr<PF_ProgPtr>, param_index: i32, channel_type: ChannelType) -> Result<Option<(PF_ChannelRef, PF_ChannelDesc)>, Error> {
        let mut found: PF_Boolean  = 0;
        // SAFETY: Zero-initializing PF_ChannelRef for FFI call.
        // Detailed explanation: (1) PF_ChannelRef is an opaque FFI type from the Adobe SDK that is valid when zero-initialized,
        // (2) the value will be fully initialized by the subsequent FFI call to PF_GetLayerChannelTypedRefAndDesc before use,
        // (3) this is a standard pattern for out-parameters in the Adobe After Effects SDK.
        // Would be UB if: PF_ChannelRef required non-zero bit patterns for validity, but as an opaque handle type it does not.
        let mut channel_ref = unsafe { std::mem::zeroed() };
        // SAFETY: Zero-initializing PF_ChannelDesc for FFI call.
        // Detailed explanation: (1) PF_ChannelDesc is a C-compatible struct from the Adobe SDK,
        // (2) the struct contains only primitive types and pointers that are valid when zero-initialized,
        // (3) the value will be fully initialized by the subsequent FFI call before being returned to the caller.
        // Would be UB if: PF_ChannelDesc contained types with validity invariants that exclude all-zeros (e.g., NonNull, bool with invalid bit pattern), but it contains only C-compatible primitives.
        let mut channel_desc = unsafe { std::mem::zeroed() };
        call_suite_fn!(self, PF_GetLayerChannelTypedRefAndDesc, effect_ref.as_ptr(), param_index, channel_type.into(), &mut found, &mut channel_ref, &mut channel_desc)?;
        if found == 1 {
            Ok(Some((channel_ref, channel_desc)))
        } else {
            Ok(None)
        }
    }

    /// Retrieves the ``PF_ChannelChunk`` containing the data associated with the given ``PF_ChannelRefPtr``.
    /// The data chunk is allocated is of the type requested.
    /// The data is in chunky format.
    pub fn checkout_layer_channel(&self, effect_ref: impl AsPtr<PF_ProgPtr>, channel_ref: &PF_ChannelRef, what_time: i32, duration: i32, time_scale: u32, data_type: DataType) -> Result<ChannelChunk, Error> {
        Ok(ChannelChunk(call_suite_fn_single!(self, PF_CheckoutLayerChannel -> PF_ChannelChunk, effect_ref.as_ptr(), channel_ref as *const _ as _, what_time, duration, time_scale, data_type.into())?))
    }

    /// The checked out channel must be checked in to avoid memory leaks.
    pub fn checkin_layer_channel(&self, effect_ref: impl AsPtr<PF_ProgPtr>, channel_ref: &PF_ChannelRef, channel_chunk: &ChannelChunk) -> Result<(), Error> {
        call_suite_fn!(self, PF_CheckinLayerChannel, effect_ref.as_ptr(), channel_ref as *const _ as _, channel_chunk as *const _ as _)
    }
}

define_enum! {
    ae_sys::PF_ChannelType,
    /// The kinds of multichannels we understand
    ChannelType {
        Depth           = PF_CHANNELTYPE_DEPTH,
        DepthAA         = PF_CHANNELTYPE_DEPTHAA,
        Normals         = PF_CHANNELTYPE_NORMALS,
        ObjectID        = PF_CHANNELTYPE_OBJECTID,
        MotionVector    = PF_CHANNELTYPE_MOTIONVECTOR,
        BackgroundColor = PF_CHANNELTYPE_BK_COLOR,
        Texture         = PF_CHANNELTYPE_TEXTURE,
        Coverage        = PF_CHANNELTYPE_COVERAGE,
        Node            = PF_CHANNELTYPE_NODE,
        Material        = PF_CHANNELTYPE_MATERIAL,
        Unclamped       = PF_CHANNELTYPE_UNCLAMPED,
        Unknown         = PF_CHANNELTYPE_UNKNOWN,
    }
}

define_enum! {
    ae_sys::PF_DataType,
    /// These are the elementary data types we understand.
    /// By convention we reserve the last characters of the type to designate the size in bytes of a plane of data.
    /// This together with the dimension tells us the size of each pixel.
    /// For example, data of [`ChannelType::BackgroundColor`] with [`DataType::Double`] would consist of 32 bytes per pixel.
    DataType {
        /// 4 byte
        Float         = PF_DATATYPE_FLOAT,
        /// 8 byte
        Double        = PF_DATATYPE_DOUBLE,
        /// 4 bytes
        Long          = PF_DATATYPE_LONG,
        /// 2 bytes
        Short         = PF_DATATYPE_SHORT,
        /// 4 bytes
        Fixed16_16    = PF_DATATYPE_FIXED_16_16,
        /// 1 byte
        Char          = PF_DATATYPE_CHAR,
        /// 1 byte
        UByte         = PF_DATATYPE_U_BYTE,
        /// 2 bytes
        UShort        = PF_DATATYPE_U_SHORT,
        /// 4 bytes
        UFixed16_16   = PF_DATATYPE_U_FIXED_16_16,
        /// 3 bytes
        Rgb           = PF_DATATYPE_RGB,
    }
}

const PF_CHANNELTYPE_DEPTH:        i32 = i32::from_be_bytes(*b"DPTH");
const PF_CHANNELTYPE_DEPTHAA:      i32 = i32::from_be_bytes(*b"DPAA"); // since 16.0 for 3D Precomp in some Artisans
const PF_CHANNELTYPE_NORMALS:      i32 = i32::from_be_bytes(*b"NRML");
const PF_CHANNELTYPE_OBJECTID:     i32 = i32::from_be_bytes(*b"OBID");
const PF_CHANNELTYPE_MOTIONVECTOR: i32 = i32::from_be_bytes(*b"MTVR");
const PF_CHANNELTYPE_BK_COLOR:     i32 = i32::from_be_bytes(*b"BKCR");
const PF_CHANNELTYPE_TEXTURE:      i32 = i32::from_be_bytes(*b"TEXR");
const PF_CHANNELTYPE_COVERAGE:     i32 = i32::from_be_bytes(*b"COVR");
const PF_CHANNELTYPE_NODE:         i32 = i32::from_be_bytes(*b"NODE");
const PF_CHANNELTYPE_MATERIAL:     i32 = i32::from_be_bytes(*b"MATR");
const PF_CHANNELTYPE_UNCLAMPED:    i32 = i32::from_be_bytes(*b"UNCP");
const PF_CHANNELTYPE_UNKNOWN:      i32 = i32::from_be_bytes(*b"UNKN");

const PF_DATATYPE_FLOAT:         i32 = i32::from_be_bytes(*b"FLT4"); // 4 byte
const PF_DATATYPE_DOUBLE:        i32 = i32::from_be_bytes(*b"DBL8"); // 8 byte
const PF_DATATYPE_LONG:          i32 = i32::from_be_bytes(*b"LON4"); // 4 bytes
const PF_DATATYPE_SHORT:         i32 = i32::from_be_bytes(*b"SHT2"); // 2 bytes
const PF_DATATYPE_FIXED_16_16:   i32 = i32::from_be_bytes(*b"FIX4"); // 4 bytes
const PF_DATATYPE_CHAR:          i32 = i32::from_be_bytes(*b"CHR1"); // 1 byte
const PF_DATATYPE_U_BYTE:        i32 = i32::from_be_bytes(*b"UBT1"); // 1 byte
const PF_DATATYPE_U_SHORT:       i32 = i32::from_be_bytes(*b"UST2"); // 2 bytes
const PF_DATATYPE_U_FIXED_16_16: i32 = i32::from_be_bytes(*b"UFX4"); // 4 bytes
const PF_DATATYPE_RGB:           i32 = i32::from_be_bytes(*b"RBG "); // 3 bytes

/// the channel data parallels the image data in size and shape.
/// the width is the number of pixels, the height is the number of scanlines
/// the height is image_height
/// the dimension is the number of planes in a pixel
/// the row_bytes is the length of a scanline in bytes
/// the data type is the type of data in a plane
/// Note : a pixel consists of dimensionL * sizeof(data_type) bytes
/// dataH is a handle to the data.
/// dataPV is a pointer to the dereferenced locked handle
/// effects should always have dataPV non null.
#[repr(transparent)]
pub struct ChannelChunk(ae_sys::PF_ChannelChunk);
impl std::ops::Deref for ChannelChunk {
    type Target = ae_sys::PF_ChannelChunk;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Copy)]
pub enum ChannelDataType {
    Float(*mut f32),
    Double(*mut f64),
    Long(*mut i32),
    Short(*mut i16),
    Fixed16_16(*mut i32),
    Char(*mut i8),
    UByte(*mut u8),
    UShort(*mut u16),
    UFixed16_16(*mut u32),
    Rgb(*mut u8),
}

impl ChannelChunk {
    pub fn channel_data(&self) -> ChannelDataType {
        match self.0.data_type {
            PF_DATATYPE_FLOAT         => ChannelDataType::Float(      self.0.dataPV as *mut _),
            PF_DATATYPE_DOUBLE        => ChannelDataType::Double(     self.0.dataPV as *mut _),
            PF_DATATYPE_LONG          => ChannelDataType::Long(       self.0.dataPV as *mut _),
            PF_DATATYPE_SHORT         => ChannelDataType::Short(      self.0.dataPV as *mut _),
            PF_DATATYPE_FIXED_16_16   => ChannelDataType::Fixed16_16( self.0.dataPV as *mut _),
            PF_DATATYPE_CHAR          => ChannelDataType::Char(       self.0.dataPV as *mut _),
            PF_DATATYPE_U_BYTE        => ChannelDataType::UByte(      self.0.dataPV as *mut _),
            PF_DATATYPE_U_SHORT       => ChannelDataType::UShort(     self.0.dataPV as *mut _),
            PF_DATATYPE_U_FIXED_16_16 => ChannelDataType::UFixed16_16(self.0.dataPV as *mut _),
            PF_DATATYPE_RGB           => ChannelDataType::Rgb(        self.0.dataPV as *mut _),
            _ => unreachable!(),
        }
    }
    pub fn channel_row_data(&self, row: i32) -> ChannelDataType {
        if row < 0 || row >= self.0.heightL {
            panic!("Invalid row: {row}, height: {}", self.0.heightL);
        }
        let offset = row as isize * self.0.row_bytesL as isize;
        match self.0.data_type {
            // SAFETY: Computing byte offset into channel data buffer for f32 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_FLOAT         => ChannelDataType::Float(      unsafe { (self.0.dataPV as *mut f32).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for f64 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_DOUBLE        => ChannelDataType::Double(     unsafe { (self.0.dataPV as *mut f64).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for i32 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_LONG          => ChannelDataType::Long(       unsafe { (self.0.dataPV as *mut i32).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for i16 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_SHORT         => ChannelDataType::Short(      unsafe { (self.0.dataPV as *mut i16).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for fixed-point i32 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_FIXED_16_16   => ChannelDataType::Fixed16_16( unsafe { (self.0.dataPV as *mut i32).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for i8 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_CHAR          => ChannelDataType::Char(       unsafe { (self.0.dataPV as *mut i8 ).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for u8 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_U_BYTE        => ChannelDataType::UByte(      unsafe { (self.0.dataPV as *mut u8 ).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for u16 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_U_SHORT       => ChannelDataType::UShort(     unsafe { (self.0.dataPV as *mut u16).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for fixed-point u32 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_U_FIXED_16_16 => ChannelDataType::UFixed16_16(unsafe { (self.0.dataPV as *mut u32).byte_offset(offset) }),
            // SAFETY: Computing byte offset into channel data buffer for RGB u8 type.
            // Detailed explanation: (1) row bounds are validated above (0 <= row < heightL),
            // (2) offset is computed using row_bytesL which defines the stride per scanline provided by Adobe SDK,
            // (3) dataPV is guaranteed valid by the SDK after successful checkout via PF_CheckoutLayerChannel.
            // Would be UB if: row validation was skipped, row_bytesL was incorrect, or dataPV was not properly initialized by the SDK.
            PF_DATATYPE_RGB           => ChannelDataType::Rgb(        unsafe { (self.0.dataPV as *mut u8 ).byte_offset(offset) }),
            _ => unreachable!(),
        }
    }

    pub fn channel_row_col_data(&self, row: i32, col: i32) -> ChannelDataType {
        if row < 0 || row >= self.0.heightL {
            panic!("Invalid row: {row}, height: {}", self.0.heightL);
        }
        if col < 0 || col >= self.0.widthL {
            panic!("Invalid col: {col}, width: {}", self.0.widthL);
        }
        let row_offset = row as isize * self.0.row_bytesL as isize;
        let col_offset = col as isize * self.0.dimensionL as isize;
        match self.0.data_type {
            // SAFETY: Computing row and column offset into channel data buffer for f32 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_FLOAT         => ChannelDataType::Float(      unsafe { (self.0.dataPV as *mut f32).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for f64 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_DOUBLE        => ChannelDataType::Double(     unsafe { (self.0.dataPV as *mut f64).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for i32 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_LONG          => ChannelDataType::Long(       unsafe { (self.0.dataPV as *mut i32).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for i16 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_SHORT         => ChannelDataType::Short(      unsafe { (self.0.dataPV as *mut i16).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for fixed-point i32 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_FIXED_16_16   => ChannelDataType::Fixed16_16( unsafe { (self.0.dataPV as *mut i32).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for i8 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_CHAR          => ChannelDataType::Char(       unsafe { (self.0.dataPV as *mut i8 ).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for u8 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_U_BYTE        => ChannelDataType::UByte(      unsafe { (self.0.dataPV as *mut u8 ).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for u16 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_U_SHORT       => ChannelDataType::UShort(     unsafe { (self.0.dataPV as *mut u16).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for fixed-point u32 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_U_FIXED_16_16 => ChannelDataType::UFixed16_16(unsafe { (self.0.dataPV as *mut u32).byte_offset(row_offset).offset(col_offset) }),
            // SAFETY: Computing row and column offset into channel data buffer for RGB u8 type.
            // Detailed explanation: (1) row bounds validated above (0 <= row < heightL) and col bounds (0 <= col < widthL),
            // (2) row_offset computed using row_bytesL stride per scanline, col_offset using dimensionL (planes per pixel),
            // (3) dataPV guaranteed valid by Adobe SDK after successful PF_CheckoutLayerChannel call.
            // Would be UB if: bounds validation was skipped, row_bytesL/dimensionL were incorrect, or the combined offset exceeded allocated buffer size.
            PF_DATATYPE_RGB           => ChannelDataType::Rgb(        unsafe { (self.0.dataPV as *mut u8 ).byte_offset(row_offset).offset(col_offset) }),
            _ => unreachable!(),
        }
    }
}
