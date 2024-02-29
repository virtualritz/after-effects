use crate::ae_sys;
use crate::AsPtr;

define_handle_wrapper!(InSpecHandle, AEIO_InSpecH);
define_handle_wrapper!(Handle, AEIO_Handle);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FileType {
    Dir     = -2,
    None    = -1,
    Any     = 0,
    Generic = 1,
}

define_enum! {
    ae_sys::AEIO_SndEncoding,
    SoundEncoding {
        UnsignedPcm = ae_sys::PF_UNSIGNED_PCM,
        SignedPcm   = ae_sys::PF_SIGNED_PCM,
        SignedFloat = ae_sys::PF_SIGNED_FLOAT,
    }
}

define_enum! {
    ae_sys::AEIO_SndSampleSize,
    SoundSampleSize {
        Size1 = ae_sys::PF_SSS_1,
        Size2 = ae_sys::PF_SSS_2,
        Size4 = ae_sys::PF_SSS_4,
    }
}

define_enum! {
    ae_sys::AEIO_SndChannels,
    SoundChannels {
        Mono   = ae_sys::PF_Channels_MONO,
        Stereo = ae_sys::PF_Channels_STEREO,
    }
}
