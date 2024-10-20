#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

use premiere_sys as pr_sys;
use std::ptr;
use std::cell::RefCell;

#[macro_use]
mod macros;

mod types;
pub use types::*;

pub(crate) mod pf_suites {
    pub(crate) mod background_frame;
    pub(crate) mod cache_on_load;
    pub(crate) mod pixel_format;
    pub(crate) mod source_settings;
    pub(crate) mod transition;
    pub(crate) mod utility;
}
pub mod suites {
    pub(crate) mod gpu_device;               pub use gpu_device          ::GPUDeviceSuite          as GPUDevice;
    pub(crate) mod gpu_image_processing;     pub use gpu_image_processing::GPUImageProcessingSuite as GPUImageProcessing;
    pub(crate) mod memory_manager;           pub use memory_manager      ::MemoryManagerSuite      as MemoryManager;
    pub(crate) mod ppix;                     pub use ppix                ::{PPixSuite              as PPix,
                                                                            PPix2Suite             as PPix2 };
    pub(crate) mod time;                     pub use time                ::TimeSuite               as Time;
    pub(crate) mod sequence_info;            pub use sequence_info       ::SequenceInfoSuite       as SequenceInfo;
    pub(crate) mod video_segment;            pub use video_segment       ::VideoSegmentSuite       as VideoSegment;
    pub(crate) mod string;                   pub use string              ::PrStringSuite           as PrString;
    pub(crate) mod window;                   pub use window              ::WindowSuite             as Window;
    pub(crate) mod video_segment_properties;
    #[cfg(has_ae_sdk)] mod opaque_effect_data;
    #[cfg(has_ae_sdk)] pub use opaque_effect_data::OpaqueEffectDataSuite as OpaqueEffectData;

    pub use crate::pf_suites::background_frame::BackgroundFrameSuite as BackgroundFrame;
    pub use crate::pf_suites::cache_on_load   ::CacheOnLoadSuite     as CacheOnLoad;
    pub use crate::pf_suites::pixel_format    ::PixelFormatSuite     as PixelFormat;
    pub use crate::pf_suites::source_settings ::SourceSettingsSuite  as SourceSettings;
    pub use crate::pf_suites::transition      ::TransitionSuite      as Transition;
    pub use crate::pf_suites::utility         ::UtilitySuite         as Utility;
}
pub use suites::string::PrString;
pub use suites::video_segment_properties::*;
pub use suites::video_segment::VideoSegmentProperties;
pub use suites::ppix::YUV420PlanarBuffers;
pub use suites::sequence_info::ImmersiveVideoVRConfiguration;
pub use pf_suites::background_frame::TransferMode;
pub use pf_suites::pixel_format::NewWorldFlags;

pub use premiere_sys as sys;

mod gpu_filter;
pub use gpu_filter::*;

define_enum! {
    pr_sys::prSuiteError,
    Error {
        None                                            = pr_sys::suiteError_NoError,
        Fail                                            = pr_sys::suiteError_Fail,
        InvalidParms                                    = pr_sys::suiteError_InvalidParms,
        OutOfMemory                                     = pr_sys::suiteError_OutOfMemory,
        InvalidCall                                     = pr_sys::suiteError_InvalidCall,
        NotImplemented                                  = pr_sys::suiteError_NotImplemented,
        IDNotValid                                      = pr_sys::suiteError_IDNotValid,
        RenderPending                                   = pr_sys::suiteError_RenderPending,
        RenderedFrameNotFound                           = pr_sys::suiteError_RenderedFrameNotFound,
        RenderedFrameCanceled                           = pr_sys::suiteError_RenderedFrameCanceled,
        RenderInvalidPixelFormat                        = pr_sys::suiteError_RenderInvalidPixelFormat,
        RenderCompletionProcNotSet                      = pr_sys::suiteError_RenderCompletionProcNotSet,
        TimeRoundedAudioRate                            = pr_sys::suiteError_TimeRoundedAudioRate,
        CompilerCompileAbort                            = pr_sys::suiteError_CompilerCompileAbort,
        CompilerCompileDone                             = pr_sys::suiteError_CompilerCompileDone,
        CompilerOutputFormatAccept                      = pr_sys::suiteError_CompilerOutputFormatAccept,
        CompilerOutputFormatDecline                     = pr_sys::suiteError_CompilerOutputFormatDecline,
        CompilerRebuildCutList                          = pr_sys::suiteError_CompilerRebuildCutList,
        CompilerIterateCompiler                         = pr_sys::suiteError_CompilerIterateCompiler,
        CompilerIterateCompilerDone                     = pr_sys::suiteError_CompilerIterateCompilerDone,
        CompilerInternalErrorSilent                     = pr_sys::suiteError_CompilerInternalErrorSilent,
        CompilerIterateCompilerCacheable                = pr_sys::suiteError_CompilerIterateCompilerCacheable,
        CompilerBadFormatIndex                          = pr_sys::suiteError_CompilerBadFormatIndex,
        CompilerInternalError                           = pr_sys::suiteError_CompilerInternalError,
        CompilerOutOfDiskSpace                          = pr_sys::suiteError_CompilerOutOfDiskSpace,
        CompilerBufferFull                              = pr_sys::suiteError_CompilerBufferFull,
        CompilerErrOther                                = pr_sys::suiteError_CompilerErrOther,
        CompilerErrMemory                               = pr_sys::suiteError_CompilerErrMemory,
        CompilerErrFileNotFound                         = pr_sys::suiteError_CompilerErrFileNotFound,
        CompilerErrTooManyOpenFiles                     = pr_sys::suiteError_CompilerErrTooManyOpenFiles,
        CompilerErrPermErr                              = pr_sys::suiteError_CompilerErrPermErr,
        CompilerErrOpenErr                              = pr_sys::suiteError_CompilerErrOpenErr,
        CompilerErrInvalidDrive                         = pr_sys::suiteError_CompilerErrInvalidDrive,
        CompilerErrDupFile                              = pr_sys::suiteError_CompilerErrDupFile,
        CompilerErrIo                                   = pr_sys::suiteError_CompilerErrIo,
        CompilerErrInUse                                = pr_sys::suiteError_CompilerErrInUse,
        CompilerErrCodecBadInput                        = pr_sys::suiteError_CompilerErrCodecBadInput,
        ExporterSuspended                               = pr_sys::suiteError_ExporterSuspended,
        ExporterNoMoreFrames                            = pr_sys::suiteError_ExporterNoMoreFrames,
        FileBufferTooSmall                              = pr_sys::suiteError_FileBufferTooSmall,
        FileNotImportableFileType                       = pr_sys::suiteError_FileNotImportableFileType,
        LegacyInvalidVideoRate                          = pr_sys::suiteError_LegacyInvalidVideoRate,
        PlayModuleAudioInitFailure                      = pr_sys::suiteError_PlayModuleAudioInitFailure,
        PlayModuleAudioIllegalPlaySetting               = pr_sys::suiteError_PlayModuleAudioIllegalPlaySetting,
        PlayModuleAudioNotInitialized                   = pr_sys::suiteError_PlayModuleAudioNotInitialized,
        PlayModuleAudioNotStarted                       = pr_sys::suiteError_PlayModuleAudioNotStarted,
        PlayModuleAudioIllegalAction                    = pr_sys::suiteError_PlayModuleAudioIllegalAction,
        PlayModuleDeviceControlSuiteIllegalCallSequence = pr_sys::suiteError_PlayModuleDeviceControlSuiteIllegalCallSequence,
        MediaAcceleratorSuitePathNotFound               = pr_sys::suiteError_MediaAcceleratorSuitePathNotFound,
        MediaAcceleratorSuiteRegisterFailure            = pr_sys::suiteError_MediaAcceleratorSuiteRegisterFailure,
        RepositoryReadFailed                            = pr_sys::suiteError_RepositoryReadFailed,
        RepositoryWriteFailed                           = pr_sys::suiteError_RepositoryWriteFailed,
        NotActivated                                    = pr_sys::suiteError_NotActivated,
        DataNotPresent                                  = pr_sys::suiteError_DataNotPresent,
        ServerCommunicationFailed                       = pr_sys::suiteError_ServerCommunicationFailed,
        Internal                                        = pr_sys::suiteError_Internal,
        StringNotFound                                  = pr_sys::suiteError_StringNotFound,
        StringBufferTooSmall                            = pr_sys::suiteError_StringBufferTooSmall,
        NoKeyframeAfterInTime                           = pr_sys::suiteError_NoKeyframeAfterInTime,
        NoMoreData                                      = pr_sys::suiteError_NoMoreData,
        InstanceDestroyed                               = pr_sys::suiteError_InstanceDestroyed,
    }
}

impl From<Error> for &'static str {
    fn from(error: Error) -> &'static str {
        match error {
            Error::None                                 => "No error",
            Error::Fail                                 => "Method failed",
            Error::InvalidParms                         => "A parameter to this method is invalid",
            Error::OutOfMemory                          => "There is not enough memory to complete this method",
            Error::InvalidCall                          => "Usually this means this method call is not appropriate at this time",
            Error::NotImplemented                       => "The requested action is not implemented",
            Error::IDNotValid                           => "The passed in ID (pluginID, clipID...) is not valid",
            Error::RenderPending                        => "Render is pending",
            Error::RenderedFrameNotFound                => "A cached frame was not found.",
            Error::RenderedFrameCanceled                => "A render was canceled",
            Error::RenderInvalidPixelFormat             => "Render output pixel format list is invalid",
            Error::RenderCompletionProcNotSet           => "The render completion proc was not set for an async request",
            Error::TimeRoundedAudioRate                 => "Audio rate returned was rounded",
            Error::CompilerCompileAbort                 => "User aborted the compile",
            Error::CompilerCompileDone                  => "Compile finished normally",
            Error::CompilerOutputFormatAccept           => "The output format is valid",
            Error::CompilerOutputFormatDecline          => "The compile module cannot compile to the output format",
            Error::CompilerRebuildCutList               => "Return value from compGetFilePrefs used to force Premiere to bebuild its cutlist",
            Error::CompilerIterateCompiler              => "6.0 Return value from compInit to request compiler iteration",
            Error::CompilerIterateCompilerDone          => "6.0 Return value from compInit to indicate there are no more compilers",
            Error::CompilerInternalErrorSilent          => "6.0 Silent error code; Premiere will not display an error message on screen. \nCompilers can return this error code from compDoCompile if they wish to put their own customized error message on screen just before returning control to Premiere",
            Error::CompilerIterateCompilerCacheable     => "7.0 Return value from compInit to request compiler iteration and indicating that this compiler is cacheable.",
            Error::CompilerBadFormatIndex               => "Invalid format index - used to stop compGetIndFormat queries",
            Error::CompilerInternalError                => "Compiler interna error",
            Error::CompilerOutOfDiskSpace               => "Out of disk space error",
            Error::CompilerBufferFull                   => "The offset into the audio buffer would overflow it",
            Error::CompilerErrOther                     => "Someone set gCompileErr",
            Error::CompilerErrMemory                    => "Ran out of memory",
            Error::CompilerErrFileNotFound              => "File not found",
            Error::CompilerErrTooManyOpenFiles          => "Too many open files",
            Error::CompilerErrPermErr                   => "Permission violation",
            Error::CompilerErrOpenErr                   => "Unable to open the file",
            Error::CompilerErrInvalidDrive              => "Drive isn't valid.",
            Error::CompilerErrDupFile                   => "Duplicate Filename",
            Error::CompilerErrIo                        => "File io error",
            Error::CompilerErrInUse                     => "File is in use",
            Error::CompilerErrCodecBadInput             => "A video codec refused the input format",
            Error::ExporterSuspended                    => "The host has suspended the export",
            Error::ExporterNoMoreFrames                 => "Halt export early skipping all remaining frames including this one. AE uses",
            Error::FileBufferTooSmall                   => "File buffer is too small",
            Error::FileNotImportableFileType            => "Not an importable file type",
            Error::LegacyInvalidVideoRate               => "Invalid video rate (scale and sample rate don't match a valid rate)",
            Error::PlayModuleAudioInitFailure           => "PlayModuleAudio - init failure",
            Error::PlayModuleAudioIllegalPlaySetting    => "PlayModuleAudio - illegal play setting",
            Error::PlayModuleAudioNotInitialized        => "PlayModuleAudio - not initialized",
            Error::PlayModuleAudioNotStarted            => "PlayModuleAudio - not started",
            Error::PlayModuleAudioIllegalAction         => "PlayModuleAudio - illegal action",
            Error::PlayModuleDeviceControlSuiteIllegalCallSequence => "PlayModuleDeviceControlSuite - illegal call sequence",
            Error::MediaAcceleratorSuitePathNotFound    => "MediaAcceleratorSuite - path notFound",
            Error::MediaAcceleratorSuiteRegisterFailure => "MediaAcceleratorSuite - register failure",
            Error::RepositoryReadFailed                 => "Repository read failed",
            Error::RepositoryWriteFailed                => "Repository write failed",
            Error::NotActivated                         => "Not activated",
            Error::DataNotPresent                       => "Data not present",
            Error::ServerCommunicationFailed            => "Server communication failed",
            Error::Internal                             => "Internal error",
            Error::StringNotFound                       => "String not found",
            Error::StringBufferTooSmall                 => "String buffer is too small",
            Error::NoKeyframeAfterInTime                => "No keyframe after InTime",
            Error::NoMoreData                           => "No more data",
            Error::InstanceDestroyed                    => "Instance destroyed",
        }
    }
}

thread_local! {
    pub(crate) static PICA_BASIC_SUITE: RefCell<*const pr_sys::SPBasicSuite> = RefCell::new(ptr::null_mut());
}

#[inline]
pub(crate) fn borrow_pica_basic_as_ptr() -> *const pr_sys::SPBasicSuite {
    let mut pica_basic_ptr: *const pr_sys::SPBasicSuite = ptr::null();

    PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
        pica_basic_ptr = *pica_basic_ptr_cell.borrow();
    });

    pica_basic_ptr
}

/// This lets us access a thread-local version of the `PicaBasic`
/// suite. Whenever we generate a new `SPBasic_Suite` from Ae somehow,
/// we create a PicaBasicSuite::new() from that and use that to initialize
/// access to any suites.
///
/// When we leave scope, `drop()` ic alled automatically and restores the
/// previous value to our thread-local storage so the caller
/// can continue using their pointer to the suite.
///
/// FIXME: Is this really neccessary? Check if the pointer is always the
///        same and if so, confirm with Adobe we can get rid of it.
pub struct PicaBasicSuite {
    previous_pica_basic_suite_ptr: *const pr_sys::SPBasicSuite,
}

impl PicaBasicSuite {
    fn set(pica_basic_suite_ptr: *const pr_sys::SPBasicSuite) -> *const pr_sys::SPBasicSuite {
        let mut previous_pica_basic_suite_ptr: *const pr_sys::SPBasicSuite = ptr::null();

        PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
            previous_pica_basic_suite_ptr = pica_basic_ptr_cell.replace(pica_basic_suite_ptr);
        });

        previous_pica_basic_suite_ptr
    }

    #[inline]
    pub fn from_sp_basic_suite_raw(pica_basic_suite_ptr: *const pr_sys::SPBasicSuite) -> Self {
        Self {
            previous_pica_basic_suite_ptr: PicaBasicSuite::set(pica_basic_suite_ptr),
        }
    }
}

impl Drop for PicaBasicSuite {
    #[inline]
    fn drop(&mut self) {
        PICA_BASIC_SUITE.with(|pica_basic_ptr_cell| {
            pica_basic_ptr_cell.replace(self.previous_pica_basic_suite_ptr);
        });
    }
}

pub(crate) trait Suite {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

}

pub trait AsPtr<T> {
    fn as_ptr(&self) -> T
    where
        T: Sized;
}

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> T
    where T: Sized;
}
